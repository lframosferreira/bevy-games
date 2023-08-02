use super::components::{Direction, SnakeBody, SnakeCounter, SnakeHead};
use crate::events::GameOver;
use crate::game::fruit::systems::spawn_fruit;
use crate::game::score::resources::Score;
use crate::game::{fruit::components::Fruit, BLOCK_SIZE};
use crate::AppState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

pub fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.25),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
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
                color: Color::rgb(0.25, 0.75, 0.25),
                custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
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

pub fn move_snake(
    mut commands: Commands,
    body_positions: Query<(Entity, &SnakeBody), With<SnakeBody>>,
    mut head_position: Query<(&mut Direction, &mut Transform), With<SnakeHead>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut counter: ResMut<SnakeCounter>,
    score: Res<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    // TODO: extrair em várias funções

    for (direction, mut transform) in &mut head_position {
        let translation = &mut transform.translation;

        // Spawno um novo body para ocupar a posição antiga da cabeça
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
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

        // TODO checar colisão com corpo

        if collided {
            // HACK Aqui nós estamos primeiro ativando o estado de GameOver e depois mandando o
            // valor final do jogo. Isso é necessário pois o menu de GameOver é inicializado no
            // estado GameOver. Ou seja, se a gente mandar o valor final antes de ativar o estado
            // de GameOver, o menu vai achar que não há valor final.
            //
            // Provavelmente tem um jeito melhor de fazer isso, em relação à gerência de estado.
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(GameOver { score: score.value });
            // Early return para não comer a cauda
            return;
        }

        // Procuramos a última posição do corpo e removemos
        // Somente se a cobra não comeu
        if !score.is_changed() || score.value == 0 {
            let mut min = u32::MAX;
            let mut tail: Option<Entity> = None;
            for (entity, body) in body_positions.iter() {
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
                Vec2::new(40.0, 40.0),
                fruit_transform.translation,
                Vec2::new(40.0, 40.0),
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
