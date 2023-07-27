use bevy::prelude::*;

mod fruit;
mod snake;
mod systems;

use fruit::FruitPlugin;
use snake::SnakePlugin;

use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins((SnakePlugin, FruitPlugin));
    }
}