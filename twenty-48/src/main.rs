mod game;
mod system;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use common::CommonPlugin;
use game::{GamePlugin, WINDOW_X, WINDOW_Y};
use system::draw_grid;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Bevy 2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins(CommonPlugin::default())
        .add_plugins(GamePlugin)
        .add_systems(Update, draw_grid)
        .run()
}
