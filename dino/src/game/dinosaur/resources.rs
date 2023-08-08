use bevy::prelude::Resource;

use super::DINO_INITIAL_VERTICAL_SPEED;

#[derive(Resource)]
pub struct DinoVerticalMovement {
    pub moving: bool,
    pub speed: f32,
}

impl Default for DinoVerticalMovement {
    fn default() -> Self {
        Self {
            moving: false,
            speed: DINO_INITIAL_VERTICAL_SPEED,
        }
    }
}

#[derive(Resource)]
pub struct DinoDown {
    pub is_down: bool,
}

impl Default for DinoDown {
    fn default() -> Self {
        Self { is_down: false }
    }
}
