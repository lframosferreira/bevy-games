use bevy::prelude::*;
use common::*;

mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CloudSpawnTimer>().add_systems(
            Update,
            (
                tick_cloud_spawn_timer,
                spawn_clouds_over_time,
                despawn_clouds_over_time,
                move_clouds_over_time,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
