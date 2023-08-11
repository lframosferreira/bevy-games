mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Score;
use systems::*;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 800.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, (spawn_player, spawn_aliens))
            .add_systems(
                Update,
                (
                    spawn_bullets,
                    spawn_lasers,
                    move_player,
                    move_bullets,
                    move_aliens,
                    move_lasers,
                    collide_bullets_with_aliens,
                    collide_lasers_with_player,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::GameOver),
                (
                    despawn_bullets,
                    despawn_lasers,
                    reset_score,
                    respawn_aliens,
                    respawn_player,
                ),
            );
    }
}
