use tracing::warn;

use super::{
  CommandArgs,
  WorldCommand,
};
use crate::{
  account::Session,
  character::Puppet,
  coords::{
    Cubic,
    DIRECTIONS,
  },
  map::Transform,
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
      let mut current_xform = try_opt!(world.get_mut::<Transform>(entity), {
        warn!(?entity, "attempt to turn locationless entity");
        return;
      });
      current_xform.facing += off;
      current_xform.facing %= 6;
    }))
  }
}

fn move_relative(dir: usize) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let entity = world.get::<Puppet>(args.caller.unwrap()).unwrap().0;
      let mut xform = try_opt!(world.get_mut::<Transform>(entity), {
        warn!(?entity, "attempt to move locationless entity");
        return;
      });
      let offset = DIRECTIONS[dir].rotate(xform.facing);
      xform.coords += offset;
    }))
  }
}

fn move_absolute(dir: usize) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let entity = world.get::<Puppet>(args.caller.unwrap()).unwrap().0;
      let mut xform = try_opt!(world.get_mut::<Transform>(entity), {
        warn!(?entity, "attempt to move locationless entity");
        return;
      });
      xform.coords += DIRECTIONS[dir];
    }))
  }
}

const N: usize = 2;
const NE: usize = 1;
const SE: usize = 0;
const S: usize = 5;
const SW: usize = 4;
const NW: usize = 3;

command_set! { PlayerCommands =>
  ("who", who),
  ("forward", move_relative(2)),
  ("forwardright", move_relative(1)),
  ("forwardleft", move_relative(3)),
  ("backward", move_relative(5)),
  ("backwardright", move_relative(0)),
  ("backwardleft", move_relative(4)),
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
}
