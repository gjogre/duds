use crate::components::highlight::Highlight;
use crate::components::map_position::MapPosition;
use crate::components::player::Player;
use crate::components::walkable::Walkable;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct Node {
    position: MapPosition,
    cost: u32,
    priority: u32,
}

// For BinaryHeap to be a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(
    player_query: Query<&MapPosition, With<Player>>,
    highlight_query: Query<&MapPosition, With<Highlight>>,
    tiles: &Query<(Entity, &MapPosition, Option<&Walkable>)>,
) -> Option<Vec<MapPosition>> {
    let mut frontier = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    if let (Ok(player_pos), Ok(goal_pos)) = (player_query.single(), highlight_query.single()) {
        frontier.push(Node {
            position: *player_pos,
            cost: 0,
            priority: 0,
        });
        came_from.insert(*player_pos, None);
        cost_so_far.insert(*player_pos, 0);

        while let Some(Node { position, .. }) = frontier.pop() {
            if position == *goal_pos {
                break;
            }

            for neighbor in neighbors(position) {
                if let Some(walkable_cost) = get_walkable_cost(&neighbor, tiles) {
                    let new_cost = cost_so_far.get(&position).unwrap() + walkable_cost;

                    if cost_so_far.get(&neighbor).map_or(true, |&c| new_cost < c) {
                        cost_so_far.insert(neighbor, new_cost);
                        let priority = new_cost + manhattan_distance(neighbor, *goal_pos);
                        frontier.push(Node {
                            position: neighbor,
                            cost: new_cost,
                            priority,
                        });
                        came_from.insert(neighbor, Some(position));
                    }
                }
            }
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = *goal_pos;
        while let Some(prev) = came_from.get(&current).copied().flatten() {
            path.push(current);
            current = prev;
        }

        if current == *player_pos {
            path.push(*player_pos);
            path.reverse();
            Some(path)
        } else {
            None
        }
    } else {
        None
    }
}

fn neighbors(pos: MapPosition) -> Vec<MapPosition> {
    vec![
        MapPosition {
            x: pos.x + 1,
            y: pos.y,
        },
        MapPosition {
            x: pos.x - 1,
            y: pos.y,
        },
        MapPosition {
            x: pos.x,
            y: pos.y + 1,
        },
        MapPosition {
            x: pos.x,
            y: pos.y - 1,
        },
    ]
}

fn manhattan_distance(a: MapPosition, b: MapPosition) -> u32 {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as u32
}

// Now Walkable and Cost aware
fn get_walkable_cost(
    pos: &MapPosition,
    tiles: &Query<(Entity, &MapPosition, Option<&Walkable>)>,
) -> Option<u32> {
    for (_, tile_pos, maybe_walkable) in tiles.iter() {
        if tile_pos == pos {
            if let Some(walkable) = maybe_walkable {
                return Some(walkable.cost.max(1)); // Never 0 cost
            }
        }
    }
    None
}
