use crate::{
    asset_manager::{AssetManager, TileSheetType},
    components::{
        highlight::Highlighted, map_position::MapPosition, sheetsprite::SheetSprite,
        visible::Visible, walkable::Walkable,
    },
    entities::FloorTileBundle,
};
use bevy::prelude::*;
use rand::Rng;

use super::game_input::CursorState;
pub struct Map;

pub const TILE_SIZE: usize = 16;

pub fn map_to_world_coordinates(map_position: &MapPosition) -> (usize, usize) {
    (map_position.x * TILE_SIZE, map_position.y * TILE_SIZE)
}

pub fn is_inside(x: f32, y: f32, cx: f32, cy: f32, radius: f32) -> bool {
    if radius <= 0.0 {
        return false;
    }

    x > cx - (radius / 2.0)
        && x < cx + (radius / 2.0)
        && y > cy - (radius / 2.0)
        && y < cy + (radius / 2.0)
}
pub struct MapSize {
    width: usize,
    height: usize,
}
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
                let (x, y) = map_to_world_coordinates(map_position);
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
            if is_inside(
                transform.translation.x,
                transform.translation.y,
                cursor_state.world.x,
                cursor_state.world.y,
                16.0,
            ) {
                sprite.color.set_alpha(0.5);
                commands.entity(entity).insert(Highlighted);
            }
        }
    }

    pub fn unhighlight_sprite(
        cursor_state: Res<CursorState>,
        mut commands: Commands,
        mut query: Query<(Entity, &mut Sprite, &Transform), With<Highlighted>>,
    ) {
        for (entity, mut sprite, transform) in query.iter_mut() {
            if !is_inside(
                transform.translation.x,
                transform.translation.y,
                cursor_state.world.x,
                cursor_state.world.y,
                TILE_SIZE as f32,
            ) {
                sprite.color.set_alpha(1.0);
                commands.entity(entity).remove::<Highlighted>();
            }
        }
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
                        walkable: Walkable,
                    },
                    Visible,
                ));
            }
        }
    }
}
