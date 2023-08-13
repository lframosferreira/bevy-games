use crate::game::components::Stats;
use bevy::prelude::*;

const LASER_LENGTH: f32 = 8.;
const LASER_SIZE: Vec2 = Vec2::new(LASER_LENGTH, LASER_LENGTH);
const LASER_SPEED: f32 = 250.;
const LASER_COLOR: Color = Color::RED;

#[derive(Component, Clone, Copy)]
pub struct Laser {
    pub stats: Stats,
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                LASER_LENGTH,
                LASER_LENGTH,
                LASER_SIZE,
                LASER_SPEED,
                LASER_COLOR,
            ),
        }
    }
}
