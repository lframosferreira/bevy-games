use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use self::resources::DinoVerticalMovement;
use self::systems::*;

use super::floor::FLOOR_HEIGHT;

pub const DINO_WIDTH: f32 = 40.0;
pub const DINO_HEIGHT: f32 = 80.0;
pub const DINO_X_POS: f32 = 100.0;
pub const DINO_INITIAL_Y_POS: f32 = FLOOR_HEIGHT + DINO_HEIGHT / 2.0;
pub const GRAVITY: f32 = -1.5;
pub const DINO_INITIAL_VERTICAL_SPEED: f32 = 20.0;

pub struct DinosaurPlugin;

impl Plugin for DinosaurPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DinoVerticalMovement>()
            .add_systems(Startup, spawn_dinosaur)
            .add_systems(
                Update,
                (handle_jump, handle_collision, dinosaur_down_movement),
            );
    }
}
