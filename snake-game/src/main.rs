pub mod events;
mod game;
mod menu;
mod systems;

use bevy::prelude::*;
use game::{
    snake::{HEAD_X, HEAD_Y},
    GamePlugin,
};
use menu::MenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (HEAD_X * 2., HEAD_Y * 2.).into(),
                title: "Bevy Snake".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (pause_game, resume_game, handle_game_over))
        .run()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Pause,
    GameOver,
}
