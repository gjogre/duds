use crate::asset_manager::TileSheetType;
use bevy::prelude::*;

#[derive(Component)]
pub struct Blocking;

#[derive(Component)]
pub struct Highlight;

#[derive(Component, Default)]
pub struct Layer(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}
impl Default for MapPosition {
    fn default() -> Self {
        MapPosition { x: 0, y: 0 }
    }
}

#[derive(Component)]
pub struct SheetSprite {
    pub tilesheet: TileSheetType,
    pub tilesheet_x: u32,
    pub tilesheet_y: u32,
}

#[derive(Component)]
pub struct Target {
    pub path: Option<Vec<MapPosition>>,
    pub position: Option<MapPosition>,
}

#[derive(Component, Default)]
pub struct Walkable {
    pub cost: u32,
}
