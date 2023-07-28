use bevy::prelude::*;

pub mod resources;
mod systems;

use self::resources::*;
use self::systems::*;
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score)
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)));
    }
}
