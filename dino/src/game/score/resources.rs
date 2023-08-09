use bevy::prelude::{Resource, Timer, TimerMode};

const SCORE_UPDATE_TIME: f32 = 1.0;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct ScoreUpdateTimer {
    pub timer: Timer,
}

impl Default for ScoreUpdateTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SCORE_UPDATE_TIME, TimerMode::Repeating),
        }
    }
}
