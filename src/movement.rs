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
use hexx::{
  hex,
  EdgeDirection,
  Hex,
};
use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  action::{
    Action,
    Busy,
    Queue,
    StopEvent,
  },
  map::{
    GlobalTransform,
    Transform,
  },
  output::PlayerOutput,
  savestate::traits::AppWorldExt,
  util::debug_trigger,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<MoveEvent>();

    app.persist::<Speed>();
    app.register_type::<Speed>();

    app.register_type::<MoveDebt>().register_type::<Moving>();

    app.add_systems(FixedUpdate, (movedebt_system, movement_system).chain());
    app.observe(debug_trigger::<MoveEvent>);
    app.observe(debug_trigger::<StopEvent>);
    app.observe(moving_output);
    app.observe(stop_moving);
  }
}

/// The rate at which [MoveDebt] is paid off.
/// Each [FixedUpdate], the speed values are subtracted from the debt values,
/// stopping at movement = 0 and rotation = -1.
#[derive(Component, Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
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
  MoveAbsolute(Hex),
  MoveRelative(Hex),
  Turn(i8),
}

impl MoveAction {
  fn to_relative(self, facing: EdgeDirection) -> Self {
    match self {
      Self::MoveAbsolute(coords) => {
        Self::MoveRelative(coords.rotate_cw(facing.const_neg().index() as _))
      }
      other => other,
    }
  }
  fn debt(&self) -> MoveDebt {
    match self {
      Self::MoveAbsolute(off) | Self::MoveRelative(off) => MoveDebt {
        movement: Hex::default().distance_to(*off) as f32,
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
    MoveAction::MoveRelative(hex(0, 0) + hexx::EdgeDirection::FLAT_NORTH)
  }
}

impl Action for MoveAction {
  fn busies(&self) -> bool {
    true
  }
  fn waits(&self) -> bool {
    true
  }
  fn execute(&self, cmd: EntityCommands) {
    let action = *self;
    cmd.queue(move |mut entity: EntityWorldMut<'_>| {
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
        world.trigger_targets(
          MoveEvent {
            typ: MoveState::Start,
            action,
          },
          id,
        );
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
  pub typ: MoveState,
  pub action: MoveAction,
}

fn movedebt_system(mut query: Query<(&mut MoveDebt, &Speed)>) {
  query.par_iter_mut().for_each(|(mut debt, speed)| {
    if debt.movement > 0f32 {
      debt.movement = 0f32.max(debt.movement - (speed.movement / 60f32));
    }
    if debt.rotation > -1f32 {
      debt.rotation = (-1f32).max(debt.rotation - (speed.rotation / 60f32));
    }
  })
}

fn movement_system(
  cmd: ParallelCommands,
  mut query: Query<(Entity, &MoveDebt, &Speed, &MoveAction, &mut Transform)>,
) {
  query
    .par_iter_mut()
    .for_each(|(ent, debt, speed, action, mut xform)| {
      debug!(entity = %ent, ?debt, ?speed, ?action, location = ?xform, "movement system");
      let action = *action;
      let moved = match action {
        MoveAction::MoveAbsolute(off) if debt.movement <= 0f32 => {
          xform.coords += off;
          true
        }
        MoveAction::MoveRelative(off) if debt.movement <= 0f32 => {
          let rotation = xform.facing.index();
          xform.coords += off.rotate_cw(rotation as _);
          true
        }
        MoveAction::Turn(mut dir) if debt.rotation <= 0f32 => {
          while dir < 0 {
            dir += 6
          }
          xform.facing = xform.facing.rotate_cw(dir as _);
          true
        }
        _ => false,
      };
      if moved {
        cmd.command_scope(|mut cmd| {
          debug!("completed movement, un-busying");
          let entity = cmd.entity(ent);
          entity.queue(move |mut entity: EntityWorldMut<'_>| {
            let moving = entity
              .get::<Queue>()
              .and_then(|q| {
                debug!("checking queue for movement action");
                q.front()
                  .and_then(|act| (act as &dyn Any).downcast_ref::<MoveAction>())
              })
              .is_some();
            if !moving {
              debug!("not moving, removing tag");
              entity.remove::<Moving>();
            }
            entity.remove::<(MoveAction, Busy)>();
            entity.world_scope(|world| {
              world.trigger_targets(
                MoveEvent {
                  typ: MoveState::Finish,
                  action,
                },
                ent,
              );
            });
          });
        });
      }
    })
}

const HUMAN_DIRECTIONS: [&str; 6] = [
  "forward",
  "forward and to your right",
  "backward and to your right",
  "backward",
  "backward and to your left",
  "forward and to your left",
];

fn moving_output(
  trigger: Trigger<MoveEvent>,
  output: PlayerOutput,
  locations: Query<&GlobalTransform>,
) {
  let entity = trigger.entity();
  let ev = trigger.event();
  let Some(out) = output.get(entity) else {
    return;
  };
  let Some(xform) = locations.get(entity).ok() else {
    return;
  };

  let start = match ev.typ {
    // MoveState::Start => "You start to",
    MoveState::Start => return,
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
          if hex(0, 0).main_direction_to(coords) == EdgeDirection::ALL_DIRECTIONS[i] {
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

fn stop_moving(trigger: Trigger<StopEvent>, mut cmd: Commands) {
  let entity = trigger.entity();
  cmd.entity(entity).remove::<(Busy, Moving, MoveAction)>();
}
