use super::{
  CommandArgs,
  WorldCommand,
};
use crate::{
  account::Session,
  character::Puppet,
  coords::Cubic,
  map::{
    MapCoords,
    MapFacing,
  },
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

fn turn_direction(clockwise: bool) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let puppet = world.get::<Puppet>(args.caller.unwrap()).unwrap();
      let current_facing = world.get::<MapFacing>(**puppet).unwrap();
      let new_facing = if clockwise {
        (**current_facing + 1) % 6
      } else if **current_facing > 0 {
        **current_facing - 1
      } else {
        5
      };
      world.entity_mut(**puppet).insert(MapFacing(new_facing));
    }))
  }
}

fn move_forward(fwd: bool) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let puppet = world.get::<Puppet>(args.caller.unwrap()).unwrap();
      let current_coords = world.get::<MapCoords>(**puppet).unwrap();
      let current_facing = world.get::<MapFacing>(**puppet).unwrap();
      let offset = if fwd {
        N.rotate(**current_facing as _)
      } else {
        S.rotate(**current_facing as _)
      };
      let dest_coords = **current_coords + offset;
      world.entity_mut(**puppet).insert(MapCoords(dest_coords));
    }))
  }
}

fn move_direction(offset: Cubic) -> impl Fn(CommandArgs) -> anyhow::Result<WorldCommand> {
  move |args| {
    Ok(Box::new(move |world| {
      let puppet = world.get::<Puppet>(args.caller.unwrap()).unwrap();
      let current_coords = world.get::<MapCoords>(**puppet).unwrap();
      let dest_coords = **current_coords + offset;
      world.entity_mut(**puppet).insert(MapCoords(dest_coords));
    }))
  }
}

const N: Cubic = Cubic(0, -1, 1);
const NE: Cubic = Cubic(1, -1, 0);
const SE: Cubic = Cubic(1, 0, -1);
const S: Cubic = Cubic(0, 1, -1);
const SW: Cubic = Cubic(-1, 1, 0);
const NW: Cubic = Cubic(-1, 0, 1);

command_set! { PlayerCommands =>
  ("who", who),
  ("forward", move_forward(true)),
  ("w", move_forward(true)),
  ("backward", move_forward(false)),
  ("s", move_forward(false)),
  ("right", turn_direction(true)),
  ("d", turn_direction(true)),
  ("left", turn_direction(false)),
  ("a", turn_direction(false)),
  ("north", move_direction(N)),
  ("northeast", move_direction(NE)),
  ("southeast", move_direction(SE)),
  ("south", move_direction(S)),
  ("southwest", move_direction(SW)),
  ("northwest", move_direction(NW)),
  ("n", move_direction(N)),
  ("ne", move_direction(NE)),
  ("se", move_direction(SE)),
  ("s", move_direction(S)),
  ("sw", move_direction(SW)),
  ("nw", move_direction(NW)),
}
