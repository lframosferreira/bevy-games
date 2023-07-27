use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
