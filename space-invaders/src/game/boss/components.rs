use crate::game::components::Stats;
use bevy::prelude::*;

const BOSS_WIDTH: f32 = 60.;
const BOSS_HEIGHT: f32 = 30.;
const BOSS_SIZE: Vec2 = Vec2::new(BOSS_WIDTH, BOSS_HEIGHT);
const BOSS_SPEED: f32 = 200.;
const BOSS_COLOR: Color = Color::WHITE;

#[derive(Component, Clone, Copy)]
pub struct Boss {
    pub stats: Stats,
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            stats: Stats::new(BOSS_WIDTH, BOSS_HEIGHT, BOSS_SIZE, BOSS_SPEED, BOSS_COLOR),
        }
    }
}
