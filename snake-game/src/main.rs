pub mod events;
mod game;
mod systems;

use bevy::prelude::*;
use game::{
    snake::{HEAD_X, HEAD_Y},
    GamePlugin,
};
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
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
