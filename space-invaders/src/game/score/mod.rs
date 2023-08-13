pub mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Score;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnExit(AppState::GameOver), reset_score);
    }
}
