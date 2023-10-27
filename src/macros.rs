#[macro_export]
macro_rules! extract {
  ($v:expr, $pat:pat $(,)*) => {
    if matches!($v, $pat) {
      Ok($v)
    } else {
      Err($v)
    }
  };
  ($v:expr, $pat:pat => $res:expr $(,)*) => {
    match $v {
      $pat => Ok($res),
      event => Err(event),
    }
  };
}

#[macro_export]
macro_rules! try_opt {
  ($opt:expr, $block:expr $(,)*) => {{
    #[allow(clippy::redundant_closure_call)]
    let res = (|| $opt)();
    match res {
      Some(v) => v,
      None => $block,
    }
  }};
}

#[macro_export]
macro_rules! try_res {
  ($opt:expr, $e:tt => $block:expr $(,)*) => {{
    #[allow(clippy::redundant_closure_call)]
    let res = (|| $opt)();
    match res {
      Ok(v) => v,
      Err($e) => $block,
    }
  }};
}
