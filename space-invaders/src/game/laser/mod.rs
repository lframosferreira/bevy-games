pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_lasers, move_lasers).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::GameOver), despawn_lasers);
    }
}
