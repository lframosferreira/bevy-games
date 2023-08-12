mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::AppState;
use resources::Lives;
use resources::Score;
use systems::*;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 800.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<Lives>()
            .add_systems(
                Startup,
                (
                    spawn_player,
                    spawn_aliens,
                    spawn_barriers,
                    spawn_score,
                    spawn_lives_hud,
                ),
            )
            .add_systems(
                Update,
                (
                    update_score,
                    update_lives,
                    spawn_boss,
                    spawn_bullets,
                    spawn_lasers,
                    move_player,
                    move_bullets,
                    move_aliens,
                    move_lasers,
                    move_boss,
                    collide_bullets_with_aliens,
                    collide_lasers_with_player,
                    collide_projectiles_with_barriers,
                    collide_bullets_with_boss,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::GameOver),
                (
                    despawn_bullets,
                    despawn_lasers,
                    despawn_boss,
                    reset_score,
                    reset_lives,
                    respawn_aliens,
                    respawn_barriers,
                    respawn_player,
                    respawn_live_hud,
                ),
            );
    }
}
