pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub const BOSS_POINTS: usize = 1000;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_boss, move_boss).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::GameOver), despawn_boss);
    }
}
