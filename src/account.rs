use std::mem;

use bcrypt::{
  hash,
  verify,
};
use bevy::{
  ecs::system::Command,
  prelude::*,
};

use crate::{
  character::{
    Player,
    Puppet,
  },
  db::Db,
  net::*,
  savestate::{
    DbEntity,
    Load,
  },
  tasks::EntityCommandsExt as _,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Session {
  pub username: String,
  pub id: i64,
  pub admin: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum LoginState {
  Start,
  Username,
  Password { name: String, user_id: i64 },
  NewUserPassword { name: String },
  NewUserConfirm { name: String, password: String },
}

impl Default for LoginState {
  fn default() -> Self {
    Self::Start
  }
}

pub struct StartLogin(pub Entity);

impl Command for StartLogin {
  fn apply(self, world: &mut World) {
    world.entity_mut(self.0).insert(LoginState::Start);
  }
}

pub struct AccountPlugin;

impl Plugin for AccountPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Session>();
    app.add_systems(
      Update,
      login_system.run_if(any_with_component::<LoginState>()),
    );
    app.add_systems(Update, disconnect_system);
  }
}

fn login_system(
  mut cmd: Commands,
  db: Res<Db>,
  mut query: Query<(Entity, &mut LoginState, &mut TelnetIn, &TelnetOut)>,
) {
  for (entity, mut state, mut input, output) in query.iter_mut() {
    match &mut *state {
      LoginState::Start => {
        *state = LoginState::Username;
        output.string("Account name: ");
        command!(output, GA);
      }
      LoginState::Username => {
        let name = try_opt!(input.next_line(), continue);
        let name = name.trim().to_string();
        if !validate_name(&name) {
          output.line("invalid user name");
          *state = LoginState::Start;
          continue;
        }

        let db = db.clone();

        cmd.entity(entity).remove::<LoginState>().spawn_callback(
          async move {
            let results = sqlx::query!("SELECT id from user where name = ?", name)
              .fetch_optional(&*db)
              .await?
              .map(|row| row.id);
            Ok((name, results))
          },
          move |(name, id), entity, world| {
            let output = world.entity(entity).get::<TelnetOut>().unwrap().clone();
            match id {
              None => {
                output.line("Creating new account.");
                world
                  .entity_mut(entity)
                  .insert(LoginState::NewUserPassword { name });
              }
              Some(user_id) => {
                world
                  .entity_mut(entity)
                  .insert(LoginState::Password { name, user_id });
              }
            };
            output.string("Password: ");
            command!(output, GA);
            negotiate!(output, WILL, ECHO);
          },
          move |err, entity, world| {
            world
              .entity(entity)
              .get::<TelnetOut>()
              .unwrap()
              .line(format!("error checking username: {}", err));
            world.entity_mut(entity).insert(LoginState::Start);
          },
        );
      }
      LoginState::Password { name, user_id } => {
        let password = try_opt!(input.next_line(), continue);
        negotiate!(output, WONT, ECHO);
        let db = db.clone();
        let id = *user_id;
        let username = mem::take(name);

        cmd.entity(entity).remove::<LoginState>().spawn_callback(
          async move {
            let row = sqlx::query!("SELECT password FROM user WHERE id = ?", id)
              .fetch_one(&*db)
              .await?;
            let hashed = row.password;
            Ok(verify(password, &hashed)?)
          },
          move |success, entity, world| {
            let output = world.entity(entity).get::<TelnetOut>().unwrap().clone();
            if success {
              output.line("\nSuccess!").string("> ");
              command!(output, GA);
              let db = world.resource::<Db>().clone();
              world
                .entity_mut(entity)
                .remove::<LoginState>()
                .insert(Session {
                  username,
                  id,
                  admin: true,
                })
                .spawn_callback(
                  async move {
                    let mut conn = db.acquire().await?;
                    let row = sqlx::query!("select entity from character where user_id = ?", id)
                      .fetch_one(&mut *conn)
                      .await?;
                    Ok(DbEntity::from_bits(row.entity as _))
                  },
                  move |res, entity, world| {
                    let character = world.spawn((res, Player(entity), Load)).id();
                    world.entity_mut(entity).insert(Puppet(character));
                  },
                  move |error, entity, _world| {
                    warn!(?entity, %error, "failed to find character");
                  },
                );
            } else {
              output.line("\nInvalid password.");
              world.entity_mut(entity).insert(LoginState::Start);
            }
          },
          move |err, entity, world| {
            world
              .entity(entity)
              .get::<TelnetOut>()
              .unwrap()
              .line(format!("\nerror checking password:\n{}", err));
            world.entity_mut(entity).insert(LoginState::Start);
          },
        );
      }

      LoginState::NewUserPassword { name } => {
        let password = try_opt!(input.next_line(), continue);
        output.string("\nConfirm password: ");
        command!(output, GA);
        *state = LoginState::NewUserConfirm {
          name: mem::take(name),
          password,
        };
      }
      LoginState::NewUserConfirm { name, password } => {
        let confirm = try_opt!(input.next_line(), continue);
        if *password != confirm {
          output.line("\nPasswords do not match.");
          negotiate!(output, WONT, ECHO);
          *state = LoginState::Start;
          continue;
        }

        output.string("\nCreating account...");
        command!(output, GA);
        negotiate!(output, WONT, ECHO);

        let db = db.clone();
        let name = mem::take(name);

        cmd.entity(entity).remove::<LoginState>().spawn_callback(
          async move {
            let hashed = hash(confirm, 4)?;

            let res = sqlx::query!(
              "INSERT INTO user (name, password) VALUES (?, ?)",
              name,
              hashed
            )
            .execute(&*db)
            .await?;
            Ok((name, res.last_insert_rowid()))
          },
          move |(name, id), entity, world| {
            if let Some(out) = world.entity(entity).get::<TelnetOut>() {
              out.line(" done!");
            }
            world
              .entity_mut(entity)
              .remove::<LoginState>()
              .insert(Session {
                username: name,
                id,
                admin: true,
              });
          },
          move |err, entity, world| {
            if let Some(out) = world.entity(entity).get::<TelnetOut>() {
              out.line(format!("\nerror creating user:\n{}", err));
            }
            world.entity_mut(entity).insert(LoginState::Start);
          },
        );
      }
    }
  }
}

fn validate_name(_name: &str) -> bool {
  true
}
