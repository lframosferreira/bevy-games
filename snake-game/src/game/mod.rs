use bevy::prelude::*;

mod fruit;
mod score;
mod snake;
mod systems;

use fruit::FruitPlugin;
use score::ScorePlugin;
use snake::SnakePlugin;

use crate::events::GameOver;

const BLOCK_SIZE: f32 = 40.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins((SnakePlugin, FruitPlugin, ScorePlugin));
    }
}
