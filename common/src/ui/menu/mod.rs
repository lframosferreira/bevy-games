mod components;
mod game_over;
mod interactions;
mod layout;
mod styles;

use crate::AppState;
use bevy::prelude::*;
use bevy::prelude::{BackgroundColor, Interaction};
use game_over::GameOverMenuPlugin;
use interactions::interact_with_quit_button;

type InteractionBackground<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameOverMenuPlugin).add_systems(
            Update,
            interact_with_quit_button
                .run_if(in_state(AppState::Pause).or_else(in_state(AppState::GameOver))),
        );
    }
}
