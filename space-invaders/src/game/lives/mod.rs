pub mod resources;
mod systems;

pub struct LivesPlugin;

use bevy::prelude::*;
use common::AppState;
use resources::Lives;
use systems::*;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lives>()
            .add_systems(OnExit(AppState::GameOver), reset_lives);
    }
}
