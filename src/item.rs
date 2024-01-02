use bevy::prelude::*;

/// Marker for an item.
#[derive(Component, Reflect, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Component, Hash)]
pub struct Item;
