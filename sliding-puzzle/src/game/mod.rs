use bevy::prelude::*;
use common::events::EndGame;

pub const BLOCK_SIZE: f32 = 40.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>();
    }
}
