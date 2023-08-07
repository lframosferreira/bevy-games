use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Floor;
use super::FLOOR_HEIGHT;

pub fn spawn_floor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., FLOOR_HEIGHT, 0.0),
            texture: asset_server.load("sprites/floor.png"),
            ..default()
        },
        Floor {},
    ));
}
