mod demo;

use bevy::prelude::*;
use common::{events::EndGame, game::ScorePlugin};
use demo::DemoPlugin;

pub const BLOCK_LENGTH: f32 = 50.0;
pub const WINDOW_X: f32 = 600.;
pub const WINDOW_Y: f32 = 650.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>()
            .add_plugins((DemoPlugin, ScorePlugin));
    }
}
