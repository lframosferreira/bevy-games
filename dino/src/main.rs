mod game;

use bevy::prelude::*;
use common::CommonPlugin;
use game::GamePlugin;

const WINDOW_X: f32 = 1000.0;
const WINDOW_Y: f32 = 600.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Bevy Dinosaur".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GamePlugin, CommonPlugin))
        .run();
}
