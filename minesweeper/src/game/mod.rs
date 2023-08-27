mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub const WINDOW_X: f32 = 600.;
pub const WINDOW_Y: f32 = WINDOW_X;
const NUM_BOMBS: usize = 9;
const GRID_SIZE: usize = 10;
const BOMB: usize = 9;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::default())
            .insert_resource(Visited::default())
            .add_systems(Startup, (spawn_grid, build_overlay))
            .add_systems(Update, (interact_with_button, reveal));
    }
}
