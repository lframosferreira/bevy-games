pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub const FLOOR_HEIGHT: f32 = 150.0;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FloorEntitiesCount>()
            .add_systems(Startup, spawn_floor)
            .add_systems(Update, move_floor);
    }
}
