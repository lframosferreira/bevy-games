mod resources;
mod systems;

use super::ScorePlugin;
use crate::AppState;
use bevy::prelude::*;
pub use resources::Lives;
use systems::*;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScorePlugin)
            .insert_resource(Lives::default())
            .add_systems(Update, watch_death.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::GameOver), bring_back_to_life);
    }
}
