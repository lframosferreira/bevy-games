use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub struct Block(pub Color, pub usize, pub usize);

impl Block {
    pub fn new(x: usize, y: usize) -> Self {
        let color = match rand::thread_rng().gen_range(0..6) {
            0 => Color::PINK,
            1 => Color::ORANGE_RED,
            2 => Color::GREEN,
            3 => Color::BLUE,
            4 => Color::YELLOW,
            _ => Color::CYAN,
        };
        Block(color, x, y)
    }
}

#[derive(Component)]
pub struct Heart(pub usize);
