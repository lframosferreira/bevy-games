mod resources;
mod systems;

use crate::AppState;
use bevy::prelude::*;
pub use resources::Score;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_systems(OnExit(AppState::GameOver), reset_score);
    }
}
