use bevy::prelude::*;
use hexer::Cubic;

#[derive(Component, Copy, Clone)]
pub struct Position(Cubic);
