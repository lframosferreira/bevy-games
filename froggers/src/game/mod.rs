mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use common::events::EndGame;
use common::game::LivesPlugin;
use common::AppState;
use resources::*;
use systems::*;

pub const BLOCK_LENGTH: f32 = 50.0;
pub const WINDOW_X: f32 = 600.;
pub const WINDOW_Y: f32 = 700.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>()
            .add_plugins(LivesPlugin)
            .insert_resource(MaxHeight::default())
            .insert_resource(HitLeftOverFrogs::default())
            .insert_resource(HitHaven::default())
            .add_systems(
                Startup,
                (init_timers, spawn_frog, spawn_scenario, spawn_timer),
            )
            .add_systems(
                OnExit(AppState::GameOver),
                (
                    respawn_frog,
                    spawn_timer,
                    despawn_left_over_frogs,
                    reset_height,
                ),
            )
            .add_systems(
                Update,
                (
                    move_frog,
                    move_vehicles,
                    reset_left_over,
                    scale_timer,
                    tick_timers,
                    track_lives,
                    spawn_vehicles,
                    collide_frog_with_vehicles,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (
                    collide_frog_with_frogs,
                    collide_frog_with_haven,
                    collide_frog_with_lake,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
