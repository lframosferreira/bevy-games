use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub const FLOOR_HEIGHT: f32 = 150.0;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}
