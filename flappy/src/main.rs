mod game;

use bevy::prelude::*;
use common::CommonPlugin;
use game::{GamePlugin, WINDOW_X, WINDOW_Y};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::CYAN))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Bevy Flappy".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CommonPlugin::default())
        .add_plugins(GamePlugin)
        .run()
}
