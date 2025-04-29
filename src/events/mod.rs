use bevy::prelude::*;

#[derive(Event)]
pub struct HighlightEvent(pub Entity, pub bool); // true is add, false is remove
