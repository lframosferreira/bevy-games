use super::{HEAD_X, HEAD_Y};
use crate::game::BLOCK_SIZE;
use bevy::prelude::*;

pub struct SnakeBodyPart {
    pub size: f32,
}

impl Default for SnakeBodyPart {
    fn default() -> Self {
        SnakeBodyPart { size: BLOCK_SIZE }
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
            head_x_pos: HEAD_X,
            head_y_pos: HEAD_Y,
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
