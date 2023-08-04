mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub const BLOCK_SIZE: f32 = 30.0;
pub const MAX_LIVES: usize = 20;
const SIZE: Vec2 = Vec2::new(BLOCK_SIZE, BLOCK_SIZE);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_life_bar, spawn_blocks, spawn_buttons).chain(),
        )
        .add_systems(
            OnExit(AppState::GameOver),
            (respawn_life_bar, respawn_blocks),
        )
        .add_systems(
            Update,
            (update_color, take_life).run_if(in_state(AppState::InGame)),
        );
    }
}
