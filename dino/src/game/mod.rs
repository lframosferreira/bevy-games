use bevy::prelude::*;

mod cloud;
pub mod dinosaur;
mod floor;
pub mod obstacle;
pub mod score;

use cloud::CloudPlugin;
use dinosaur::DinosaurPlugin;
use floor::FloorPlugin;
use obstacle::ObstaclePlugin;
use score::ScorePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DinosaurPlugin,
            FloorPlugin,
            ObstaclePlugin,
            ScorePlugin,
            CloudPlugin,
        ));
    }
}
