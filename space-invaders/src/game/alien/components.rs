use crate::game::components::Stats;
use bevy::prelude::*;

const ALIEN_LENGTH: f32 = 30.;
const ALIEN_SIZE: Vec2 = Vec2::new(ALIEN_LENGTH, ALIEN_LENGTH);
const ALIEN_SPEED: f32 = 25. * 40.;
const ALIEN_COLOR: Color = Color::RED;

#[derive(Component, Clone, Copy)]
pub struct Alien {
    pub stats: Stats,
}

impl Default for Alien {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                ALIEN_LENGTH,
                ALIEN_LENGTH,
                ALIEN_SIZE,
                ALIEN_SPEED,
                ALIEN_COLOR,
            ),
        }
    }
}
