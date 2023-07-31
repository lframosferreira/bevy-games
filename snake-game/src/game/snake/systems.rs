use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::game::fruit::systems::_spawn_fruit;
use crate::game::fruit::FRUIT_SIZE;
use crate::game::score::resources::Score;
use crate::game::{fruit::components::Fruit, BLOCK_SIZE};

use super::components::{Direction, Snake};

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/body.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2.0, 0.),
            ..default()
        },
        Snake::default(),
        Direction::Left,
    ));
}

pub fn update_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Direction, With<Snake>>,
) {
    if let Ok(mut dir) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::H) {
            *dir = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::L) {
            *dir = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::K) {
            *dir = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::J) {
            *dir = Direction::Down;
        }
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since the last frame.
/// See <https://bevyengine.org/examples/2D%20Rendering/move-sprite/>
pub fn sprite_movement(
    mut sprite_position: Query<(&mut Direction, &mut Transform), With<Snake>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for (logo, mut transform) in &mut sprite_position {
        let translation = &mut transform.translation;
        match *logo {
            Direction::Up => {
                if translation.y + BLOCK_SIZE < window.height() {
                    translation.y += BLOCK_SIZE
                }
            }
            Direction::Down => {
                if translation.y - BLOCK_SIZE > 0. {
                    translation.y -= BLOCK_SIZE
                }
            }
            Direction::Left => {
                if translation.x - BLOCK_SIZE > 0. {
                    translation.x -= BLOCK_SIZE
                }
            }
            Direction::Right => {
                if translation.x + BLOCK_SIZE < window.width() {
                    translation.x += BLOCK_SIZE
                }
            }
        }
    }
}

pub fn handle_eat_fruit(
    mut commands: Commands,
    mut snake_query: Query<&Transform, With<Snake>>,
    fruit_query: Query<(Entity, &Transform), With<Fruit>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok(snake_transform) = snake_query.get_single_mut() {
        if let Ok((fruit_entity, fruit_transform)) = fruit_query.get_single() {
            if collide(
                snake_transform.translation,
                Vec2::new(40.0, 40.0),
                fruit_transform.translation,
                Vec2::new(40.0, 40.0),
            )
            .is_some()
            {
                commands.entity(fruit_entity).despawn();

                // possivelmente isso aqui pode virar uma funcao
                // spawn de nova frutinha
                // cor aleatoria / fruta aleatoria
                _spawn_fruit(window_query, commands, asset_server);
            }
        }
    }
}
