use super::DINO_INITIAL_VERTICAL_SPEED;
use bevy::prelude::Resource;

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

#[derive(Resource, Default)]
pub struct DinoDown {
    pub is_down: bool,
}
