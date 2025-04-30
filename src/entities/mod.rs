use crate::asset_manager::TileSheetType;
use crate::components::tiles::*;
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
    pub blocking: Blocking,
    pub sheetsprite: SheetSprite,
    pub map_position: MapPosition,
}

impl Default for WallTileBundle {
    fn default() -> Self {
        WallTileBundle {
            sheetsprite: SheetSprite {
                tilesheet: TileSheetType::World,
                tilesheet_x: 16,
                tilesheet_y: 6,
            },
            blocking: Blocking,
            map_position: MapPosition::default(),
        }
    }
}
