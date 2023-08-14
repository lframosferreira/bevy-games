mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct ScoreHUDPlugin;

impl Plugin for ScoreHUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_hud)
            .add_systems(Update, update_score_hud.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::GameOver), despawn_score_hud)
            .add_systems(OnExit(AppState::GameOver), spawn_score_hud);
    }
}
