use super::{
    components::{Bird, Pipe},
    resources::Gravity,
    WINDOW_X, WINDOW_Y,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use common::game::Score;
use common::{events::EndGame, AppState};
use rand::Rng;

const BIRD_LENGTH: f32 = 40.;
const BIRD_SIZE: Vec2 = Vec2::new(BIRD_LENGTH, BIRD_LENGTH);
const BIRD_X_POS: f32 = 150.;
const PIPE_WIDTH: f32 = 60.;

pub fn reset_gravity(mut gravity: ResMut<Gravity>) {
    gravity.speed = 0.
}

pub fn respawn_bird(mut commands: Commands, bird_query: Query<Entity, With<Bird>>) {
    if let Ok(entity) = bird_query.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_bird(commands);
}

pub fn despawn_pipes(mut commands: Commands, bird_query: Query<Entity, With<Pipe>>) {
    for entity in bird_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_bird(mut commands: Commands) {
    const Y_POS: f32 = 300.;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(BIRD_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(BIRD_X_POS, Y_POS, 0.0),
            ..default()
        },
        Bird {},
    ));
}

pub fn spawn_pipe(mut commands: Commands) {
    const PIPE_BLOCK_SIZE: f32 = 100.;
    const MAX_SIZE: f32 = WINDOW_Y / PIPE_BLOCK_SIZE;
    let mut rng = rand::thread_rng();
    let ceiling_size = rng.gen_range(0..MAX_SIZE as u32 - 1) as f32;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(PIPE_WIDTH, ceiling_size * PIPE_BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                WINDOW_X,
                WINDOW_Y - (ceiling_size * PIPE_BLOCK_SIZE) / 2.,
                0.0,
            ),
            ..default()
        },
        Pipe {
            height: ceiling_size * PIPE_BLOCK_SIZE,
            behind: false,
        },
    ));
    const GAP_SIZE: f32 = 2.;
    let floor_size = MAX_SIZE - ceiling_size - GAP_SIZE;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(PIPE_WIDTH, floor_size * PIPE_BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X, floor_size * PIPE_BLOCK_SIZE / 2., 0.0),
            ..default()
        },
        Pipe {
            height: floor_size * PIPE_BLOCK_SIZE,
            behind: false,
        },
    ));
}

pub fn move_bird(
    time: Res<Time>,
    mut bird_query: Query<&mut Transform, With<Bird>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut gravity: ResMut<Gravity>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        gravity.set_max();
    }
    if let Ok(mut transform) = bird_query.get_single_mut() {
        let delta = gravity.speed * time.delta_seconds();
        let translation = &mut transform.translation;
        if translation.y + delta < 0. {
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(EndGame {
                score: score.get() / 2,
            });
        } else if translation.y + delta > WINDOW_Y - BIRD_LENGTH / 2. {
            translation.y = WINDOW_Y - BIRD_LENGTH / 2.;
        } else {
            translation.y += delta;
        }
        gravity.decrease();
    }
}

pub fn move_pipe(
    mut pipe_query: Query<(Entity, &mut Transform, &mut Pipe), With<Pipe>>,
    time: Res<Time>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    const PIPE_SPEED: f32 = -125.0;
    for (entity, mut transform, mut pipe) in pipe_query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += PIPE_SPEED * time.delta_seconds();
        if translation.x < BIRD_X_POS && !pipe.behind {
            pipe.behind = true;
            score.increment(1);
        }
        if translation.x < -PIPE_WIDTH / 2. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_collision(
    pipe_query: Query<(&Pipe, &Transform), With<Pipe>>,
    bird_query: Query<&Transform, With<Bird>>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    if let Ok(bird) = bird_query.get_single() {
        let bird_translation = bird.translation;
        for (pipe, transform) in pipe_query.iter() {
            if collide(
                bird_translation,
                BIRD_SIZE,
                transform.translation,
                (PIPE_WIDTH, pipe.height).into(),
            )
            .is_some()
            {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame {
                    score: score.get() / 2,
                });
            }
        }
    }
}
