use bevy::prelude::Resource;

pub const LIVES: usize = 3;

#[derive(Resource)]
pub struct Lives(usize);

impl Default for Lives {
    fn default() -> Self {
        Self(LIVES)
    }
}

impl Lives {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn reset(&mut self) {
        self.0 = LIVES
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
