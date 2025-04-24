use bevy::prelude::*;

use crate::{
    components::{map_position::MapPosition, visible::Visible},
    entities::FloorTileBundle,
};

pub struct Map {
    width: usize,
    height: usize,
}

pub fn generate_test_map(mut commands: Commands) {
    let map = Map {
        width: 32,
        height: 32,
    };
    for x in 0..map.width {
        for y in 0..map.height {
            commands.spawn((
                FloorTileBundle {
                    map_position: MapPosition { x, y },
                    ..default()
                },
                Visible,
            ));
        }
    }
}
