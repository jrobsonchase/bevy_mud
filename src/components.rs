use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Position(crate::coords::Cubic);
