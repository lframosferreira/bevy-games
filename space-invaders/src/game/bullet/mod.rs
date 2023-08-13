pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bullets, move_bullets).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::GameOver), despawn_bullets);
    }
}
