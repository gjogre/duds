use crate::{
    components::{
        basic::Player,
        tiles::{Highlight, MapPosition, Target},
    },
    events::HighlightEvent,
    systems::game_input::cursor::CursorPosition,
};
use bevy::prelude::*;

use super::util::is_inside;

pub fn highlight_mouse_hover(
    cursor_state: Res<CursorPosition>,
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
