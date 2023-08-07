use bevy::prelude::*;

use super::OBSTACLE_INITIAL_SPEED;

pub const OBSTACLE_SPAWN_TIME: f32 = 2.0;

#[derive(Resource)]
pub struct ObstacleSpawnTimer {
    pub timer: Timer,
}

impl Default for ObstacleSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(OBSTACLE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct ObstacleSpeed {
    pub speed: f32,
}

impl Default for ObstacleSpeed {
    fn default() -> Self {
        Self {
            speed: OBSTACLE_INITIAL_SPEED,
        }
    }
}
