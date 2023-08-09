use bevy::prelude::{Color, Component};
use rand::{thread_rng, Rng};

#[derive(Component, Clone, Copy)]
pub struct Block(pub usize, usize, usize);

impl Default for Block {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..4);
        let y = rng.gen_range(0..4);
        let value = if rng.gen_range(0..10) == 0 { 4 } else { 2 };
        Block(value, x, y)
    }
}

impl Block {
    pub fn get_color(&self) -> Color {
        match self.0 {
            2 => Color::RED,
            4 => Color::YELLOW,
            8 => Color::BLUE,
            16 => Color::GREEN,
            32 => Color::PURPLE,
            64 => Color::CYAN,
            128 => Color::PINK,
            _ => Color::WHITE,
        }
    }
    pub fn new_random(index: &usize) -> Block {
        Block::new(
            if thread_rng().gen_range(0..10) == 0 {
                4
            } else {
                2
            },
            index / 4,
            index % 4,
        )
    }
    pub fn new(value: usize, x: usize, y: usize) -> Self {
        Self(value, x, y)
    }
    pub fn set_number(&mut self, new: usize) {
        self.0 = new
    }
    pub fn number(&self) -> usize {
        self.0
    }
    pub fn x(&self) -> usize {
        self.1
    }
    pub fn y(&self) -> usize {
        self.2
    }
}
