use crate::{
    asset_manager::TileSheetType,
    components::{basic::*, tiles::*},
    entities::{FloorTileBundle, WallTileBundle},
};
use bevy::prelude::*;
use rand::Rng;

pub struct MapSize {
    width: usize,
    height: usize,
}

pub fn generate_test_map(mut commands: Commands) {
    let map = MapSize {
        width: 32,
        height: 31,
    };
    let mut rng = rand::rng();
    for x in 0..map.width {
        for y in 0..map.height {
            commands.spawn((
                FloorTileBundle {
                    map_position: MapPosition { x, y },
                    sheetsprite: SheetSprite {
                        tilesheet: TileSheetType::World,
                        tilesheet_x: 5,
                        tilesheet_y: rng.random_range(8..12),
                    },
                    walkable: Walkable { cost: 1 },
                },
                Visible,
            ));
            if rng.random_range(0..10) > 8 {
                commands.spawn((
                    WallTileBundle {
                        map_position: MapPosition { x, y },
                        ..default()
                    },
                    Visible,
                ));
            }
        }
    }
}
