mod game;
mod systems;

use common::CommonPlugin;
use game::GamePlugin;
use systems::*;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use common::*;

pub const WINDOW_X: f32 = 1000.0;
pub const WINDOW_Y: f32 = 600.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_X, WINDOW_Y).into(),
                title: "Chrome Dinosaur".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins((GamePlugin, CommonPlugin))
        .add_systems(
            OnExit(AppState::GameOver),
            (reset_score, reset_obstacle_speed, despawn_obstacles, set_dinosaur_in_initial_position),
        )
        .run();
}