use super::components::{Ball, Block, Player};
use super::MAX_LIVES;
use crate::game::{WINDOW_X, WINDOW_Y};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{self, Collision};
use common::game::{Lives, Score};

// https://en.wikipedia.org/wiki/Breakout_(video_game)
const PLAYER_Y_OFFSET: f32 = 40.;
const PLAYER_WIDTH: f32 = 200.;
const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
const PLAYER_SPEED: f32 = 500.;
const BLOCK_WIDTH: f32 = 100.;
const BLOCK_HEIGHT: f32 = 30.;
const BLOCK_SIZE: Vec2 = Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT);
// TODO make ball round
const BALL_LENGTH: f32 = 10.;
const BALL_SIZE: Vec2 = Vec2::new(BALL_LENGTH, BALL_LENGTH);

pub fn reset_lives(mut commands: Commands) {
    commands.insert_resource(Lives::new(MAX_LIVES));
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., PLAYER_Y_OFFSET, 0.0),
            ..default()
        },
    ));
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let translation = &mut player.translation;
        let delta = PLAYER_SPEED * time.delta_seconds();
        if keyboard_input.pressed(KeyCode::Right)
            && translation.x + delta < WINDOW_X - PLAYER_WIDTH / 2.
        {
            translation.x += delta;
        }
        if keyboard_input.pressed(KeyCode::Left) && translation.x - delta > PLAYER_WIDTH / 2. {
            translation.x -= delta;
        }
    }
}

pub fn respawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_player(commands);
}

pub fn spawn_blocks(mut commands: Commands) {
    const BLOCK_Y_OFFSET: f32 = WINDOW_Y - 100.;
    const BLOCK_COLORS: [Color; 6] = [
        Color::RED,
        Color::ORANGE_RED,
        Color::ORANGE,
        Color::YELLOW,
        Color::GREEN,
        Color::BLUE,
    ];
    for (i, &color) in BLOCK_COLORS.iter().enumerate() {
        for j in 0..(WINDOW_X / BLOCK_WIDTH) as u32 {
            commands.spawn((
                Block(BLOCK_COLORS.len() - i),
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(BLOCK_SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        BLOCK_WIDTH / 2. + j as f32 * BLOCK_WIDTH,
                        BLOCK_Y_OFFSET - i as f32 * BLOCK_HEIGHT,
                        0.0,
                    ),
                    ..default()
                },
            ));
        }
    }
}

pub fn respawn_blocks(mut commands: Commands, block_query: Query<Entity, With<Block>>) {
    for entity in block_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_blocks(commands);
}

pub fn spawn_ball(mut commands: Commands) {
    const BALL_Y_OFFSET: f32 = PLAYER_Y_OFFSET + PLAYER_HEIGHT;
    commands.spawn((
        Ball::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., BALL_Y_OFFSET, 0.0),
            ..default()
        },
    ));
}

pub fn respawn_ball(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    if let Ok(entity) = ball_query.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_ball(commands);
}

pub fn move_ball(
    mut ball_query: Query<(&mut Transform, &mut Ball, Entity), With<Ball>>,
    time: Res<Time>,
    mut lives: ResMut<Lives>,
    mut commands: Commands,
) {
    if let Ok((mut transform, mut ball, entity)) = ball_query.get_single_mut() {
        let translation = &mut transform.translation;
        let delta_x = ball.x_speed * time.delta_seconds();
        if ball.is_going_right {
            if translation.x + delta_x <= WINDOW_X - BALL_LENGTH / 2. {
                translation.x += delta_x;
            } else {
                ball.is_going_right = false;
            }
        } else if translation.x - delta_x >= BALL_LENGTH / 2. {
            translation.x -= delta_x;
        } else {
            ball.is_going_right = true;
        }
        let delta_y = ball.y_speed * time.delta_seconds();
        if ball.is_going_up {
            if translation.y + delta_y <= WINDOW_Y - BALL_LENGTH / 2. {
                translation.y += delta_y;
            } else {
                ball.is_going_up = false;
            }
        } else if translation.y - delta_y > PLAYER_Y_OFFSET {
            translation.y -= delta_y;
        } else {
            lives.decrement();
            commands.entity(entity).despawn();
            if lives.get() > 0 {
                spawn_ball(commands);
            }
        }
    }
}

pub fn collide_ball_with_player(
    mut ball_query: Query<(&Transform, &mut Ball), With<Ball>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((ball_transform, mut ball)) = ball_query.get_single_mut() {
            let player_pos = player_transform.translation;
            let ball_pos = ball_transform.translation;
            if collide_aabb::collide(player_pos, PLAYER_SIZE, ball_pos, BALL_SIZE).is_some() {
                ball.is_going_right = ball_pos.x >= player_pos.x;
                ball.is_going_up = true;
            }
        }
    }
}

pub fn collide_ball_with_blocks(
    mut ball_query: Query<(&Transform, &mut Ball), With<Ball>>,
    block_query: Query<(&Transform, Entity, &Block), With<Block>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    if let Ok((ball_transform, mut ball)) = ball_query.get_single_mut() {
        for (block_transform, entity, block) in block_query.iter() {
            if let Some(collision) = collide_aabb::collide(
                block_transform.translation,
                BLOCK_SIZE,
                ball_transform.translation,
                BALL_SIZE,
            ) {
                match collision {
                    Collision::Top | Collision::Bottom => ball.is_going_up = !ball.is_going_up,
                    _ => ball.is_going_right = !ball.is_going_right,
                }

                commands.entity(entity).despawn();

                score.increment(block.0);

                // We destroyed the last block
                if block_query.iter().len() == 1 {
                    spawn_blocks(commands);
                }

                return;
            }
        }
    }
}
