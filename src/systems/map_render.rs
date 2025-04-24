use crate::{
    asset_manager::{self, AssetManager, TileSheetType},
    components::{map_position::MapPosition, sheetsprite::SheetSprite},
    map,
};

use bevy::prelude::*;
pub struct MapRender;
impl MapRender {
    pub fn attach_sprites(
        mut commands: Commands,
        asset_manager: Res<AssetManager>,
        query: Query<(Entity, &SheetSprite, &MapPosition), Without<Sprite>>,
    ) {
        for (entity, sheet_sprite, map_position) in query.iter() {
            if let Some(sprite) = asset_manager.get_sprite(
                &sheet_sprite.tilesheet,
                sheet_sprite.tilesheet_x,
                sheet_sprite.tilesheet_y,
            ) {
                let (x, y) = map::map_to_world_coordinates(map_position);
                //println!("Creating Sprite coordinates: ({}, {})", x, y);
                commands
                    .entity(entity)
                    .insert((sprite, Transform::from_xyz(x as f32, y as f32, 0.0)));
            } else {
                warn!("Failed to get sprite for entity {:?}", entity);
            }
        }
    }
}
