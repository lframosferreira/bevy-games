mod game;
mod systems;

use common::CommonPlugin;
use game::GamePlugin;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Chrome Dinosaur".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins((GamePlugin, CommonPlugin::new_light()))
        .add_systems(Update, (pause_game, resume_game, death_sound_effect))
        .run();
}
