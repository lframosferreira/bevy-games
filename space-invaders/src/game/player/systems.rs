use super::{components::Player, PLAYER_Y_OFFSET};
use crate::game::{components::Stats, WINDOW_X};
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    let player = Player::default();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: player.stats.color(),
                custom_size: Some(player.stats.size()),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_X / 2., PLAYER_Y_OFFSET, 0.0),
            ..default()
        },
        player,
        player.stats,
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

pub fn respawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn();
        spawn_player(commands);
    }
}
