use bevy::prelude::Resource;

#[derive(Resource, Default, Clone, Copy, Debug)]
pub enum AlienDirection {
    #[default]
    Right,
    Left,
}

impl AlienDirection {
    pub fn toggle(&mut self) {
        match self {
            AlienDirection::Right => *self = AlienDirection::Left,
            AlienDirection::Left => *self = AlienDirection::Right,
        };
    }
}

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
