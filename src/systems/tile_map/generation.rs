use crate::{
    asset_manager::TileSheetType,
    components::{basic::*, tiles::*},
    entities::{FloorTileBundle, WallTileBundle},
    events::HighlightEvent,
};
use bevy::prelude::*;
use rand::Rng;

pub struct MapSize {
    width: usize,
    height: usize,
}

pub fn generate_test_map(mut commands: Commands) {
    let map = MapSize {
        width: 32,
        height: 31,
    };
    let mut rng = rand::rng();
    for x in 0..map.width {
        for y in 0..map.height {
            commands
                .spawn((
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
                ))
                .observe(walkable_hover_trigger);
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
fn walkable_hover_trigger(
    hover: Trigger<Pointer<Over>>,
    mut ev_highlight: EventWriter<HighlightEvent>,
    mut tiles: Query<(Entity, &mut Transform, Option<&Highlight>)>,
) {
    for (tile_entity, _transform, highlight) in tiles.iter_mut() {
        if tile_entity == hover.target() && highlight.is_none() {
            ev_highlight.write(HighlightEvent(tile_entity, true));
        } else if tile_entity != hover.target() && highlight.is_some() {
            ev_highlight.write(HighlightEvent(tile_entity, false));
        }
    }
}
