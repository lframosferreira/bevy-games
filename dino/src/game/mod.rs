use bevy::prelude::*;

mod dinosaur;
mod floor;

use dinosaur::DinosaurPlugin;
use floor::FloorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DinosaurPlugin, FloorPlugin));
    }
}
