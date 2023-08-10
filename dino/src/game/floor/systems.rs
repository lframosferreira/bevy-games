use super::components::Floor;
use super::resources::FloorEntitiesCount;
use super::FLOOR_HEIGHT;
use crate::game::obstacle::resources::ObstacleSpeed;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_floor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut floor_entities_count: ResMut<FloorEntitiesCount>,
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
    floor_entities_count.count = 1;
}

pub fn move_floor(
    mut commands: Commands,
    mut floor_query: Query<(&mut Transform, Entity), With<Floor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    obstacle_speed: Res<ObstacleSpeed>,
    mut floor_entities_count: ResMut<FloorEntitiesCount>,
) {
    let window: &Window = window_query.get_single().unwrap();
    println!("{:?}", -window.width() / 2.0);

    for (mut floor_transform, floor_entity) in floor_query.iter_mut() {
        floor_transform.translation.x -= obstacle_speed.speed * time.delta_seconds();
        println!("{:?}", floor_transform.translation);
        if floor_transform.translation.x < 0.0 && floor_entities_count.count < 2 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(window.width(), FLOOR_HEIGHT, 0.0),
                    texture: asset_server.load("sprites/floor.png"),
                    ..default()
                },
                Floor {},
            ));
            floor_entities_count.count += 1;
            if floor_transform.translation.x < (-window.width() / 2.0) {
                println!("oi");
                commands.entity(floor_entity).despawn();
                floor_entities_count.count -= 1;
            }
        }
    }
}
