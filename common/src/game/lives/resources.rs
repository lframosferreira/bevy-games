use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Lives(usize);

impl Lives {
    pub fn new(lives: usize) -> Self {
        Lives(lives)
    }
    pub fn get(&self) -> usize {
        self.0
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

impl Default for Lives {
    fn default() -> Self {
        Lives(3)
    }
}
