pub mod components;
mod systems;

use bevy::prelude::*;
use common::AppState;
use components::Player;
use systems::*;

pub const PLAYER_Y_OFFSET: f32 = 100.;
pub fn player_sprite() -> Sprite {
    let player = Player::default();
    Sprite {
        color: player.stats.color(),
        custom_size: Some(player.stats.size()),
        ..default()
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::GameOver), respawn_player);
    }
}
