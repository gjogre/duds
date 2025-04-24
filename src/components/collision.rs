use bevy::prelude::*;

#[derive(Component)]
pub struct Collision {
    pub solid: bool,
}

impl Default for Collision {
    fn default() -> Self {
        Collision { solid: true }
    }
}
