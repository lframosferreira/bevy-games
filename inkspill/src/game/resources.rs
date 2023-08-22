use super::MAX_LIVES;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Lives(pub usize);

impl Default for Lives {
    fn default() -> Self {
        Lives(MAX_LIVES)
    }
}

impl Lives {
    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1
        }
    }
}
