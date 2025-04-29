use bevy::prelude::*;

use super::map_position::MapPosition;

#[derive(Component)]
pub struct Target {
    pub path: Option<Vec<MapPosition>>,
    pub position: Option<MapPosition>,
}
