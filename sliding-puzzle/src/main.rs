mod game;

use bevy::prelude::*;
use common::CommonPlugin;
use game::GamePlugin;

pub const WINDOW_X: f32 = 400.0;
pub const WINDOW_Y: f32 = 400.0;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Bevy Sliding-Puzzle".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CommonPlugin::default())
        .add_plugins(GamePlugin)
        .run()
}
