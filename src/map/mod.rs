use crate::components::map_position::MapPosition;

pub mod map;
pub const TILE_SIZE: u32 = 16;

pub fn map_to_world_coordinates(map_position: &MapPosition) -> (u32, u32) {
    (
        map_position.x as u32 * TILE_SIZE,
        map_position.y as u32 * TILE_SIZE,
    )
}
