mod game;
mod systems;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use common::events::EndGame;
use common::ui::menu::MenuPlugin;
use common::{systems::spawn_camera, AppState};
use game::GamePlugin;
use systems::{draw_grid, WINDOW_X, WINDOW_Y};

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
        .add_state::<AppState>()
        .add_event::<EndGame>()
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, draw_grid)
        .run()
}
