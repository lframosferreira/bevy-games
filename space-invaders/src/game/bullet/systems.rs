use super::components::Bullet;
use crate::game::{components::Stats, player::components::Player, WINDOW_Y};
use bevy::prelude::*;

pub fn spawn_bullets(
    mut commands: Commands,
    player_query: Query<(&Transform, &Stats), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(player) = player_query.get_single() {
        let (transform, stats) = player;
        let translation = transform.translation;
        if keyboard_input.just_pressed(KeyCode::Space) {
            let bullet = Bullet::default();
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
    mut bullets_query: Query<(Entity, &mut Transform, &Stats), With<Bullet>>,
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

pub fn despawn_bullets(mut commands: Commands, bullets_query: Query<Entity, With<Bullet>>) {
    for entity in bullets_query.iter() {
        commands.entity(entity).despawn();
    }
}
