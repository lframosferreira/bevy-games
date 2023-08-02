mod components;
mod game_over;
mod pause;
mod styles;

use bevy::prelude::*;
use {game_over::GameOverMenuPlugin, pause::PauseMenuPlugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseMenuPlugin)
            .add_plugins(GameOverMenuPlugin);
    }
}
