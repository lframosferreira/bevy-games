use bevy::prelude::*;

pub const OBSTACLE_SPEED: f32 = 300.0;

pub mod components;
mod resources;
mod systems;

use self::resources::ObstacleSpawnTimer;
use self::systems::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimer>().add_systems(
            Update,
            (
                tick_obstacle_spawn_timer,
                spawn_obstacles_over_time,
                obstacles_movement,
            ),
        );
    }
}
