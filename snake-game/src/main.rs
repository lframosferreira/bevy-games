use bevy::prelude::*;

pub mod events;
mod game;
mod systems;

use game::GamePlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1000.0, 600.0).into(),
                title: "Snake".to_string(),
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
