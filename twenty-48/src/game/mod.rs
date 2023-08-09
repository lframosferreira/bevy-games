mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Score;
use systems::*;

const WINDOW_SIZE: f32 = 400.;
pub const WINDOW_X: f32 = WINDOW_SIZE;
pub const WINDOW_Y: f32 = WINDOW_SIZE;
pub const BLOCK_LENGTH: f32 = 100.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, spawn_blocks)
            .add_systems(
                Update,
                (update_direction).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::GameOver), (respawn_blocks, reset_score));
    }
}
