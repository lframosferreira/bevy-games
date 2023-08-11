use super::components::{Alien, AlienLaser, HitPoints, Player, PlayerBullet};
use super::resources::{AlienDirection, Score};
use super::{WINDOW_X, WINDOW_Y};
use bevy::sprite::collide_aabb;
use bevy::{prelude::*, window::PrimaryWindow};
use common::events::EndGame;
use common::AppState;
use rand::random;

const PLAYER_WIDTH: f32 = 40.;
const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_HP: usize = 3;
const PLAYER_Y_OFFSET: f32 = 100.;
const ALIEN_WIDTH: f32 = 30.;
const ALIEN_HEIGHT: f32 = 30.;
const ALIEN_SIZE: Vec2 = Vec2::new(ALIEN_WIDTH, ALIEN_HEIGHT);
const ALIEN_SPEED: f32 = 25. * 40.;
const ALIEN_POINTS: usize = 10;
const ALIEN_LINE_OFFSET: f32 = 50.;
const BULLET_LENGTH: f32 = 5.;
const BULLET_SIZE: Vec2 = Vec2::new(BULLET_LENGTH, BULLET_LENGTH);
const LASER_LENGTH: f32 = 8.;
const LASER_SIZE: Vec2 = Vec2::new(LASER_LENGTH, LASER_LENGTH);
const ENEMY_SHOOT_ODDS: f32 = 0.001;

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., PLAYER_Y_OFFSET, 0.0),
            ..default()
        },
        Player,
        HitPoints::new(PLAYER_HP),
    ));
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const PLAYER_SPEED: f32 = 250.;
    if let Ok(mut transform) = player_query.get_single_mut() {
        let translation = &mut transform.translation;
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

pub fn spawn_bullets(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(player) = player_query.get_single() {
        let translation = player.translation;
        if keyboard_input.just_pressed(KeyCode::Space) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(BULLET_SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        translation.x,
                        translation.y + PLAYER_HEIGHT / 2. + BULLET_LENGTH,
                        0.0,
                    ),
                    ..default()
                },
                PlayerBullet,
            ));
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut bullets_query: Query<(Entity, &mut Transform), With<PlayerBullet>>,
    time: Res<Time>,
) {
    const BULLET_SPEED: f32 = 250.;
    for (entity, mut bullet) in bullets_query.iter_mut() {
        let translation = &mut bullet.translation;
        let delta = BULLET_SPEED * time.delta_seconds();
        if translation.y + delta < WINDOW_Y {
            translation.y += delta;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_aliens(mut commands: Commands) {
    commands.insert_resource(AlienDirection::default());
    const Y_OFFSET: f32 = WINDOW_Y - 100.;
    const X_OFFSET: f32 = 30.;
    const COLUMN_OFFSET: f32 = 50.;
    for i in 0..4 {
        for j in 0..=10 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(ALIEN_WIDTH, ALIEN_HEIGHT)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        X_OFFSET + COLUMN_OFFSET * j as f32,
                        Y_OFFSET - i as f32 * ALIEN_LINE_OFFSET,
                        0.0,
                    ),
                    ..default()
                },
                Alien,
            ));
        }
    }
}

pub fn move_aliens(
    mut aliens_query: Query<&mut Transform, With<Alien>>,
    mut direction: ResMut<AlienDirection>,
    mut player_query: Query<&mut HitPoints, With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    if aliens_query.is_empty() {
        spawn_aliens(commands);
        for mut player in player_query.iter_mut() {
            player.increment();
        }
        return;
    }
    let multiplier = aliens_query.iter().len() as f32;
    let mut x: Vec<f32> = Vec::new();
    for alien in aliens_query.iter() {
        x.push(alien.translation.x);
    }
    let delta = ALIEN_SPEED / multiplier * time.delta_seconds();
    if should_aliens_move_horizontally(&x, &direction, &delta) {
        for mut alien in aliens_query.iter_mut() {
            let translation = &mut alien.translation;
            match *direction {
                AlienDirection::Right => translation.x += delta,
                AlienDirection::Left => translation.x -= delta,
            }
        }
    } else {
        for mut alien in aliens_query.iter_mut() {
            let translation = &mut alien.translation;
            translation.y -= ALIEN_LINE_OFFSET;
            if translation.y <= PLAYER_Y_OFFSET {
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame {
                    score: score.score(),
                });
            }
        }
        direction.toggle();
    }
}

