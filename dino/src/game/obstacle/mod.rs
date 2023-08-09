use bevy::prelude::*;
use common::*;

pub const OBSTACLE_INITIAL_SPEED: f32 = 600.0;
pub const OBSTACLE_SPEED_INCREASE_RATE: f32 = 100.0;

pub mod components;
pub mod resources;
pub mod systems;

use self::resources::{ObstacleSpawnTimer, ObstacleSpeed};
use self::systems::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimer>()
            .init_resource::<ObstacleSpeed>()
            .add_systems(
                Update,
                (
                    tick_obstacle_spawn_timer,
                    spawn_obstacles_over_time,
                    obstacles_movement,
                    despawn_obstacles_out_of_screen,
                    set_obstacle_speed,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnExit(AppState::GameOver),
                (reset_obstacle_speed, despawn_obstacles),
            );
    }
}
