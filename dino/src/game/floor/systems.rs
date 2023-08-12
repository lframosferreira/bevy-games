use super::components::Floor;
use super::FLOOR_HEIGHT;
use crate::game::obstacle::resources::ObstacleSpeed;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_floor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width(), FLOOR_HEIGHT, 0.0),
            texture: asset_server.load("sprites/floor.png"),
            ..default()
        },
        Floor {},
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() * 3.0, FLOOR_HEIGHT, 0.0),
            texture: asset_server.load("sprites/floor.png"),
            ..default()
        },
        Floor {},
    ));
}

pub fn move_floor(
    mut commands: Commands,
    mut floor_query: Query<(&mut Transform, Entity), With<Floor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    obstacle_speed: Res<ObstacleSpeed>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for (mut floor_transform, floor_entity) in floor_query.iter_mut() {
        floor_transform.translation.x -= obstacle_speed.speed * time.delta_seconds();
        if floor_transform.translation.x < -window.width() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(window.width() * 3.0, FLOOR_HEIGHT, 0.0),
                    texture: asset_server.load("sprites/floor.png"),
                    ..default()
                },
                Floor {},
            ));
            commands.entity(floor_entity).despawn();
        }
    }
}
