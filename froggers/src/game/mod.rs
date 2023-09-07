mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::events::EndGame;
use common::game::LivesPlugin;
use common::AppState;
use systems::*;

pub const BLOCK_LENGTH: f32 = 50.0;
pub const WINDOW_X: f32 = 600.;
pub const WINDOW_Y: f32 = 650.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>()
            .add_plugins(LivesPlugin)
            .add_systems(Startup, (init_timers, spawn_frog, spawn_scenario))
            .add_systems(
                OnExit(AppState::GameOver),
                (respawn_frog, despawn_left_over_frogs, despawn_vehicles),
            )
            .add_systems(
                Update,
                (
                    move_frog,
                    move_vehicles,
                    tick_timers,
                    spawn_vehicles,
                    collide,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
