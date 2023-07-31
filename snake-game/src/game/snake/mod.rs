use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub mod components;
mod systems;

use self::systems::*;

pub const SNAKE_BODY_PART_SIZE: f32 = 40.0;
pub const SNAKE_SPEED: f32 = 40.0;

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
