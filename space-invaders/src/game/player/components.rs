use crate::game::components::Stats;
use bevy::prelude::*;

const PLAYER_WIDTH: f32 = 40.;
const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
const PLAYER_SPEED: f32 = 250.;
const PLAYER_COLOR: Color = Color::GREEN;

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub stats: Stats,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            stats: Stats::new(
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
                PLAYER_SIZE,
                PLAYER_SPEED,
                PLAYER_COLOR,
            ),
        }
    }
}
