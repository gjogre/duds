use crate::components::{basic::PathMarker, tiles::*};
use bevy::prelude::*;

use super::util::map_to_world_coordinates;

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
