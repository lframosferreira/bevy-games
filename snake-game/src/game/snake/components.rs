use crate::game::snake::SNAKE_BODY_PART_SIZE;
use bevy::prelude::*;

pub struct SnakeBodyPart {
    pub size: f32,
}

impl Default for SnakeBodyPart {
    fn default() -> Self {
        SnakeBodyPart {
            size: SNAKE_BODY_PART_SIZE,
        }
    }
}

#[derive(Component)]
pub struct Snake {
    pub body: Vec<SnakeBodyPart>,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            body: vec![SnakeBodyPart::default()],
        }
    }
}
