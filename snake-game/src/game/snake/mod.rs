use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub const SNAKE_BODY_PART_SIZE: f32 = 50.0;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake);
    }
}
