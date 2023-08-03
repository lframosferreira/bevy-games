use bevy::prelude::*;

mod dinosaur;

use dinosaur::DinosaurPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DinosaurPlugin);
    }
}
