mod components;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::interactions::interact_with_resume_button;
use systems::layout::{despawn_pause_menu, spawn_pause_menu};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Pause), spawn_pause_menu)
            .add_systems(
                Update,
                interact_with_resume_button.run_if(in_state(AppState::Pause)),
            )
            .add_systems(OnExit(AppState::Pause), despawn_pause_menu);
    }
}
