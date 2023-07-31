use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fruit);
    }
}
