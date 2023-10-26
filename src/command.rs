use std::collections::VecDeque;

use bevy::prelude::{
  Component,
  *,
};

use crate::net::TelnetIn;

/// Parse a command string into a list of tokens.
/// Currently only splits on space and allows double-quoted strings.
pub fn parse_command(input: &str) -> Vec<String> {
  let mut out = vec![];
  let mut current = String::new();
  let mut in_string = false;
  let mut chars = input.chars().peekable();
  while let Some(c) = chars.next() {
    match c {
      '\\' => {
        if let Some('"') = chars.peek() {
          current.push('"');
          chars.next();
        } else {
          current.push(c);
        }
      }
      '"' => {
        if in_string {
          in_string = false;
          out.push(current);
          current = String::new();
        } else {
          in_string = true;
        }
      }
      ' ' | '\t' if !in_string => {
        if !current.is_empty() {
          out.push(current);
          current = String::new();
        }
      }
      _ => current.push(c),
    }
  }
  if !current.is_empty() {
    out.push(current)
  }
  out
}

#[derive(Component, Debug, Default)]
pub struct CommandQueue(pub VecDeque<Vec<String>>);

impl CommandQueue {
  pub fn first_command(&self) -> Option<&str> {
    self
      .0
      .front()
      .and_then(|cmd| cmd.get(0).map(|s| s.as_str()))
  }

  pub fn dequeue(&mut self) -> Option<Vec<String>> {
    self.0.pop_front()
  }
}

pub fn parse_commands_system(mut query: Query<(Entity, &mut CommandQueue, &mut TelnetIn)>) {
  query
    .par_iter_mut()
    .for_each(|(entity, mut output, mut input)| {
      while let Some(line) = input.next_line() {
        let parsed = parse_command(&line);
        debug!(?entity, line, ?parsed, "parsed command line");
        output.0.push_back(parsed);
      }
    })
}

#[cfg(test)]
mod test {
  use crate::command::parse_command;

  #[test]
  fn test_parse() {
    const CASES: &[(&str, &[&str])] = &[
      ("Hello, world!", &["Hello,", "world!"]),
      ("Hello, \"lovely world!\"", &["Hello,", "lovely world!"]),
      (
        "Hello, \"lovely \\\"world\\\"!\"",
        &["Hello,", "lovely \"world\"!"],
      ),
    ];
    for (input, expected) in CASES {
      let output = parse_command(input);

      assert_eq!(
        expected
          .iter()
          .map(|s| s.to_string())
          .collect::<Vec<String>>(),
        output,
      );
    }
  }
}
