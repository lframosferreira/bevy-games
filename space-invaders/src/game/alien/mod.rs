pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub const ALIEN_POINTS: usize = 10;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_aliens)
            .add_systems(Update, move_aliens.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::GameOver), respawn_aliens);
    }
}
