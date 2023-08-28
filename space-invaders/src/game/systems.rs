use super::alien::components::Alien;
use super::alien::ALIEN_POINTS;
use super::barrier::components::Barrier;
use super::boss::components::Boss;
use super::boss::BOSS_POINTS;
use super::bullet::components::Bullet;
use super::components::{HitPoints, Stats};
use super::laser::components::Laser;
use super::player::components::Player;
use super::MAX_LIVES;
use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use common::game::{Lives, Score};

pub fn reset_lives(mut commands: Commands) {
    commands.insert_resource(Lives::new(MAX_LIVES));
}

type ProjectileQuery = Or<(With<Bullet>, With<Laser>)>;

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
