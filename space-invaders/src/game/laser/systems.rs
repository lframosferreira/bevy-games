use super::components::Laser;
use crate::game::{alien::components::Alien, components::Stats};
use bevy::prelude::*;
use rand::random;

const ENEMY_SHOOT_ODDS: f32 = 0.003;

pub fn spawn_lasers(
    mut commands: Commands,
    aliens_query: Query<(&Transform, &Stats), With<Alien>>,
) {
    for (transform, stats) in aliens_query.iter() {
        let translation = transform.translation;
        if random::<f32>() < ENEMY_SHOOT_ODDS {
            let laser = Laser::default();
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
    mut laser_query: Query<(Entity, &mut Transform, &Stats), With<Laser>>,
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

pub fn despawn_lasers(mut commands: Commands, lasers_query: Query<Entity, With<Laser>>) {
    for entity in lasers_query.iter() {
        commands.entity(entity).despawn();
    }
}
