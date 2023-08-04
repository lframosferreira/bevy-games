use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use self::resources::ObstacleSpawnTimer;
use self::systems::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimer>()
            .add_systems(Update, spawn_obstacles_over_time);
    }
}
