use super::components::{Alien, AlienLaser, Barrier, HitPoints, Player, PlayerBullet, Stats};
use super::resources::{AlienDirection, Score};
use super::{WINDOW_X, WINDOW_Y};
use bevy::sprite::collide_aabb;
use bevy::{prelude::*, window::PrimaryWindow};
use common::events::EndGame;
use common::AppState;
use rand::random;

const PLAYER_HP: usize = 3;
const PLAYER_Y_OFFSET: f32 = 100.;
const ALIEN_POINTS: usize = 10;
const ALIEN_LINE_OFFSET: f32 = 50.;
const BARRIER_HP: usize = 10;
const ENEMY_SHOOT_ODDS: f32 = 0.001;

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let player = Player::default();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: player.stats.color(),
                custom_size: Some(player.stats.size()),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., PLAYER_Y_OFFSET, 0.0),
            ..default()
        },
        player,
        player.stats,
        HitPoints::new(PLAYER_HP),
    ));
}

pub fn move_player(
    mut player_query: Query<(&mut Transform, &Stats), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.get_single_mut() {
        let (mut transform, stats) = player;
        let translation = &mut transform.translation;
        let delta = stats.speed() * time.delta_seconds();
        if keyboard_input.pressed(KeyCode::Right)
            && translation.x + delta < WINDOW_X - stats.width() / 2.
        {
            translation.x += delta;
        }
        if keyboard_input.pressed(KeyCode::Left) && translation.x - delta > stats.width() / 2. {
            translation.x -= delta;
        }
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    player_query: Query<(&Transform, &Stats), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(player) = player_query.get_single() {
        let (transform, stats) = player;
        let translation = transform.translation;
        if keyboard_input.just_pressed(KeyCode::Space) {
            let bullet = PlayerBullet::default();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: bullet.stats.color(),
                        custom_size: Some(bullet.stats.size()),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        translation.x,
                        translation.y + stats.height() / 2. + bullet.stats.height(),
                        0.0,
                    ),
                    ..default()
                },
                bullet,
                bullet.stats,
            ));
        }
    }
}

pub fn move_bullets(
    mut commands: Commands,
    mut bullets_query: Query<(Entity, &mut Transform, &Stats), With<PlayerBullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, stats) in bullets_query.iter_mut() {
        let translation = &mut transform.translation;
        let delta = stats.speed() * time.delta_seconds();
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
            let alien = Alien::default();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: alien.stats.color(),
                        custom_size: Some(alien.stats.size()),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        X_OFFSET + COLUMN_OFFSET * j as f32,
                        Y_OFFSET - i as f32 * ALIEN_LINE_OFFSET,
                        0.0,
                    ),
                    ..default()
                },
                alien,
                alien.stats,
            ));
        }
    }
}

