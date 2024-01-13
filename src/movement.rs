use std::{
  any::Any,
  cmp::Ordering,
  ops::{
    Add,
    AddAssign,
    Mul,
    MulAssign,
  },
};

use bevy::{
  ecs::system::EntityCommands,
  prelude::*,
};

use crate::{
  action::{
    Action,
    Busy,
    Queue,
    StopEvent,
  },
  coords::{
    Cubic,
    DIRECTIONS,
  },
  map::{
    GlobalTransform,
    Transform,
  },
  output::PlayerOutput,
  savestate::SaveExt,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<MoveEvent>();

    app.persist_component::<Speed>();

    app.register_type::<MoveDebt>().register_type::<Moving>();

    app.add_systems(FixedUpdate, movement_system);
    app.add_systems(FixedUpdate, moving_output.before(movement_system));
    app.add_systems(Update, stop_moving.after(movement_system));
  }
}

/// The rate at which [MoveDebt] is paid off.
/// Each [FixedUpdate], the speed values are subtracted from the debt values,
/// stopping at movement = 0 and rotation = -1.
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Speed {
  pub movement: f32,
  pub rotation: f32,
}

impl Default for Speed {
  fn default() -> Self {
    Speed {
      movement: 1f32,
      rotation: 1f32,
    }
  }
}

/// Movement debt that an entity as accrued. This must be paid off before
/// movement occurs.
/// `movement` is clamped to >=0, and `rotation` to >=-1. This allows characters
/// to turn once "for free," but quick turns take time.
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct MoveDebt {
  pub movement: f32,
  pub rotation: f32,
}

impl Default for MoveDebt {
  fn default() -> Self {
    MoveDebt {
      movement: 0f32,
      rotation: -1f32,
    }
  }
}

impl Mul for MoveDebt {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    MoveDebt {
      movement: self.movement * rhs.movement,
      rotation: self.rotation * rhs.rotation,
    }
  }
}

impl Mul<f32> for MoveDebt {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self::Output {
    MoveDebt {
      movement: self.movement * rhs,
      rotation: self.rotation * rhs,
    }
  }
}

impl MulAssign<f32> for MoveDebt {
  fn mul_assign(&mut self, rhs: f32) {
    *self = *self * rhs;
  }
}

impl Add for MoveDebt {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    MoveDebt {
      movement: self.movement + rhs.movement,
      rotation: self.rotation + rhs.rotation,
    }
  }
}

impl AddAssign for MoveDebt {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

#[derive(Component, Debug, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub struct Moving;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum MoveAction {
  MoveAbsolute(Cubic),
  MoveRelative(Cubic),
  Turn(i8),
}

impl MoveAction {
  fn to_absolute(self, facing: i8) -> Self {
    match self {
      Self::MoveRelative(coords) => Self::MoveAbsolute(coords.rotate(facing)),
      other => other,
    }
  }
  fn to_relative(self, facing: i8) -> Self {
    match self {
      Self::MoveAbsolute(coords) => Self::MoveRelative(coords.rotate(-(facing))),
      other => other,
    }
  }
  fn debt(&self) -> MoveDebt {
    match self {
      Self::MoveAbsolute(off) | Self::MoveRelative(off) => MoveDebt {
        movement: Cubic::default().distance(*off) as f32,
        rotation: 0f32,
      },
      Self::Turn(rot) => MoveDebt {
        movement: 0f32,
        rotation: rot.abs() as f32,
      },
    }
  }
}

impl Default for MoveAction {
  fn default() -> Self {
    MoveAction::MoveRelative(DIRECTIONS[2])
  }
}

