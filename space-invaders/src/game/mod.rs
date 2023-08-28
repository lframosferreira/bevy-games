mod alien;
mod barrier;
mod boss;
mod bullet;
mod components;
mod laser;
mod player;
mod systems;

use alien::AlienPlugin;
use barrier::BarrierPlugin;
use bevy::prelude::*;
use boss::BossPlugin;
use bullet::BulletPlugin;
use common::{
    game::{Lives, LivesPlugin},
    AppState,
};
use laser::LaserPlugin;
pub use player::player_sprite;
use player::PlayerPlugin;
use systems::*;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 800.0;
pub const MAX_LIVES: usize = 3;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives::new(MAX_LIVES))
            .add_systems(
                Update,
                (
                    collide_bullets_with_aliens,
                    collide_lasers_with_player,
                    collide_projectiles_with_barriers,
                    collide_bullets_with_boss,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::GameOver), reset_lives)
            .add_plugins((
                AlienPlugin,
                LaserPlugin,
                BossPlugin,
                PlayerPlugin,
                BarrierPlugin,
                BulletPlugin,
                LivesPlugin,
            ));
    }
}
