use std::mem;

use bcrypt::{
  hash,
  verify,
};
use bevy::{
  ecs::{
    entity::MapEntities,
    world::Command,
  },
  prelude::*,
  utils::HashMap,
};
use bevy_replicon::core::Replicated;
use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  character::{
    CharacterBundle,
    NewCharacterBundle,
    Player,
    Puppet,
  },
  core::Live,
  net::*,
};

#[derive(Debug, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct UserEntry {
  pub hashed_password: String,
  pub character: Entity,
}

impl MapEntities for UserEntry {
  fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
    self.character = entity_mapper.map_entity(self.character);
  }
}

#[derive(Debug, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, FromWorld, Serialize, Deserialize)]
pub struct UserDb {
  pub users: HashMap<String, UserEntry>,
}

impl MapEntities for UserDb {
  fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
    self
      .users
      .values_mut()
      .for_each(|v| v.map_entities(entity_mapper));
  }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Session {
  pub username: String,
  pub admin: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum LoginState {
  Start,
  Username,
  Password { name: String },
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
    app
      .register_type::<Session>()
      .register_type::<UserDb>()
      .register_type::<UserEntry>()
      .insert_resource(UserDb::default())
      .add_systems(
        Update,
        login_system.run_if(any_with_component::<LoginState>),
      );
  }
}

fn login_system(
  mut cmd: Commands,
  mut users: ResMut<UserDb>,
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
        if users.users.contains_key(&name) {
          cmd.entity(entity).insert(LoginState::Password { name });
          output.line("Password: ");
        } else {
          output.line("Creating new account.");
          cmd
            .entity(entity)
            .insert(LoginState::NewUserPassword { name });

          output.string("Password: ");
          command!(output, GA);
          negotiate!(output, WILL, ECHO);
        }
      }
      LoginState::Password { name } => {
        let password = try_opt!(input.next_line(), continue);
        negotiate!(output, WONT, ECHO);
        let username = mem::take(name);

        let Some(entry) = users.users.get(&username) else {
          warn!(username, "user not found");
          cmd.entity(entity).insert(LoginState::Start);
          continue;
        };

        debug!(?password, hash = ?entry.hashed_password, "verifying password");
        if !verify(password, &entry.hashed_password).unwrap() {
          output.line("Invalid password.");
          cmd.entity(entity).insert(LoginState::Start);
          continue;
        }

        output.line("Welcome back!");
        command!(output, GA);

        cmd
          .entity(entry.character)
          .insert((Live, CharacterBundle::default(), Player(entity)));
        cmd
          .entity(entity)
          .insert((
            Session {
              username,
              admin: true,
            },
            Puppet(entry.character),
          ))
          .remove::<LoginState>();
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

        debug!(password = ?confirm, "hashing password");
        let hashed_password = hash(confirm, 4).unwrap();
        output.line(" done!");
        let character = cmd
          .spawn((NewCharacterBundle::default(), Replicated, Player(entity)))
          .id();
        cmd.entity(entity).remove::<LoginState>().insert((
          Session {
            username: name.clone(),
            admin: true,
          },
          Puppet(character),
        ));

        users.users.insert(
          name.clone(),
          UserEntry {
            hashed_password,
            character,
          },
        );
      }
    }
  }
}

fn validate_name(_name: &str) -> bool {
  true
}
