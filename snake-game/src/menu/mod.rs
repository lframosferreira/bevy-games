mod components;
mod game_over;
mod interactions;
mod layout;
mod pause;
mod styles;

type WorldQuery<'a> = (&'a Interaction, &'a mut BackgroundColor);

use crate::AppState;
use bevy::prelude::*;
use interactions::interact_with_quit_button;
use {game_over::GameOverMenuPlugin, pause::PauseMenuPlugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseMenuPlugin)
            .add_plugins(GameOverMenuPlugin)
            .add_systems(
                Update,
                interact_with_quit_button
                    .run_if(in_state(AppState::Pause).or_else(in_state(AppState::GameOver))),
            );
    }
}
