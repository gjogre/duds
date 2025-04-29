use crate::{
    asset_manager::TileSheetType,
    components::{
        highlight::Highlight, layer::Layer, map_position::MapPosition, path_marker::PathMarker,
        player::Player, sheetsprite::SheetSprite, target::Target, visible::Visible,
        walkable::Walkable,
    },
    entities::{FloorTileBundle, WallTileBundle},
    events::HighlightEvent,
};
use bevy::prelude::*;
use rand::Rng;

use super::game_input::CursorState;

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

pub fn highlight_mouse_hover(
    cursor_state: Res<CursorState>,
    mut ev_highlight: EventWriter<HighlightEvent>,
    mut query: Query<(Entity, &Transform, Option<&Highlight>)>,
) {
    for (entity, transform, highlight) in query.iter_mut() {
        let hovered = is_inside(
            transform.translation.x,
            transform.translation.y,
            cursor_state.world.x,
            cursor_state.world.y,
            16.0,
        );
        match (hovered, highlight) {
            (true, None) => {
                ev_highlight.write(HighlightEvent(entity, true));
            }
            (false, Some(_)) => {
                ev_highlight.write(HighlightEvent(entity, false));
            }
            _ => {}
        }
    }
}

pub fn highlight_changed(
    mut commands: Commands,
    mut ev_highlight: EventReader<HighlightEvent>,
    mut tile_query: Query<(&mut Sprite, &MapPosition)>,
    mut player_query: Query<&mut Target, With<Player>>,
) {
    let mut target_changed = false;
    let mut target_map_position = None;
    for ev in ev_highlight.read() {
        if let Ok((mut sprite, map_position)) = tile_query.get_mut(ev.0) {
            if ev.1 == true {
                commands.entity(ev.0).insert(Highlight);
                sprite.color.set_alpha(0.5);

                target_changed = true;
                target_map_position = Some(*map_position);
            }
            if ev.1 == false {
                commands.entity(ev.0).remove::<Highlight>();
                sprite.color.set_alpha(1.0);
            }
        }
    }
    if target_changed {
        if let Ok(mut player_target) = player_query.single_mut() {
            player_target.position = target_map_position;
        }
    }
}
pub fn visualize_target_path(
    mut commands: Commands,
    target_query: Query<&Target, Changed<Target>>,
    marker_query: Query<Entity, With<PathMarker>>,
) {
    if !target_query.is_empty() {
        // clear old markers
        for entity in marker_query.iter() {
            commands.entity(entity).try_despawn();
        }
    }

    for target in target_query.iter() {
        if let Some(path) = &target.path {
            for map_pos in path {
                let (x, y) = map_to_world_coordinates(map_pos);
                commands.spawn((
                    Sprite {
                        color: Color::srgba(0.2, 0.8, 1.0, 0.3),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    },
                    Transform::from_xyz(x as f32, y as f32, 1.0),
                    Layer(1),
                    PathMarker,
                ));
            }
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
