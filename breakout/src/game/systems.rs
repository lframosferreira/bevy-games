use super::components::{Ball, Block, Player};
use crate::game::{WINDOW_X, WINDOW_Y};
use bevy::prelude::*;
use bevy::sprite::collide_aabb;

// https://en.wikipedia.org/wiki/Breakout_(video_game)
const PLAYER_Y_OFFSET: f32 = 40.;
const PLAYER_WIDTH: f32 = 200.;
const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
const PLAYER_SPEED: f32 = 400.;
const BLOCK_WIDTH: f32 = 100.;
const BLOCK_HEIGHT: f32 = 30.;
const BLOCK_SIZE: Vec2 = Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT);
// TODO make ball round
const BALL_LENGTH: f32 = 10.;
const BALL_SIZE: Vec2 = Vec2::new(BALL_LENGTH, BALL_LENGTH);

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
                Block,
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

pub fn move_ball(mut ball_query: Query<(&mut Transform, &mut Ball), With<Ball>>, time: Res<Time>) {
    if let Ok((mut transform, mut ball)) = ball_query.get_single_mut() {
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
        } else if translation.y - delta_y > PLAYER_Y_OFFSET + PLAYER_HEIGHT / 2. {
            translation.y -= delta_y;
        } else {
            // TODO tirar vida
            ball.x_speed = 0.;
            ball.y_speed = 0.;
        }
    }
}

pub fn collide_ball_with_player(
    mut ball_query: Query<(&Transform, &mut Ball), With<Ball>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
            if collide_aabb::collide(
                player.translation,
                PLAYER_SIZE,
                transform.translation,
                BALL_SIZE,
            )
            .is_some()
            {
                // TODO original behavior is to update horizontal movement based on collision side
                ball.is_going_up = true;
            }
        }
    }
}

pub fn collide_ball_with_blocks(
    mut ball_query: Query<(&Transform, &mut Ball), With<Ball>>,
    block_query: Query<(&Transform, Entity), With<Block>>,
    mut commands: Commands,
) {
    if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
        for (block, entity) in block_query.iter() {
            if collide_aabb::collide(
                block.translation,
                PLAYER_SIZE,
                transform.translation,
                BALL_SIZE,
            )
            .is_some()
            {
                ball.is_going_up = !ball.is_going_up;
                commands.entity(entity).despawn();
                return;
            }
        }
    }
}
