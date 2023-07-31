use super::SNAKE_BODY_PART_SIZE;
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
    pub head_x_pos: f32,
    pub head_y_pos: f32,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            head_x_pos: 500.0,
            head_y_pos: 300.0,
            body: vec![SnakeBodyPart::default()],
        }
    }
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
