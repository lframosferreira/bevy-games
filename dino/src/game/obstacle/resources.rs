use bevy::prelude::*;

pub const OBSTACLE_SPAWN_TIME: f32 = 2.0;

#[derive(Resource)]
pub struct ObstacleSpawnTimer {
    pub timer: Timer,
}

impl Default for ObstacleSpawnTimer {
    fn default() -> Self {
        ObstacleSpawnTimer {
            timer: Timer::from_seconds(OBSTACLE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
