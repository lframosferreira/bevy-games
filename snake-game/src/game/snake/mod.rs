use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub const SNAKE_BODY_PART_SIZE: f32 = 40.0;
pub const SNAKE_SPEED: f32 = 200.0;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, update_direction)
            .add_systems(Update, sprite_movement)
            .add_systems(Update, handle_eat_fruit);
    }
}
