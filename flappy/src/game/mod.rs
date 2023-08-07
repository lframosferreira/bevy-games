mod components;
mod resources;
mod systems;

use bevy::{prelude::*, time::common_conditions::on_timer};
use common::AppState;
use resources::{Gravity, Score};
use std::time::Duration;
use systems::*;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 800.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .init_resource::<Score>()
            .add_systems(
                Update,
                spawn_pipe
                    .run_if(in_state(AppState::InGame))
                    .run_if(on_timer(Duration::from_secs(2))),
            )
            .init_resource::<Gravity>()
            .add_systems(
                Update,
                (move_bird, move_pipe, check_collision).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::GameOver),
                (respawn_bird, despawn_pipes, reset_gravity, reset_score),
            );
    }
}
