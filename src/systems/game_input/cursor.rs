use bevy::prelude::*;

use crate::components::{attributes::Moving, basic::Player};

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
