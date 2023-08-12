use bevy::prelude::*;

pub const CLOUD_SPAWN_TIME: f32 = 1.5;

#[derive(Resource)]
pub struct CloudSpawnTimer {
    pub timer: Timer,
}

impl Default for CloudSpawnTimer {
    fn default() -> Self {
        CloudSpawnTimer {
            timer: Timer::from_seconds(CLOUD_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
