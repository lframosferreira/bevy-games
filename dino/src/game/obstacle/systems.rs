use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::OBSTACLE_SPEED;
use crate::game::dinosaur::DINO_INITIAL_Y_POS;

use super::components::{Obstacle, ObstacleKind};
use super::resources::ObstacleSpawnTimer;

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
) {
    if obstacle_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(window.width(), DINO_INITIAL_Y_POS, 0.0),
                ..default()
            },
            Obstacle {
                kind: ObstacleKind::Cactus,
                height: 300.0,
            },
        ));
    }
}

pub fn obstacles_movement(
    mut obstacle_query: Query<&mut Transform, With<Obstacle>>,
    time: Res<Time>,
) {
    for mut obstacle_transform in obstacle_query.iter_mut() {
        obstacle_transform.translation.x -= OBSTACLE_SPEED * time.delta_seconds();
    }
}
