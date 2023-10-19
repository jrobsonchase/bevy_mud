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
  db::Db,
  net::*,
  savestate::{
    DbEntity,
    Load,
  },
  tasks::{
    check_task,
    Task,
  },
  TokioRuntime,
};

#[derive(Component)]
pub struct Account {
  pub username: String,
  pub id: i64,
}

#[derive(Component)]
enum LoginState {
  Start,
  Username,
  CheckUsername {
    name: String,
    task: Task<Option<i64>>,
  },
  Password {
    name: String,
    user_id: i64,
  },
  CheckPassword {
    name: String,
    user_id: i64,
    task: Task<bool>,
  },
  NewUserPassword {
    name: String,
  },
  NewUserConfirm {
    name: String,
    password: String,
  },
  CreateUser {
    name: String,
    task: Task<i64>,
  },
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
    app.add_systems(Update, login_system);
  }
}

fn login_system(
  mut cmd: Commands,
  rt: Res<TokioRuntime>,
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
      LoginState::CheckUsername { task, name } => {
        let res = try_opt!(check_task(task), continue);
        let id = try_res!(res, err => {
            output.line(format!("error checking username: {}", err));
            *state = LoginState::Start;
            continue;
        });
        let name = mem::take(name);
        match id {
          None => {
            output.line("Creating new account.");
            *state = LoginState::NewUserPassword { name };
          }
          Some(user_id) => {
            *state = LoginState::Password { name, user_id };
          }
        };
        output.string("Password: ");
        command!(output, GA);
        negotiate!(output, WILL, ECHO);
      }
      LoginState::CheckPassword {
        name,
        user_id,
        task,
      } => {
        let res = try_opt!(check_task(task), continue);
        let success = try_res!(res, e => {
                output.line(format!("\nerror checking password:\n{}", e));
                *state = LoginState::Start;
                continue;
        });
        if success {
          output.line("\nSuccess!");
          cmd.entity(entity).remove::<LoginState>().insert(Account {
            username: mem::take(name),
            id: *user_id,
          });
          cmd.spawn((Load, DbEntity(Entity::from_bits(17))));
        } else {
          output.line("\nInvalid password.");
          *state = LoginState::Start;
        }
      }
      LoginState::CreateUser { name, task } => match try_opt!(check_task(task), continue) {
        Ok(id) => {
          output.line(" done!");
          cmd.entity(entity).remove::<LoginState>().insert(Account {
            username: mem::take(name),
            id,
          });
        }
        Err(e) => {
          output.line(format!("\nerror creating user:\n{}", e));
          *state = LoginState::Start;
        }
      },

      LoginState::Username => {
        let name = try_opt!(input.next_line(), continue);
        let name = name.trim().to_string();
        if !validate_name(&name) {
          output.line("invalid user name");
          *state = LoginState::Start;
          continue;
        }

        let db = db.clone();
        *state = LoginState::CheckUsername {
          name: name.clone(),
          task: rt.spawn(async move {
            let results = sqlx::query!("SELECT id from user where name = ?", name)
              .fetch_optional(&db)
              .await?
              .map(|row| row.id);
            Ok(results)
          }),
        };
      }
      LoginState::Password { name, user_id } => {
        let password = try_opt!(input.next_line(), continue);
        negotiate!(output, WONT, ECHO);
        let db = db.clone();
        let user_id = *user_id;
        *state = LoginState::CheckPassword {
          name: mem::take(name),
          user_id,
          task: rt.spawn(async move {
            let row = sqlx::query!("SELECT password FROM user WHERE id = ?", user_id)
              .fetch_one(&db)
              .await?;
            let hashed = row.password;
            Ok(verify(password, &hashed)?)
          }),
        };
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

        *state = LoginState::CreateUser {
          name: name.clone(),
          task: rt.spawn(async move {
            let hashed = hash(confirm, 4)?;

            let res = sqlx::query!(
              "INSERT INTO user (name, password) VALUES (?, ?)",
              name,
              hashed
            )
            .execute(&db)
            .await?;
            Ok(res.last_insert_rowid())
          }),
        };
      }
    }
  }
}

fn validate_name(_name: &str) -> bool {
  true
}
