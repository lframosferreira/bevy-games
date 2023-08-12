use bevy::prelude::Resource;

use super::systems::PLAYER_HP;

#[derive(Resource, Default)]
pub struct Score(pub usize);

impl Score {
    pub fn score(&self) -> usize {
        self.0
    }
    pub fn increment(&mut self, delta: usize) {
        self.0 += delta
    }
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(Resource)]
pub struct Lives(usize);

impl Default for Lives {
    fn default() -> Self {
        Self(PLAYER_HP)
    }
}

impl Lives {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn reset(&mut self) {
        self.0 = PLAYER_HP
    }
    pub fn increment(&mut self) {
        self.0 += 1
    }
    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1
        }
    }
    pub fn zero(&mut self) {
        self.0 = 0
    }
}
