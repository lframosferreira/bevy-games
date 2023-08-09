use bevy::prelude::*;
use common::AppState;
use systems::*;

mod components;
mod systems;

const WINDOW_SIZE: f32 = 400.;
pub const WINDOW_X: f32 = WINDOW_SIZE;
pub const WINDOW_Y: f32 = WINDOW_SIZE;
pub const BLOCK_LENGTH: f32 = 100.;
pub const BLOCK_SIZE: Vec2 = Vec2::new(BLOCK_LENGTH, BLOCK_LENGTH);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks).add_systems(
            Update,
            (update_direction).run_if(in_state(AppState::InGame)),
        );
    }
}
