use hexx::{
  hex,
  EdgeDirection,
};
use tracing::warn;

use super::{
  CommandArgs,
  WorldCommand,
};
use crate::{
  account::Session,
  action::{
    Queue,
    StopEvent,
  },
  character::Puppet,
  movement::MoveAction,
  net::TelnetOut,
};

fn who(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  Ok(Box::new(move |world| {
    let players = world
      .query::<&Session>()
      .iter(world)
      .map(|s| s.username.clone())
      .collect::<Vec<_>>();
    let out = if players.len() > 1 {
      let mut out = format!("There are {} players online:", players.len());
      for player in players {
        out.push_str("\n    ");
        out.push_str(&player);
      }
      out
    } else {
      String::from("It's just you!")
    };

    world
      .entity(args.caller.unwrap())
      .get::<TelnetOut>()
      .unwrap()
      .line(out);
  }))
}

fn turn_direction(off: i8) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let entity = world.get::<Puppet>(args.caller.unwrap()).unwrap().0;
      let mut action_queue = try_opt!(world.get_mut::<Queue>(entity), {
        warn!(?entity, "entity has no queue!");
        return;
      });
      action_queue.push_back(Box::new(MoveAction::Turn(off)));
      let out = try_opt!(world.get::<TelnetOut>(args.caller.unwrap()), return);
      out.line("Adding movement to queue.");
    }))
  }
}

fn move_relative(dir: EdgeDirection) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let entity = world.get::<Puppet>(args.caller.unwrap()).unwrap().0;
      let mut queue = try_opt!(world.get_mut::<Queue>(entity), {
        warn!(?entity, "entity has no queue!");
        return;
      });
      queue.push_back(Box::new(MoveAction::MoveRelative(hex(0, 0) + dir)));
      let out = try_opt!(world.get::<TelnetOut>(args.caller.unwrap()), return);
      out.line("Adding movement to queue.");
    }))
  }
}

fn move_absolute(dir: EdgeDirection) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let entity = world.get::<Puppet>(args.caller.unwrap()).unwrap().0;
      let mut queue = try_opt!(world.get_mut::<Queue>(entity), {
        warn!(?entity, "entity has no queue!");
        return;
      });
      queue.push_back(Box::new(MoveAction::MoveAbsolute(hex(0, 0) + dir)));
      let out = try_opt!(world.get::<TelnetOut>(args.caller.unwrap()), return);
      out.line("Adding movement to queue.");
    }))
  }
}

fn stop(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  Ok(Box::new(move |world| {
    let mut puppet = try_opt!(
      args
        .caller
        .and_then(|ent| world.get::<Puppet>(ent).copied())
        .and_then(|ent| world.get_entity_mut(ent.0)),
      return
    );
    if let Some(mut queue) = puppet.get_mut::<Queue>() {
      queue.clear();
    }
    let id = puppet.id();
    world.trigger_targets(StopEvent, id);
  }))
}

const N: EdgeDirection = EdgeDirection::FLAT_NORTH;
const NE: EdgeDirection = EdgeDirection::FLAT_NORTH_EAST;
const SE: EdgeDirection = EdgeDirection::FLAT_SOUTH_EAST;
const S: EdgeDirection = EdgeDirection::FLAT_SOUTH;
const SW: EdgeDirection = EdgeDirection::FLAT_SOUTH_WEST;
const NW: EdgeDirection = EdgeDirection::FLAT_NORTH_WEST;

const FWD: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[0];
const FWD_RIGHT: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[1];
const BACK_RIGHT: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[2];
const BACK: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[3];
const BACK_LEFT: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[4];
const FWD_LEFT: EdgeDirection = EdgeDirection::ALL_DIRECTIONS[5];

command_set! { PlayerCommands =>
  ("who", who),
  ("forward", move_relative(FWD)),
  ("forwardright", move_relative(FWD_RIGHT)),
  ("forwardleft", move_relative(FWD_LEFT)),
  ("backward", move_relative(BACK)),
  ("backwardright", move_relative(BACK_RIGHT)),
  ("backwardleft", move_relative(BACK_LEFT)),
  ("right", turn_direction(1)),
  ("left", turn_direction(-1)),
  ("north", move_absolute(N)),
  ("northeast", move_absolute(NE)),
  ("southeast", move_absolute(SE)),
  ("south", move_absolute(S)),
  ("southwest", move_absolute(SW)),
  ("northwest", move_absolute(NW)),
  ("n", move_absolute(N)),
  ("ne", move_absolute(NE)),
  ("se", move_absolute(SE)),
  ("s", move_absolute(S)),
  ("sw", move_absolute(SW)),
  ("nw", move_absolute(NW)),
  ("stop", stop),
}
