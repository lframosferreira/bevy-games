use bevy::prelude::*;

pub mod components;
mod systems;

use self::systems::*;

pub const FRUIT_SIZE: f32 = 40.0;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fruit);
    }
}
