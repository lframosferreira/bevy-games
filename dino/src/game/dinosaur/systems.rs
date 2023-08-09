use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::game::obstacle::components::Obstacle;
use crate::game::score::resources::Score;
use common::events::EndGame;
use common::AppState;

use super::components::Dinosaur;
use super::resources::{DinoDown, DinoVerticalMovement};
use super::{
    DINO_DOWN_Y_POS, DINO_INITIAL_VERTICAL_SPEED, DINO_INITIAL_Y_POS, DINO_X_POS, GRAVITY,
};

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
    dino_down: Res<DinoDown>,
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
        } else if keyboard_input.just_pressed(KeyCode::Space) && !dino_down.is_down {
            dino_vertical_movement.moving = true;
            dinosaur_transform.translation.y += dino_vertical_movement.speed;
            dino_vertical_movement.speed += GRAVITY;
        }
    }
}

pub fn handle_collision(
    mut commands: Commands,
    dinosaur_query: Query<(&Handle<Image>, &Transform), With<Dinosaur>>,
    obstacle_query: Query<(&Handle<Image>, &Transform), With<Obstacle>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
    assets: Res<Assets<Image>>,
) {
    if let Ok((dinosaur_image_handle, dinosaur_transform)) = dinosaur_query.get_single() {
        for (obstacle_image_handle, obstacle_transform) in obstacle_query.iter() {
            let dinosaur_dimensions: Vec2 = assets.get(dinosaur_image_handle).unwrap().size();
            let obstacle_dimensions: Vec2 = assets.get(obstacle_image_handle).unwrap().size();
            if collide(
                dinosaur_transform.translation,
                dinosaur_dimensions,
                obstacle_transform.translation,
                obstacle_dimensions,
            )
            .is_some()
            {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame {
                    score: score.value as usize,
                });
            }
        }
    }
}

pub fn dinosaur_down_movement(
    mut commands: Commands,
    mut dinosaur_query: Query<Entity, With<Dinosaur>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    dinosaur_vertical_movement: Res<DinoVerticalMovement>,
    mut dino_down: ResMut<DinoDown>,
) {
    if let Ok(dinosaur_entity) = dinosaur_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Down) {
            if !dinosaur_vertical_movement.moving {
                commands.entity(dinosaur_entity).insert(SpriteBundle {
                    transform: Transform::from_xyz(DINO_X_POS, DINO_DOWN_Y_POS, 0.0),
                    texture: asset_server.load("sprites/dino/dino_down_1.png"),
                    ..default()
                });
                dino_down.is_down = true;
            }
        } else if keyboard_input.just_released(KeyCode::Down) && !dinosaur_vertical_movement.moving
        {
            commands.entity(dinosaur_entity).insert(SpriteBundle {
                transform: Transform::from_xyz(DINO_X_POS, DINO_INITIAL_Y_POS, 0.0),
                texture: asset_server.load("sprites/dino/dino_1.png"),
                ..default()
            });
            dino_down.is_down = false;
        }
    }
}

pub fn set_dinosaur_in_initial_position(
    mut commands: Commands,
    mut dinosaur_query: Query<(&mut Transform, Entity), With<Dinosaur>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut dinosaur_transform, dinosaur_entity)) = dinosaur_query.get_single_mut() {
        dinosaur_transform.translation.y = DINO_INITIAL_Y_POS;
        commands.entity(dinosaur_entity).insert(SpriteBundle {
            transform: dinosaur_transform.clone(),
            texture: asset_server.load("sprites/dino/dino_1.png"),
            ..default()
        });
    }
}
