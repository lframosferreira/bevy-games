use super::components::{Obstacle, ObstacleKind};
use super::resources::{ObstacleSpawnTimer, ObstacleSpeed};
use super::{OBSTACLE_INITIAL_SPEED, OBSTACLE_SPEED_INCREASE_RATE};
use crate::game::dinosaur::DINO_INITIAL_Y_POS;
use crate::game::score::resources::Score;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub fn tick_obstacle_spawn_timer(
    mut obstacle_spawn_timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
) {
    obstacle_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_obstacles_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    obstacle_spawn_timer: Res<ObstacleSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if obstacle_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();
        let obstacle_kind: ObstacleKind = rand::random();
        let amount: u32 = rand::thread_rng().gen_range(1..4);
        let mut rng = rand::thread_rng();

        /* No caso de CactusSmall, um pequeno offset é utilizado no y pro cacto ficar no chão e não flutuando */
        let sprite_bundle = match obstacle_kind {
            ObstacleKind::CactusSmall => SpriteBundle {
                transform: Transform::from_xyz(window.width(), DINO_INITIAL_Y_POS - 10.0, 0.0),
                texture: asset_server.load(format!("sprites/cacti/cacti_small_{}.png", amount)),
                ..default()
            },
            ObstacleKind::CactusLarge => SpriteBundle {
                transform: Transform::from_xyz(window.width(), DINO_INITIAL_Y_POS, 0.0),
                texture: asset_server.load(format!("sprites/cacti/cacti_large_{}.png", amount)),
                ..default()
            },
            ObstacleKind::Pterodactyl => SpriteBundle {
                transform: Transform::from_xyz(
                    window.width(),
                    DINO_INITIAL_Y_POS + 100.0 * rng.gen_range(0.0..1.0),
                    0.0,
                ),
                texture: asset_server.load("sprites/ptera/ptera_1.png"),
                ..default()
            },
        };
        commands.spawn((
            sprite_bundle,
            Obstacle {
                kind: obstacle_kind,
            },
        ));
    }
}

pub fn despawn_obstacles_out_of_screen(
    mut commands: Commands,
    mut obstacle_query: Query<(&Handle<Image>, &Transform, Entity), With<Obstacle>>,
    assets: Res<Assets<Image>>,
) {
    for (obstacle_handle_image, obstacle_transform, obstacle_entity) in obstacle_query.iter_mut() {
        if let Some(obstacle_image) = assets.get(obstacle_handle_image) {
            let obstacle_width = obstacle_image.size().x;
            if obstacle_transform.translation.x < -obstacle_width / 2.0 {
                commands.entity(obstacle_entity).despawn();
            }
        }
    }
}

pub fn obstacles_movement(
    mut obstacle_query: Query<&mut Transform, With<Obstacle>>,
    time: Res<Time>,
    obstacle_speed: Res<ObstacleSpeed>,
) {
    for mut obstacle_transform in obstacle_query.iter_mut() {
        obstacle_transform.translation.x -= obstacle_speed.speed * time.delta_seconds();
    }
}

pub fn set_obstacle_speed(score: Res<Score>, mut obstacle_speed: ResMut<ObstacleSpeed>) {
    obstacle_speed.speed =
        OBSTACLE_INITIAL_SPEED + OBSTACLE_SPEED_INCREASE_RATE * ((score.value / 500) as f32)
}

pub fn reset_obstacle_speed(mut obstacle_speed: ResMut<ObstacleSpeed>) {
    obstacle_speed.speed = OBSTACLE_INITIAL_SPEED;
}

pub fn despawn_obstacles(
    mut commands: Commands,
    mut obstacle_query: Query<Entity, With<Obstacle>>,
) {
    for obstacle_entity in obstacle_query.iter_mut() {
        commands.entity(obstacle_entity).despawn();
    }
}
