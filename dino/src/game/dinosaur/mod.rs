use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use self::resources::{DinoDown, DinoVerticalMovement};
use self::systems::*;
use common::AppState;

use super::floor::FLOOR_HEIGHT;

pub const DINO_HEIGHT: f32 = 84.0;
pub const DINO_DOWN_HEIGHT: f32 = 50.0;
pub const DINO_X_POS: f32 = 100.0;
pub const DINO_INITIAL_Y_POS: f32 = FLOOR_HEIGHT + DINO_HEIGHT / 2.0;
pub const DINO_DOWN_Y_POS: f32 = FLOOR_HEIGHT + DINO_DOWN_HEIGHT / 2.0;
pub const GRAVITY: f32 = -1.2;
pub const DINO_INITIAL_VERTICAL_SPEED: f32 = 25.0;

pub struct DinosaurPlugin;

impl Plugin for DinosaurPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DinoVerticalMovement>()
            .init_resource::<DinoDown>()
            .add_systems(Startup, spawn_dinosaur)
            .add_systems(
                Update,
                (handle_jump, handle_collision, dinosaur_down_movement)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
