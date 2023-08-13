mod alien;
mod barrier;
mod boss;
mod bullet;
mod components;
mod laser;
mod lives;
mod player;
mod score;
mod systems;

use alien::AlienPlugin;
use barrier::BarrierPlugin;
use bevy::prelude::*;
use boss::BossPlugin;
use bullet::BulletPlugin;
use common::AppState;
use laser::LaserPlugin;
pub use lives::resources::Lives;
use lives::LivesPlugin;
pub use lives::LIVES;
pub use player::player_sprite;
use player::PlayerPlugin;
pub use score::resources::Score;
use score::ScorePlugin;
use systems::*;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 800.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                collide_bullets_with_aliens,
                collide_lasers_with_player,
                collide_projectiles_with_barriers,
                collide_bullets_with_boss,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_plugins((
            AlienPlugin,
            LaserPlugin,
            BossPlugin,
            PlayerPlugin,
            BarrierPlugin,
            BulletPlugin,
            ScorePlugin,
            LivesPlugin,
        ));
    }
}
