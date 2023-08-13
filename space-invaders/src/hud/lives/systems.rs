use super::components::LivesHUD;
use crate::game::{player_sprite, Lives, LIVES};
use bevy::prelude::*;

const LIVES_X_OFFSET: f32 = 30.;
const LIVES_COL_OFFSET: f32 = 50.;

pub fn spawn_lives_hud(mut commands: Commands) {
    let sprite = player_sprite();
    for i in 0..LIVES {
        commands.spawn((
            SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform::from_xyz(
                    LIVES_X_OFFSET + i as f32 * LIVES_COL_OFFSET,
                    40. / 2.,
                    0.0,
                ),
                ..default()
            },
            LivesHUD,
        ));
    }
}

pub fn update_lives_hud(
    lives: Option<Res<Lives>>,
    score_hud_query: Query<Entity, With<LivesHUD>>,
    mut commands: Commands,
) {
    if let Some(lives) = lives {
        if lives.is_changed() && !lives.is_added() {
            let len = score_hud_query.iter().len();
            let lives = lives.get();
            if lives == 0 {
                for entity in score_hud_query.iter() {
                    commands.entity(entity).despawn();
                }
            } else if len - 1 == lives {
                if let Some(entity) = score_hud_query.iter().last() {
                    commands.entity(entity).despawn();
                }
            // É importante checar se a variação é realmente 1 pois ao resetar o contador de vidas
            // (isto é, mudar de 0 para 3), a variação é maior que 1 e não queremos spawnar um
            // novo sprite porque o reset toma conta disso
            } else if len + 1 == lives {
                let sprite = player_sprite();
                commands.spawn((
                    SpriteBundle {
                        sprite: sprite.clone(),
                        transform: Transform::from_xyz(
                            LIVES_X_OFFSET + len as f32 * LIVES_COL_OFFSET,
                            40. / 2.,
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

pub fn despawn_live_hud(mut commands: Commands, hud_query: Query<Entity, With<LivesHUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
}
