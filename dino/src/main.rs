mod game;
mod systems;

use game::GamePlugin;

use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}
