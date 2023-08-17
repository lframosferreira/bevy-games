mod components;
mod systems;

pub const WINDOW_X: f32 = 1000.;
pub const WINDOW_Y: f32 = 600.;

use bevy::prelude::*;
use common::AppState;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_blocks)
            .add_systems(Startup, spawn_ball)
            .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_ball.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (collide_ball_with_player, collide_ball_with_blocks)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
