use super::{
  CommandArgs,
  WorldCommand,
};
use crate::{
  account::Session,
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

command_set! { PlayerCommands =>
  ("who", who)
}
