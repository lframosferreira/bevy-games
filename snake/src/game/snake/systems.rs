use super::components::{Direction, SnakeBody, SnakeHead};
use super::resources::SnakeCounter;
use crate::game::fruit::systems::spawn_fruit;
use crate::game::score::resources::Score;
use crate::game::SIZE;
use crate::game::{fruit::components::Fruit, BLOCK_SIZE};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use common::events::EndGame;
use common::AppState;

const SNAKE_COLOR: Color = Color::LIME_GREEN;

pub fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(SIZE),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        SnakeHead,
        Direction::Left,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(SIZE),
                ..default()
            },
            transform: Transform::from_xyz(
                window.width() / 2. + BLOCK_SIZE,
                window.height() / 2.,
                0.,
            ),
            ..default()
        },
        SnakeBody { count: 0 },
    ));
}

pub fn respawn_snake(
    mut commands: Commands,
    snake_head_query: Query<Entity, With<SnakeHead>>,
    snake_body_query: Query<Entity, With<SnakeBody>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(entity) = snake_head_query.get_single() {
        commands.entity(entity).despawn();
    }
    for entity in snake_body_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_snake(commands, window_query);
}

pub fn update_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_head_query: Query<&mut Direction, With<SnakeHead>>,
) {
    if let Ok(mut dir) = snake_head_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::H) && *dir != Direction::Right {
            *dir = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::L) && *dir != Direction::Left {
            *dir = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::K) && *dir != Direction::Down {
            *dir = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::J) && *dir != Direction::Up {
            *dir = Direction::Down;
        }
    }
}

pub fn move_snake(
    mut commands: Commands,
    body_entities: Query<(Entity, &SnakeBody, &Transform), Without<SnakeHead>>,
    mut head_position: Query<(&mut Direction, &mut Transform), With<SnakeHead>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut counter: ResMut<SnakeCounter>,
    score: Res<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    // TODO: extrair em várias funções

    for (direction, mut transform) in head_position.iter_mut() {
        let translation = &mut transform.translation;

        // Spawno um novo body para ocupar a posição antiga da cabeça
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    custom_size: Some(SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(translation.x, translation.y, 0.),
                ..default()
            },
            SnakeBody {
                count: counter.count,
            },
        ));
        counter.count += 1;

        // Movo a cabeça
        let mut collided = false;
        match *direction {
            Direction::Up => {
                if translation.y + BLOCK_SIZE < window.height() {
                    translation.y += BLOCK_SIZE
                } else {
                    collided = true;
                }
            }
            Direction::Down => {
                if translation.y - BLOCK_SIZE > 0. {
                    translation.y -= BLOCK_SIZE
                } else {
                    collided = true;
                }
            }
            Direction::Left => {
                if translation.x - BLOCK_SIZE > 0. {
                    translation.x -= BLOCK_SIZE
                } else {
                    collided = true;
                }
            }
            Direction::Right => {
                if translation.x + BLOCK_SIZE < window.width() {
                    translation.x += BLOCK_SIZE
                } else {
                    collided = true;
                }
            }
        }

        // Checando colisão com o corpo
        for (_, _, transform) in body_entities.iter() {
            let body_translation = transform.translation;
            if collide(*translation, SIZE, body_translation, SIZE).is_some() {
                collided = true;
            }
        }

        if collided {
            // HACK Aqui nós estamos primeiro ativando o estado de GameOver e depois mandando o
            // valor final do jogo. Isso é necessário pois o menu de GameOver é inicializado no
            // estado GameOver. Ou seja, se a gente mandar o valor final antes de ativar o estado
            // de GameOver, o menu vai achar que não há valor final.
            //
            // Provavelmente tem um jeito melhor de fazer isso, em relação à gerência de estado.
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(EndGame {
                score: score.value as usize,
            });
            // Early return para não comer a cauda
            return;
        }

        // Procuramos a última posição do corpo e removemos
        // Somente se a cobra não comeu
        if !score.is_changed() || score.value == 0 {
            let mut min = u32::MAX;
            let mut tail: Option<Entity> = None;
            for (entity, body, _) in body_entities.iter() {
                if body.count < min {
                    min = body.count;
                    tail = Some(entity);
                }
            }
            if let Some(tail) = tail {
                commands.entity(tail).despawn();
            }
        }
    }
}

pub fn handle_eat_fruit(
    mut commands: Commands,
    mut snake_head_query: Query<&Transform, With<SnakeHead>>,
    fruit_query: Query<(Entity, &Transform), With<Fruit>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
) {
    if let Ok(head_transform) = snake_head_query.get_single_mut() {
        if let Ok((fruit_entity, fruit_transform)) = fruit_query.get_single() {
            if collide(
                head_transform.translation,
                SIZE,
                fruit_transform.translation,
                SIZE,
            )
            .is_some()
            {
                commands.entity(fruit_entity).despawn();
                score.value += 1;
                spawn_fruit(commands, window_query);
            }
        }
    }
}