fn should_aliens_move_horizontally(
    vec: &Vec<f32>,
    direction: &AlienDirection,
    delta: &f32,
) -> bool {
    for x in vec {
        match direction {
            AlienDirection::Right => {
                if x + delta > WINDOW_X - ALIEN_WIDTH / 2. {
                    return false;
                }
            }
            AlienDirection::Left => {
                if x - delta < ALIEN_WIDTH / 2. {
                    return false;
                }
            }
        }
    }
    true
}

pub fn collide_bullets_with_aliens(
    mut commands: Commands,
    bullets_query: Query<(&Transform, Entity), With<PlayerBullet>>,
    aliens_query: Query<(&Transform, Entity), With<Alien>>,
    mut score: ResMut<Score>,
) {
    for (bullet, bullet_entity) in bullets_query.iter() {
        let bullet_translation = bullet.translation;
        for (alien, entity) in aliens_query.iter() {
            if collide_aabb::collide(
                bullet_translation,
                BULLET_SIZE,
                alien.translation,
                ALIEN_SIZE,
            )
            .is_some()
            {
                commands.entity(entity).despawn();
                commands.entity(bullet_entity).despawn();
                score.increment(ALIEN_POINTS);
            }
        }
    }
}

pub fn spawn_lasers(mut commands: Commands, aliens_query: Query<&Transform, With<Alien>>) {
    for alien in aliens_query.iter() {
        let translation = alien.translation;
        if random::<f32>() < ENEMY_SHOOT_ODDS {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(LASER_SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        translation.x,
                        translation.y - ALIEN_HEIGHT / 2. - LASER_LENGTH,
                        0.0,
                    ),
                    ..default()
                },
                AlienLaser,
            ));
        }
    }
}

pub fn move_lasers(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Transform), With<AlienLaser>>,
    time: Res<Time>,
) {
    const LASER_SPEED: f32 = 250.;
    for (entity, mut laser) in laser_query.iter_mut() {
        let translation = &mut laser.translation;
        let delta = LASER_SPEED * time.delta_seconds();
        if translation.y - delta > 0. {
            translation.y -= delta;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collide_lasers_with_player(
    lasers_query: Query<(&Transform, Entity), With<AlienLaser>>,
    mut player_query: Query<(&Transform, &mut HitPoints), With<Player>>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    for (bullet, entity) in lasers_query.iter() {
        let bullet_translation = bullet.translation;
        for (player, mut hp) in player_query.iter_mut() {
            if collide_aabb::collide(
                bullet_translation,
                BULLET_SIZE,
                player.translation,
                ALIEN_SIZE,
            )
            .is_some()
            {
                hp.hit();
                commands.entity(entity).despawn();
                if hp.points() == 0 {
                    commands.insert_resource(NextState(Some(AppState::GameOver)));
                    game_over_event_writer.send(EndGame {
                        score: score.score(),
                    });
                }
            }
        }
    }
}

pub fn respawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn();
        spawn_player(commands, window_query);
    }
}

pub fn respawn_aliens(mut commands: Commands, aliens_query: Query<Entity, With<Alien>>) {
    for entity in aliens_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_aliens(commands);
}

pub fn despawn_bullets(mut commands: Commands, bullets_query: Query<Entity, With<PlayerBullet>>) {
    for entity in bullets_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_lasers(mut commands: Commands, lasers_query: Query<Entity, With<AlienLaser>>) {
    for entity in lasers_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}

// TODO HUD with Score and Lives
// TODO Barrier
// TODO Motherhip
// TODO Split in modules
// TODO PowerUps?
