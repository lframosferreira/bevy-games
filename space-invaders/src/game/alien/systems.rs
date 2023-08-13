use super::components::Alien;
use super::resources::AlienDirection;
use crate::game::components::Stats;
use crate::game::lives::resources::Lives;
use crate::game::player::PLAYER_Y_OFFSET;
use crate::game::{WINDOW_X, WINDOW_Y};
use bevy::prelude::*;

const ALIEN_LINE_OFFSET: f32 = 50.;

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
    time: Res<Time>,
    commands: Commands,
    mut lives: ResMut<Lives>,
) {
    if aliens_query.is_empty() {
        spawn_aliens(commands);
        lives.increment();
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
                lives.zero();
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

pub fn respawn_aliens(mut commands: Commands, aliens_query: Query<Entity, With<Alien>>) {
    for entity in aliens_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_aliens(commands);
}
