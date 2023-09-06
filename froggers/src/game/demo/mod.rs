mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_timers, spawn_frog, spawn_scenario))
            .add_systems(OnExit(AppState::GameOver), respawn_frog)
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
