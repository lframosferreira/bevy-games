use bevy::prelude::Component;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component, Default)]
pub struct SnakeBody {
    pub count: u32,
}

#[derive(Component, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
