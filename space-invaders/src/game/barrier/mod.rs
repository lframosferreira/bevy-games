pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_barriers)
            .add_systems(OnExit(AppState::GameOver), respawn_barriers);
    }
}
