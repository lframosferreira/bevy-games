pub mod events;
pub mod systems;
pub mod ui;

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Pause,
    GameOver,
}
