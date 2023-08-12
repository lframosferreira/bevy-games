use crate::game::components::Stats;
use bevy::prelude::*;

const BARRIER_LENGTH: f32 = 70.;
const BARRIER_SIZE: Vec2 = Vec2::new(BARRIER_LENGTH, BARRIER_LENGTH);
const BARRIER_SPEED: f32 = 0.;
const BARRIER_COLOR: Color = Color::GREEN;

#[derive(Component, Clone, Copy)]
pub struct Barrier {
    pub stats: Stats,
}

impl Default for Barrier {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                BARRIER_LENGTH,
                BARRIER_LENGTH,
                BARRIER_SIZE,
                BARRIER_SPEED,
                BARRIER_COLOR,
            ),
        }
    }
}
