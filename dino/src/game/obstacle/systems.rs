use bevy::prelude::*;

use crate::game::dinosaur::{DINO_INITIAL_Y_POS, DINO_X_POS};

use super::components::{Obstacle, ObstacleKind};
use super::resources::ObstacleSpawnTimer;

pub fn spawn_obstacle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.00, 0.75, 0.50),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(DINO_X_POS + 300.0, DINO_INITIAL_Y_POS, 0.0),
            ..default()
        },
        Obstacle {
            kind: ObstacleKind::Cactus,
            height: 300.0,
        },
    ));
}

pub fn spawn_obstacles_over_time(
    mut commands: Commands,
    obstacle_spawn_timer: Res<ObstacleSpawnTimer>,
) {
    println!("{}", obstacle_spawn_timer.timer.finished());
    if obstacle_spawn_timer.timer.finished() {
        println!("oi");
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(DINO_X_POS + 300.0, DINO_INITIAL_Y_POS, 0.0),
                ..default()
            },
            Obstacle {
                kind: ObstacleKind::Cactus,
                height: 300.0,
            },
        ));
    }
}
