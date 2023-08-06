mod game;
mod systems;

use common::CommonPlugin;
use game::GamePlugin;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Chrome Dinosaur".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GamePlugin, CommonPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}
