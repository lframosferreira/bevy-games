use bevy::prelude::Component;
use rand::{thread_rng, Rng};

#[derive(Component, Clone, Copy)]
pub struct Block(pub usize, usize, usize);

impl Block {
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
