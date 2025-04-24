use bevy::prelude::*;

#[derive(Component)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}
impl Default for MapPosition {
    fn default() -> Self {
        MapPosition { x: 0, y: 0 }
    }
}
