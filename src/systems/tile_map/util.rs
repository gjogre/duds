use crate::components::tiles::MapPosition;

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
