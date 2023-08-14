mod components;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::{despawn_pause_menu, spawn_pause_menu};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Pause), spawn_pause_menu)
            .add_systems(OnExit(AppState::Pause), despawn_pause_menu);
    }
}
