use bevy::prelude::*;

#[derive(Component)]
pub struct Fruit {
    pub x_pos: f32,
    pub y_pos: f32,
}

impl Default for Fruit {
    fn default() -> Self {
        Fruit {
            x_pos: 0.0,
            y_pos: 0.0,
        }
    }
}
