use crate::AppState;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use components::SnakeCounter;
use std::time::Duration;
use systems::*;

pub mod components;
mod systems;

pub const HEAD_X: f32 = 500.0;
pub const HEAD_Y: f32 = 300.0;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeCounter>()
            .add_systems(Startup, spawn_snake)
            .add_systems(Update, update_direction.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                sprite_movement
                    .run_if(in_state(AppState::InGame))
                    .run_if(on_timer(Duration::from_millis(100))),
            )
            .add_systems(Update, handle_eat_fruit.run_if(in_state(AppState::InGame)));
    }
}
