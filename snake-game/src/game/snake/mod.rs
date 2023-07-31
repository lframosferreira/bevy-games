use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use systems::*;

pub mod components;
mod systems;

pub const HEAD_X: f32 = 500.0;
pub const HEAD_Y: f32 = 300.0;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
            .add_systems(Update, update_direction)
            .add_systems(
                Update,
                sprite_movement.run_if(on_timer(Duration::from_millis(100))),
            )
            .add_systems(Update, handle_eat_fruit);
    }
}
