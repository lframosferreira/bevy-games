mod resources;
mod systems;

use bevy::prelude::*;
use common::events::EndGame;
use common::*;
use resources::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridStatus>()
            .add_event::<EndGame>()
            .add_systems(Startup, spawn_blocks)
            .add_systems(Update, handle_movement)
            .add_systems(OnExit(AppState::GameOver), reset_grid_status);
    }
}
