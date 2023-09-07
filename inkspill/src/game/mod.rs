mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Lives;
use systems::*;

pub const BLOCK_SIZE: f32 = 30.0;
pub const MAX_LIVES: usize = 20;
pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 660.0;
const SIZE: Vec2 = Vec2::new(BLOCK_SIZE, BLOCK_SIZE);
const COLORS_NORMAL: [Color; 6] = [
    Color::CRIMSON,
    Color::TOMATO,
    Color::VIOLET,
    Color::SEA_GREEN,
    Color::YELLOW_GREEN,
    Color::INDIGO,
];

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lives>()
            .add_systems(Startup, (spawn_life_bar, spawn_blocks, spawn_buttons))
            .add_systems(OnEnter(AppState::GameOver), despawn_buttons)
            .add_systems(
                OnExit(AppState::GameOver),
                (spawn_life_bar, respawn_blocks, spawn_buttons),
            )
            .add_systems(
                Update,
                (take_life, check_win, interact_with_buttons).run_if(in_state(AppState::InGame)),
            );
    }
}
