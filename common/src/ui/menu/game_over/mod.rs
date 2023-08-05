mod components;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::{despawn_game_over_menu, spawn_game_over_menu};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
