use crate::components::attributes::Moving;
use crate::components::tiles::*;
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

fn build_tile_lookup(
    tiles: &Query<(&MapPosition, Option<&Walkable>, Option<&Blocking>)>,
) -> HashMap<MapPosition, (bool, Option<u32>)> {
    let mut lookup = HashMap::new();

    for (tile_pos, maybe_walkable, maybe_blocking) in tiles.iter() {
        lookup
            .entry(*tile_pos)
            .and_modify(|(blocked, walkable)| {
                if maybe_blocking.is_some() {
                    *blocked = true;
                }
                if let Some(w) = maybe_walkable {
                    *walkable = Some(w.cost.max(1));
                }
            })
            .or_insert_with(|| {
                (
                    maybe_blocking.is_some(),
                    maybe_walkable.map(|w| w.cost.max(1)),
                )
            });
    }

    lookup
}

pub fn find_path(
    mut target_query: Query<(&MapPosition, &mut Target), Without<Moving>>,
    tiles: Query<(&MapPosition, Option<&Walkable>, Option<&Blocking>)>,
) {
    let tile_lookup = build_tile_lookup(&tiles);

    for (start_pos, mut target) in target_query.iter_mut() {
        let Some(goal_pos) = target.position else {
            continue;
        };
        if Some(goal_pos) == target.path.as_ref().and_then(|p| p.last().copied()) {
            continue; // Already at destination
        }

        let mut frontier = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost_so_far = HashMap::new();

        frontier.push(Node {
            position: *start_pos,
            cost: 0,
            priority: 0,
        });
        came_from.insert(*start_pos, None);
        cost_so_far.insert(*start_pos, 0);

        while let Some(Node { position, .. }) = frontier.pop() {
            if position == goal_pos {
                break;
            }

            for neighbor in neighbors(position) {
                if let Some(walkable_cost) = get_walkable_cost(&neighbor, &tile_lookup) {
                    let new_cost = cost_so_far.get(&position).unwrap() + walkable_cost;
                    if cost_so_far.get(&neighbor).map_or(true, |&c| new_cost < c) {
                        cost_so_far.insert(neighbor, new_cost);
                        let priority = new_cost + manhattan_distance(neighbor, goal_pos);
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
        let mut current = goal_pos;
        while let Some(prev) = came_from.get(&current).copied().flatten() {
            path.push(current);
            current = prev;
        }

        if current == *start_pos {
            path.push(*start_pos);
            path.reverse();
            target.path = Some(path);
        } else {
            target.path = None;
        }
    }
}

fn neighbors(pos: MapPosition) -> Vec<MapPosition> {
    let mut neighbors = Vec::new();

    if pos.x > 0 {
        neighbors.push(MapPosition {
            x: pos.x - 1,
            y: pos.y,
        });
    }

    if pos.x < 31 {
        neighbors.push(MapPosition {
            x: pos.x + 1,
            y: pos.y,
        });
    }

    if pos.y > 0 {
        neighbors.push(MapPosition {
            x: pos.x,
            y: pos.y - 1,
        });
    }

    if pos.y < 31 {
        neighbors.push(MapPosition {
            x: pos.x,
            y: pos.y + 1,
        });
    }

    neighbors
}

fn manhattan_distance(a: MapPosition, b: MapPosition) -> u32 {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as u32
}

fn get_walkable_cost(
    pos: &MapPosition,
    lookup: &HashMap<MapPosition, (bool, Option<u32>)>,
) -> Option<u32> {
    match lookup.get(pos) {
        Some((true, _)) => None, // Blocked
        Some((false, Some(cost))) => Some(*cost),
        _ => None,
    }
}

pub fn move_along_path(
    time: Res<Time>,
    mut query: Query<(&mut MapPosition, &mut Target, &mut Moving)>,
) {
    for (mut map_pos, mut target, mut moving) in query.iter_mut() {
        moving.timer.tick(time.delta());

        if moving.timer.finished() {
            if let Some(path) = target.path.as_mut() {
                if path.len() > 1 {
                    path.remove(0); // drop current
                    *map_pos = path[0];
                    moving.timer = Timer::from_seconds(1.0 / moving.speed, TimerMode::Once);
                } else {
                    target.path = None;
                }
            }
        }
    }
}
