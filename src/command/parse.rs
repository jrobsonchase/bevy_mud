use chumsky::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Node {
  Word(String),
  Quoted(String),
  Space,
}

pub fn command<'a>() -> impl Parser<'a, &'a str, Vec<Node>, extra::Err<Rich<'a, char>>> {
  let word = any()
    .filter(|c: &char| !c.is_whitespace() && *c != '"')
    .repeated()
    .at_least(1)
    .collect()
    .map(Node::Word);

  let escaped = just("\\\"").to('"');
  let quoted = escaped
    .or(none_of("\""))
    .repeated()
    .collect()
    .delimited_by(just('"'), just('"'))
    .map(Node::Quoted);

  let space = any()
    .filter(|c: &char| c.is_whitespace())
    .repeated()
    .at_least(1)
    .map(|_| Node::Space);

  word
    .or(quoted)
    .or(space)
    .repeated()
    .collect()
    .then_ignore(end())
}

#[cfg(test)]
mod test {
  use ariadne::{
    Color,
    Config,
    Label,
    Report,
    ReportKind,
    Source,
  };

  use super::*;

  #[test]
  fn parser() {
    const INPUT: &str = "Hello, world\"!";

    let parsed = match command().parse(INPUT).into_result() {
      Ok(p) => p,
      Err(e) => {
        let mut out = Vec::new();
        e.iter().for_each(|e| {
          let report = || {
            Report::build(ReportKind::Error, (), e.span().start)
              .with_message(e.to_string())
              .with_label(
                Label::new(e.span().into_range())
                  .with_message(e.reason().to_string())
                  .with_color(Color::Blue),
              )
          };

          report()
            .with_config(Config::default().with_compact(true))
            .finish()
            .write_for_stdout(Source::from(INPUT), &mut out)
            .unwrap();

          report()
            .finish()
            .write(Source::from(INPUT), &mut out)
            .unwrap();
        });
        panic!("{}", String::from_utf8_lossy(&out));
      }
    };

    assert_eq!(
      parsed,
      vec![
        Node::Word("Hello,".into()),
        Node::Space,
        Node::Quoted("world".into()),
        Node::Word("!".into()),
      ]
    );
  }
}
