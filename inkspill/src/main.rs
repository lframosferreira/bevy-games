mod game;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use common::CommonPlugin;
use game::{GamePlugin, WINDOW_X, WINDOW_Y};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Bevy Inkspill".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            DebugLinesPlugin::default(),
            CommonPlugin::new_unpausable(),
            GamePlugin,
        ))
        .run()
}
