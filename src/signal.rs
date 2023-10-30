use std::sync::{
  atomic::AtomicUsize,
  Arc,
};

use bevy::prelude::*;
use signal_hook::{
  consts,
  flag,
};

use crate::core::CantonStartup;

#[allow(clippy::upper_case_acronyms)]
#[derive(Event, Debug, Copy, Clone)]
pub enum Signal {
  SIGINT,
  SIGTERM,
  SIGQUIT,
  SIGUSR1,
  SIGUSR2,
}

#[derive(Resource)]
struct SignalFlag(Arc<AtomicUsize>);

impl SignalFlag {
  fn get(&self) -> Option<libc::c_int> {
    match self.0.swap(0, std::sync::atomic::Ordering::Relaxed) {
      0 => None,
      n => Some(n as _),
    }
  }
}

macro_rules! register {
  ($sig:tt, $flag:expr) => {
    flag::register_usize(consts::$sig, $flag.clone(), consts::$sig as usize).expect(concat!(
      "failed to register ",
      stringify!($sig),
      " handler"
    ));
  };
}

fn start_handler(mut cmd: Commands) {
  let flag = Arc::new(AtomicUsize::new(0));
  register!(SIGINT, flag);
  register!(SIGTERM, flag);
  register!(SIGQUIT, flag);
  register!(SIGUSR1, flag);
  register!(SIGUSR2, flag);
  cmd.insert_resource(SignalFlag(flag));
}

fn check_flag(flag: Res<SignalFlag>, mut signal_writer: EventWriter<Signal>) {
  let signal = match flag.get() {
    Some(consts::SIGINT) => Signal::SIGINT,
    Some(consts::SIGTERM) => Signal::SIGTERM,
    Some(consts::SIGQUIT) => Signal::SIGQUIT,
    Some(consts::SIGUSR1) => Signal::SIGUSR1,
    Some(consts::SIGUSR2) => Signal::SIGUSR2,
    _ => return,
  };
  signal_writer.send(signal);
}

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<Signal>()
      .add_systems(Startup, start_handler.in_set(CantonStartup::System))
      .add_systems(Update, check_flag);
  }
}