pub fn move_aliens(
    mut aliens_query: Query<(&mut Transform, &Stats), With<Alien>>,
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
    let mut x_pos: Vec<f32> = Vec::new();
    let mut speeds: Vec<f32> = Vec::new();
    let mut widths: Vec<f32> = Vec::new();
    for (transform, stats) in aliens_query.iter() {
        x_pos.push(transform.translation.x);
        speeds.push(stats.speed());
        widths.push(stats.width());
    }
    if should_aliens_move_horizontally(&x_pos, &speeds, &widths, &direction, &time) {
        for (mut alien, stats) in aliens_query.iter_mut() {
            let delta = stats.speed() / multiplier * time.delta_seconds();
            let translation = &mut alien.translation;
            match *direction {
                AlienDirection::Right => translation.x += delta,
                AlienDirection::Left => translation.x -= delta,
            }
        }
    } else {
        for (mut alien, _) in aliens_query.iter_mut() {
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
    x_pos: &[f32],
    speeds: &[f32],
    widths: &[f32],
    direction: &AlienDirection,
    time: &Res<Time>,
) -> bool {
    for (i, x) in x_pos.iter().enumerate() {
        let delta = speeds[i] * time.delta_seconds();
        match direction {
            AlienDirection::Right => {
                if x + delta > WINDOW_X - widths[i] / 2. {
                    return false;
                }
            }
            AlienDirection::Left => {
                if x - delta < widths[i] / 2. {
                    return false;
                }
            }
        }
    }
    true
}

pub fn collide_bullets_with_aliens(
    mut commands: Commands,
    bullets_query: Query<(&Transform, Entity, &Stats), With<PlayerBullet>>,
    aliens_query: Query<(&Transform, Entity, &Stats), With<Alien>>,
    mut score: ResMut<Score>,
) {
    for (bullet, bullet_entity, bullet_stats) in bullets_query.iter() {
        let bullet_translation = bullet.translation;
        for (alien, entity, alien_stats) in aliens_query.iter() {
            if collide_aabb::collide(
                bullet_translation,
                bullet_stats.size(),
                alien.translation,
                alien_stats.size(),
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

pub fn spawn_lasers(
    mut commands: Commands,
    aliens_query: Query<(&Transform, &Stats), With<Alien>>,
) {
    for (transform, stats) in aliens_query.iter() {
        let translation = transform.translation;
        if random::<f32>() < ENEMY_SHOOT_ODDS {
            let laser = AlienLaser::default();
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: laser.stats.color(),
                        custom_size: Some(laser.stats.size()),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        translation.x,
                        translation.y - stats.height() / 2. - laser.stats.height(),
                        0.0,
                    ),
                    ..default()
                },
                laser,
                laser.stats,
            ));
        }
    }
}

pub fn move_lasers(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Transform, &Stats), With<AlienLaser>>,
    time: Res<Time>,
) {
    for (entity, mut laser, stats) in laser_query.iter_mut() {
        let translation = &mut laser.translation;
        let delta = stats.speed() * time.delta_seconds();
        if translation.y - delta > 0. {
            translation.y -= delta;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collide_lasers_with_player(
    lasers_query: Query<(&Transform, Entity, &Stats), With<AlienLaser>>,
    mut player_query: Query<(&Transform, &mut HitPoints, &Stats), With<Player>>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    for (laser, entity, laser_stats) in lasers_query.iter() {
        let laser_translation = laser.translation;
        for (player, mut hp, player_stats) in player_query.iter_mut() {
            if collide_aabb::collide(
                laser_translation,
                laser_stats.size(),
                player.translation,
                player_stats.size(),
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

pub fn spawn_barriers(mut commands: Commands) {
    const X_OFFSET: f32 = 100.;
    const Y_OFFSET: f32 = PLAYER_Y_OFFSET + 90.;
    const COLUMN_OFFSET: f32 = 130.;
    for i in 0..4 {
        let barrier = Barrier::default();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: barrier.stats.color(),
                    custom_size: Some(barrier.stats.size()),
                    ..default()
                },
                transform: Transform::from_xyz(X_OFFSET + COLUMN_OFFSET * i as f32, Y_OFFSET, 0.0),
                ..default()
            },
            barrier,
            barrier.stats,
            HitPoints::new(BARRIER_HP),
        ));
    }
}

type ProjectileQuery = Or<(With<PlayerBullet>, With<AlienLaser>)>;

pub fn collide_projectiles_with_barriers(
    projectile_query: Query<(&Transform, Entity, &Stats), ProjectileQuery>,
    mut barriers_query: Query<(&Transform, &mut HitPoints, Entity, &Stats), With<Barrier>>,
    mut commands: Commands,
) {
    for (transform, projectile_entity, projectile_stats) in projectile_query.iter() {
        let translation = transform.translation;
        for (barrier, mut hp, entity, barrier_stats) in barriers_query.iter_mut() {
            if collide_aabb::collide(
                translation,
                projectile_stats.size(),
                barrier.translation,
                barrier_stats.size(),
            )
            .is_some()
            {
                commands.entity(projectile_entity).despawn();
                hp.hit();
                if hp.points() == 0 {
                    commands.entity(entity).despawn();
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

pub fn respawn_barriers(mut commands: Commands, barriers_query: Query<Entity, With<Barrier>>) {
    for entity in barriers_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_barriers(commands);
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