impl Action for MoveAction {
  fn busies(&self) -> bool {
    true
  }
  fn waits(&self) -> bool {
    true
  }
  fn execute(&self, cmd: &mut EntityCommands) {
    let action = *self;
    cmd.add(move |mut entity: EntityWorldMut<'_>| {
      // TODO: check map for movement costs
      let debt_inc = action.debt();

      if let Some(mut debt) = entity.get_mut::<MoveDebt>() {
        *debt += debt_inc;
      } else {
        entity.insert(MoveDebt::default() + debt_inc);
      }

      debug!(entity = ?entity.id(), ?action, "adding move action");
      entity.insert(action);
      let id = entity.id();
      entity.world_scope(|world| {
        world.send_event(MoveEvent {
          entity: id,
          typ: MoveState::Start,
          action,
        });
      });

      if !entity.contains::<Moving>() {
        entity.insert(Moving);
      }
    });
  }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveState {
  Start,
  Finish,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct MoveEvent {
  pub entity: Entity,
  pub typ: MoveState,
  pub action: MoveAction,
}

fn movement_system(
  cmd: ParallelCommands,
  mut query: Query<(
    Entity,
    &mut MoveDebt,
    &Speed,
    Option<&MoveAction>,
    Option<&mut Transform>,
  )>,
) {
  query
    .par_iter_mut()
    .for_each(|(ent, mut debt, speed, action, xform)| {
      if debt.movement > 0f32 {
        debt.movement = 0f32.max(debt.movement - (speed.movement / 60f32));
      }
      if debt.rotation > -1f32 {
        debt.rotation = (-1f32).max(debt.rotation - (speed.rotation / 60f32));
      }

      let mut xform = try_opt!(xform, return);

      let action = *try_opt!(action, return);
      let moved = match action {
        MoveAction::MoveAbsolute(off) if debt.movement == 0f32 => {
          xform.coords += off;
          true
        }
        MoveAction::MoveRelative(off) if debt.movement == 0f32 => {
          let facing = xform.facing;
          xform.coords += off.rotate(facing);
          true
        }
        MoveAction::Turn(dir) if debt.rotation <= 0f32 => {
          xform.facing += dir;
          xform.facing %= 6;
          true
        }
        _ => false,
      };
      if moved {
        cmd.command_scope(|mut cmd| {
          debug!("completed movement, un-busying");
          let mut entity = cmd.entity(ent);
          entity.add(move |mut entity: EntityWorldMut<'_>| {
            let moving = entity
              .get::<Queue>()
              .and_then(|q| {
                debug!("checking queue for movement action");
                q.front()
                  .and_then(|act| (&**act as &dyn Any).downcast_ref::<MoveAction>())
              })
              .is_some();
            if !moving {
              debug!("not moving, removing tag");
              entity.remove::<Moving>();
            }
            entity.remove::<(MoveAction, Busy)>();
            entity.world_scope(|world| {
              world.send_event(MoveEvent {
                entity: ent,
                typ: MoveState::Finish,
                action,
              });
            });
          });
        });
      }
    })
}

const HUMAN_DIRECTIONS: [&str; 6] = [
  "backward and to your right",
  "forward and to your right",
  "forward",
  "forward and to your left",
  "backward and to your left",
  "backward",
];

fn moving_output(
  output: PlayerOutput,
  mut startstop_events: EventReader<MoveEvent>,
  locations: Query<&GlobalTransform>,
) {
  for ev in startstop_events.read() {
    let out = try_opt!(output.get(ev.entity), continue);
    let xform = try_opt!(locations.get(ev.entity).ok(), continue);

    let start = match ev.typ {
      MoveState::Start => "You start to",
      MoveState::Finish => "You",
    };

    let (action, direction) = match ev.action.to_relative(xform.facing) {
      MoveAction::Turn(dir) => (
        "turn",
        match 0.cmp(&dir) {
          Ordering::Less => "to your right",
          Ordering::Greater => "to your left",
          _ => "... nowhere? That seems wrong",
        },
      ),
      MoveAction::MoveRelative(coords) => (
        "move",
        HUMAN_DIRECTIONS
          .iter()
          .enumerate()
          .find_map(|(i, msg)| {
            if coords == DIRECTIONS[i] {
              Some(*msg)
            } else {
              None
            }
          })
          .unwrap_or("... nowhere? That seems wrong"),
      ),
      _ => unreachable!(),
    };
    out.line(format!("{} {} {}.", start, action, direction));
  }
}

fn stop_moving(mut events: EventReader<StopEvent>, mut cmd: Commands) {
  for event in events.read() {
    cmd.entity(**event).remove::<(Busy, Moving, MoveAction)>();
  }
}
