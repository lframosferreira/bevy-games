use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::game::obstacle::components::Obstacle;
use crate::game::score::resources::Score;
use common::events::EndGame;
use common::AppState;

use super::components::Dinosaur;
use super::resources::DinoVerticalMovement;
use super::{DINO_HEIGHT, DINO_INITIAL_VERTICAL_SPEED, DINO_INITIAL_Y_POS, DINO_X_POS, GRAVITY};

pub fn spawn_dinosaur(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(DINO_X_POS, DINO_INITIAL_Y_POS, 0.0),
            texture: asset_server.load("sprites/dino/dino_1.png"),
            ..default()
        },
        Dinosaur {},
    ));
}

pub fn handle_jump(
    mut dinosaur_query: Query<&mut Transform, With<Dinosaur>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dino_vertical_movement: ResMut<DinoVerticalMovement>,
) {
    if let Ok(mut dinosaur_transform) = dinosaur_query.get_single_mut() {
        if dino_vertical_movement.moving {
            if dinosaur_transform.translation.y < DINO_INITIAL_Y_POS {
                dino_vertical_movement.moving = false;
                dino_vertical_movement.speed = DINO_INITIAL_VERTICAL_SPEED;
                dinosaur_transform.translation.y = DINO_INITIAL_Y_POS;
            } else {
                dinosaur_transform.translation.y += dino_vertical_movement.speed;
                dino_vertical_movement.speed += GRAVITY;
            }
        } else {
            if keyboard_input.just_pressed(KeyCode::Space) {
                dino_vertical_movement.moving = true;
                dinosaur_transform.translation.y += dino_vertical_movement.speed;
                dino_vertical_movement.speed += GRAVITY;
            }
        }
    }
}

pub fn handle_collision(
    mut commands: Commands,
    dinosaur_query: Query<&Transform, With<Dinosaur>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    if let Ok(dinosaur_transform) = dinosaur_query.get_single() {
        for obstacle_transform in obstacle_query.iter() {
            if collide(
                dinosaur_transform.translation,
                Vec2::new(88.0, 94.0),
                obstacle_transform.translation,
                Vec2::new(92., 80.),
            )
            .is_some()
            {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame {
                    score: score.value as usize,
                });
                return;
            }
        }
    }
}

pub fn dinosaur_down_movement(
    mut dinosaur_query: Query<&mut Transform, With<Dinosaur>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut dinosaur_transform) = dinosaur_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Down) {
            dinosaur_transform.translation.y = DINO_INITIAL_Y_POS - DINO_HEIGHT / 2.0;
        } else if keyboard_input.just_released(KeyCode::Down) {
            dinosaur_transform.translation.y = DINO_INITIAL_Y_POS;
        }
    }
}
