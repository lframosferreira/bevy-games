pub mod resources;
mod systems;

use bevy::prelude::*;
use common::{game::ScorePlugin, AppState};
use resources::*;
use systems::*;

pub struct ScoreWithTimerPlugin;

impl Plugin for ScoreWithTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScorePlugin)
            .init_resource::<ScoreUpdateTimer>()
            .add_systems(Startup, spawn_score_text)
            .add_systems(
                Update,
                (tick_score_update_timer, update_score, update_score_text)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
