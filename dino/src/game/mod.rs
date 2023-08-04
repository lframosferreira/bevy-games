use bevy::prelude::*;

mod dinosaur;
mod floor;
mod obstacle;

use dinosaur::DinosaurPlugin;
use floor::FloorPlugin;
use obstacle::ObstaclePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DinosaurPlugin, FloorPlugin, ObstaclePlugin));
    }
}
