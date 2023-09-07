mod fruit;
mod snake;

use bevy::prelude::*;
use common::{events::EndGame, game::ScorePlugin};
use fruit::FruitPlugin;
use snake::SnakePlugin;

pub const BLOCK_SIZE: f32 = 40.0;
const SIZE: Vec2 = Vec2::new(BLOCK_SIZE, BLOCK_SIZE);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>()
            .add_plugins((SnakePlugin, FruitPlugin, ScorePlugin));
    }
}
