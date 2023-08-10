use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::obstacle::resources::ObstacleSpeed;

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

pub fn spawn_floor_when_needed(mut commands: Commands, floor_query: Query<&Transform, With<Floor>>){
    for floor_transform in floor_query.iter() {
        if floor_transform.translation.x > 0.0 {}
    }
}

pub fn move_floor(
    mut floor_query: Query<&mut Transform, With<Floor>>,
    time: Res<Time>,
    obstacle_speed: Res<ObstacleSpeed>,
) {
    for mut floor_transform in floor_query.iter_mut() {
        floor_transform.translation.x -= obstacle_speed.speed * time.delta_seconds();
    }
}
