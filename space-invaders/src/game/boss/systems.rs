use super::components::Boss;
use crate::game::{components::Stats, WINDOW_X, WINDOW_Y};
use bevy::prelude::*;
use rand::random;

const BOSS_SPAWN_ODDS: f32 = 0.001;

pub fn spawn_boss(mut commands: Commands) {
    if random::<f32>() < BOSS_SPAWN_ODDS {
        let boss = Boss::default();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: boss.stats.color(),
                    custom_size: Some(boss.stats.size()),
                    ..default()
                },
                transform: Transform::from_xyz(
                    -boss.stats.width(),
                    WINDOW_Y - boss.stats.height(),
                    0.0,
                ),
                ..default()
            },
            boss,
            boss.stats,
        ));
    }
}

pub fn move_boss(
    mut boss_query: Query<(&mut Transform, Entity, &Stats), With<Boss>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, entity, stats) in boss_query.iter_mut() {
        let translation = &mut transform.translation;
        let delta = stats.speed() * time.delta_seconds();
        if translation.x + delta <= WINDOW_X + stats.width() / 2. {
            translation.x += delta;
        } else {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_boss(mut commands: Commands, boss_query: Query<Entity, With<Boss>>) {
    for entity in boss_query.iter() {
        commands.entity(entity).despawn();
    }
}
