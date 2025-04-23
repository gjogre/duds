use crate::{
    asset_manager::{AssetManager, TileSheetType},
    components::{sheetsprite::SheetSprite, walkable::Walkable},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct FloorTileBundle {
    walkable: Walkable,
    sheetsprite: SheetSprite,
}

impl Default for FloorTileBundle {
    fn default() -> Self {
        FloorTileBundle {
            walkable: Walkable::default(),
            sheetsprite: SheetSprite {
                tilesheet: TileSheetType::World,
                tilesheet_x: 0,
                tilesheet_y: 0,
            },
        }
    }
}
