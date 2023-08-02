use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component, Default)]
pub struct SnakeBody {
    pub count: u32,
}

#[derive(Resource)]
pub struct SnakeCounter {
    pub count: u32,
}
impl Default for SnakeCounter {
    fn default() -> Self {
        // Número inicial de peças no CORPO (excluindo cabeça)
        Self { count: 1 }
    }
}

#[derive(Component, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
