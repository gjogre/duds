use crate::{
    asset_manager::AssetManager,
    components::{highlight::Highlighted, map_position::MapPosition, sheetsprite::SheetSprite},
    map,
};

use bevy::prelude::*;

use super::game_input::CursorState;
pub struct Map;

impl Map {
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

    pub fn highlight_sprite(
        cursor_state: Res<CursorState>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut Sprite, &Transform), Without<Highlighted>>,
    ) {
        for (entity, mut sprite, transform) in query.iter_mut() {
            if transform.translation.x > cursor_state.world.x - 8.0
                && transform.translation.x < cursor_state.world.x + 8.0
                && transform.translation.y > cursor_state.world.y - 8.0
                && transform.translation.y < cursor_state.world.y + 8.0
            {
                sprite.color.set_alpha(0.5);
                commands.entity(entity).insert(Highlighted);
            }
        }
    }
}
