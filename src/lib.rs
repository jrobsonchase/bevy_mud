#![allow(clippy::type_complexity)]

#[macro_use]
pub mod macros;

#[macro_use]
pub mod net;

pub mod util;

pub mod account;
pub mod action;
pub mod character;
pub mod command;
pub mod item;
pub mod movement;
pub mod output;

pub mod coords;
pub mod core;
pub mod framerate;
pub mod oneshot;
pub mod signal;

pub mod ascii_map;
pub mod map;
