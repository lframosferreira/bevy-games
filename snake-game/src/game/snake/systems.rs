use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::game::fruit::components::Fruit;
use crate::game::score::resources::Score;

use super::{
    components::{Direction, Snake},
    SNAKE_SPEED,
};

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/body.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
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
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += SNAKE_SPEED * time.delta_seconds(),
            Direction::Down => transform.translation.y -= SNAKE_SPEED * time.delta_seconds(),
            Direction::Left => transform.translation.x -= SNAKE_SPEED * time.delta_seconds(),
            Direction::Right => transform.translation.x += SNAKE_SPEED * time.delta_seconds(),
        }

        // TODO extrair isso aqui em uma função
        // TODO levar em conta o comprimento das partes para fazer a colisão com a parede
        if transform.translation.x > window.width() {
            *logo = Direction::Left
        } else if transform.translation.x < 0. {
            *logo = Direction::Right
        }

        // TODO levar em conta o comprimento das partes para fazer a colisão com a parede
        if transform.translation.y > window.height() {
            *logo = Direction::Down;
        } else if transform.translation.y < 0. {
            *logo = Direction::Up;
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
                let window: &Window = window_query.get_single().unwrap();
                let fruit_x_pos: f32 = random::<f32>() * window.width();
                let fruit_y_pos: f32 = random::<f32>() * window.height();
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(fruit_x_pos, fruit_y_pos, 0.0),
                        texture: asset_server.load("sprites/fruit.png"),
                        ..default()
                    },
                    Fruit {
                        x_pos: fruit_x_pos,
                        y_pos: fruit_y_pos,
                    },
                ));
            }
        }
    }
}
