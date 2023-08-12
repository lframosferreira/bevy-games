pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub const FLOOR_HEIGHT: f32 = 150.0;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor)
            .add_systems(Update, move_floor.run_if(in_state(AppState::InGame)));
    }
}
