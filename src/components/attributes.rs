use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Moving {
    pub speed: f32, // tiles/sec
    pub timer: Timer,
}

impl Default for Moving {
    fn default() -> Self {
        Self {
            speed: 0.5,
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}
