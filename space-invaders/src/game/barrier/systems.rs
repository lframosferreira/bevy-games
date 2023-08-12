use super::components::Barrier;
use crate::game::{components::HitPoints, player::PLAYER_Y_OFFSET};
use bevy::prelude::*;

pub fn spawn_barriers(mut commands: Commands) {
    const X_OFFSET: f32 = 120.;
    const Y_OFFSET: f32 = PLAYER_Y_OFFSET + 100.;
    const COLUMN_OFFSET: f32 = X_OFFSET;
    const BARRIER_HP: usize = 10;
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

pub fn respawn_barriers(mut commands: Commands, barriers_query: Query<Entity, With<Barrier>>) {
    for entity in barriers_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_barriers(commands);
}
