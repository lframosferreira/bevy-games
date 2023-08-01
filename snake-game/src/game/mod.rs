mod fruit;
mod score;
pub mod snake;
mod systems;

use crate::events::GameOver;
use bevy::prelude::*;
use fruit::FruitPlugin;
use score::ScorePlugin;
use snake::SnakePlugin;

pub const BLOCK_SIZE: f32 = 40.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins((SnakePlugin, FruitPlugin, ScorePlugin));
    }
}
