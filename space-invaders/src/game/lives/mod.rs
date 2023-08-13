pub mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Lives;
use systems::*;

pub const LIVES: usize = 3;

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lives>()
            .add_systems(Update, watch_death.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::GameOver), reset_lives);
    }
}
