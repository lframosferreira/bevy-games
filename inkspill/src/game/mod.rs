mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Lives;
use systems::*;

pub const BLOCK_SIZE: f32 = 30.0;
pub const MAX_LIVES: usize = 20;
const SIZE: Vec2 = Vec2::new(BLOCK_SIZE, BLOCK_SIZE);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lives>()
            .add_systems(Startup, (spawn_life_bar, spawn_blocks, spawn_buttons))
            .add_systems(OnExit(AppState::GameOver), (spawn_life_bar, respawn_blocks))
            .add_systems(
                Update,
                (update_color, take_life, check_win).run_if(in_state(AppState::InGame)),
            );
    }
}
