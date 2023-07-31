use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub mod components;
mod systems;

use self::systems::*;

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
