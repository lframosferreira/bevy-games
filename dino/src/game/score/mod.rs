pub mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<ScoreUpdateTimer>()
            .add_systems(Startup, spawn_score_text)
            .add_systems(
                Update,
                (tick_score_update_timer, update_score, update_score_text)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), insert_score)
            .add_systems(OnExit(AppState::GameOver), reset_score);
    }
}
