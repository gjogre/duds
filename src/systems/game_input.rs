use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorState {
    pub screen: Vec2,
    pub world: Vec2,
}

pub fn cursor_events(
    mut cursor_state: ResMut<CursorState>,
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
    mut cursor_state: ResMut<CursorState>,
) {
    if let Some(last_event) = evr_cursor.read().last() {
        cursor_state.screen = last_event.position;
    }
}
