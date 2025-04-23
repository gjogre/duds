use bevy::prelude::*;

#[derive(Component)]
pub struct Walkable(u32);

impl Default for Walkable {
    fn default() -> Self {
        Walkable(1)
    }
}
