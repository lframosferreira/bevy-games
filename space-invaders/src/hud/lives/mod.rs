mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct LivesHUDPlugin;

impl Plugin for LivesHUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lives_hud)
            .add_systems(Update, update_lives_hud.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::GameOver), despawn_live_hud)
            .add_systems(OnExit(AppState::GameOver), spawn_lives_hud);
    }
}
