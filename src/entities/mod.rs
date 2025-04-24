use crate::{
    asset_manager::TileSheetType,
    components::{
        collision::Collision, map_position::MapPosition, sheetsprite::SheetSprite,
        walkable::Walkable,
    },
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct FloorTileBundle {
    pub walkable: Walkable,
    pub sheetsprite: SheetSprite,
    pub map_position: MapPosition,
}

impl Default for FloorTileBundle {
    fn default() -> Self {
        FloorTileBundle {
            walkable: Walkable::default(),
            sheetsprite: SheetSprite {
                tilesheet: TileSheetType::World,
                tilesheet_x: 5,
                tilesheet_y: 8,
            },
            map_position: MapPosition::default(),
        }
    }
}

#[derive(Bundle)]
pub struct WallTileBundle {
    pub collision: Collision,
    pub sheetsprite: SheetSprite,
    pub map_position: MapPosition,
}

impl Default for WallTileBundle {
    fn default() -> Self {
        WallTileBundle {
            collision: Collision::default(),
            sheetsprite: SheetSprite {
                tilesheet: TileSheetType::World,
                tilesheet_x: 16,
                tilesheet_y: 6,
            },
            map_position: MapPosition::default(),
        }
    }
}
