use crate::game::components::Stats;
use bevy::prelude::*;

const BULLET_LENGTH: f32 = 5.;
const BULLET_SIZE: Vec2 = Vec2::new(BULLET_LENGTH, BULLET_LENGTH);
const BULLET_SPEED: f32 = 250.;
const BULLET_COLOR: Color = Color::GREEN;

#[derive(Component, Clone, Copy)]
pub struct Bullet {
    pub stats: Stats,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                BULLET_LENGTH,
                BULLET_LENGTH,
                BULLET_SIZE,
                BULLET_SPEED,
                BULLET_COLOR,
            ),
        }
    }
}
