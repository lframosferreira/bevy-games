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
