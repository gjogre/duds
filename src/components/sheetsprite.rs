use crate::asset_manager::TileSheetType;
use bevy::prelude::*;

#[derive(Component)]
pub struct SheetSprite {
    pub tilesheet: TileSheetType,
    pub tilesheet_x: u32,
    pub tilesheet_y: u32,
}
