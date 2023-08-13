use super::alien::components::Alien;
use super::alien::ALIEN_POINTS;
use super::barrier::components::Barrier;
use super::boss::components::Boss;
use super::boss::BOSS_POINTS;
use super::bullet::components::Bullet;
use super::components::{HitPoints, LivesHUD, ScoreHUD, Stats};
use super::laser::components::Laser;
use super::lives::resources::Lives;
use super::player::components::Player;
use super::score::resources::Score;
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use common::events::EndGame;
use common::AppState;

const LIVES_X_OFFSET: f32 = 30.;
const LIVES_COL_OFFSET: f32 = 50.;
const FONT_SIZE: f32 = 40.;

type ProjectileQuery = Or<(With<Bullet>, With<Laser>)>;

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    const X_OFFSET: f32 = 500.;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Score {:05}", score.score()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(X_OFFSET, FONT_SIZE / 2., 0.),
            ..default()
        },
        ScoreHUD,
    ));
}

pub fn update_score(score: Res<Score>, mut score_hud_query: Query<&mut Text, With<ScoreHUD>>) {
    if let Ok(mut text) = score_hud_query.get_single_mut() {
        for section in &mut text.sections {
            section.value = format!("Score {:05}", score.score());
        }
    }
}

pub fn spawn_lives_hud(mut commands: Commands, lives: Option<Res<Lives>>) {
    let player = Player::default();
    let sprite = Sprite {
        color: player.stats.color(),
        custom_size: Some(player.stats.size()),
        ..default()
    };
    if let Some(lives) = lives {
        for i in 0..lives.get() {
            commands.spawn((
                SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform::from_xyz(
                        LIVES_X_OFFSET + i as f32 * LIVES_COL_OFFSET,
                        FONT_SIZE / 2.,
                        0.0,
                    ),
                    ..default()
                },
                LivesHUD,
            ));
        }
    }
}

pub fn update_lives(
    lives: Option<Res<Lives>>,
    score_hud_query: Query<Entity, With<LivesHUD>>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
    score: Res<Score>,
) {
    if let Some(lives) = lives {
        if lives.is_changed() && !lives.is_added() {
            let len = score_hud_query.iter().len();
            let lives = lives.get();
            if lives == 0 {
                for entity in score_hud_query.iter() {
                    commands.entity(entity).despawn();
                }
                commands.insert_resource(NextState(Some(AppState::GameOver)));
                game_over_event_writer.send(EndGame {
                    score: score.score(),
                });
            } else if len - 1 == lives {
                if let Some(entity) = score_hud_query.iter().last() {
                    commands.entity(entity).despawn();
                }
            // É importante checar se a variação é realmente 1 pois ao resetar o contador de vidas
            // (isto é, mudar de 0 para 3), a variação é maior que 1 e não queremos spawnar um
            // novo sprite porque o reset toma conta disso
            } else if len + 1 == lives {
                let player = Player::default();
                let sprite = Sprite {
                    color: player.stats.color(),
                    custom_size: Some(player.stats.size()),
                    ..default()
                };
                commands.spawn((
                    SpriteBundle {
                        sprite: sprite.clone(),
                        transform: Transform::from_xyz(
                            LIVES_X_OFFSET + len as f32 * LIVES_COL_OFFSET,
                            FONT_SIZE / 2.,
                            0.0,
                        ),
                        ..default()
                    },
                    LivesHUD,
                ));
            }
        }
    }
}

pub fn collide_bullets_with_aliens(
    mut commands: Commands,
    bullets_query: Query<(&Transform, Entity, &Stats), With<Bullet>>,
    aliens_query: Query<(&Transform, Entity, &Stats), With<Alien>>,
    mut score: ResMut<Score>,
) {
    for (bullet, bullet_entity, bullet_stats) in bullets_query.iter() {
        let bullet_translation = bullet.translation;
        for (alien, alien_entity, alien_stats) in aliens_query.iter() {
            if collide_aabb::collide(
                bullet_translation,
                bullet_stats.size(),
                alien.translation,
                alien_stats.size(),
            )
            .is_some()
            {
                commands.entity(alien_entity).despawn();
                commands.entity(bullet_entity).despawn();
                score.increment(ALIEN_POINTS);
            }
        }
    }
}

pub fn collide_lasers_with_player(
    lasers_query: Query<(&Transform, Entity, &Stats), With<Laser>>,
    player_query: Query<(&Transform, &Stats), With<Player>>,
    mut commands: Commands,
    mut lives: ResMut<Lives>,
) {
    for (laser, entity, laser_stats) in lasers_query.iter() {
        let laser_translation = laser.translation;
        for (player, player_stats) in player_query.iter() {
            if collide_aabb::collide(
                laser_translation,
                laser_stats.size(),
                player.translation,
                player_stats.size(),
            )
            .is_some()
            {
                lives.decrement();
                commands.entity(entity).despawn();
            }
        }
    }
}

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

pub fn collide_bullets_with_boss(
    bullets_query: Query<(&Transform, Entity, &Stats), With<Bullet>>,
    boss_query: Query<(&Transform, Entity, &Stats), With<Boss>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for (bullet, bullet_entity, bullet_stats) in bullets_query.iter() {
        let bullet_translation = bullet.translation;
        for (alien, boss_entity, alien_stats) in boss_query.iter() {
            if collide_aabb::collide(
                bullet_translation,
                bullet_stats.size(),
                alien.translation,
                alien_stats.size(),
            )
            .is_some()
            {
                commands.entity(boss_entity).despawn();
                commands.entity(bullet_entity).despawn();
                score.increment(BOSS_POINTS);
            }
        }
    }
}

pub fn respawn_live_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<LivesHUD>>,
    lives: Option<Res<Lives>>,
) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_lives_hud(commands, lives);
}
