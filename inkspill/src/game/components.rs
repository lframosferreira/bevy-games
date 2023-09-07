use super::COLORS_NORMAL;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub struct Block(pub Color, pub usize, pub usize);

impl Block {
    pub fn new(x: usize, y: usize) -> Self {
        let index = rand::thread_rng().gen_range(0..6);
        let color = COLORS_NORMAL[index];
        Block(color, x, y)
    }
}

#[derive(Component)]
pub struct Heart();

#[derive(Component)]
pub struct ColorIndexer(pub usize);

#[derive(Component)]
pub struct Buttons;
