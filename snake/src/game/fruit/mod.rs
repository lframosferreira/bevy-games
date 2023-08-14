pub mod components;
pub mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fruit)
            .add_systems(OnExit(AppState::GameOver), respawn_fruit);
    }
}
