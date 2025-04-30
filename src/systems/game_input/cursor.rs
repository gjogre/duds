use bevy::prelude::*;

use crate::components::{attributes::Moving, basic::Player};

#[derive(Resource)]
pub struct CursorPosition {
    pub screen: Vec2,
    pub world: Vec2,
}

pub fn cursor_events(
    mut cursor_state: ResMut<CursorPosition>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, camera_transform)) = q_camera.single() {
        if let Ok(local_world_position) = camera
            .viewport_to_world(camera_transform, cursor_state.screen)
            .map(|ray| ray.origin.truncate())
        {
            cursor_state.world = local_world_position;
        }
    } else {
        println!("No camera or multiple cameras found");
    }
}

pub fn cursor_moved(
    mut evr_cursor: EventReader<CursorMoved>,
    mut cursor_state: ResMut<CursorPosition>,
) {
    if let Some(last_event) = evr_cursor.read().last() {
        cursor_state.screen = last_event.position;
    }
}

pub fn cursor_clicked(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    player_query: Query<(Entity, Option<&Moving>), With<Player>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if let Ok((entity, moving)) = player_query.single() {
        if moving.is_none() {
            commands.entity(entity).insert(Moving {
                speed: 3.,
                ..default()
            });
        } else {
            commands.entity(entity).remove::<Moving>();
        }
    } else {
        println!("cursor_clicked: Player not found!");
    }
}
