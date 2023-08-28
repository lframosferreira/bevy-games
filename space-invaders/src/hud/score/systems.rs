use super::components::ScoreHUD;
use crate::hud::FONT_SIZE;
use bevy::prelude::*;
use common::game::Score;

pub fn spawn_score_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    const X_OFFSET: f32 = 500.;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score 00000",
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

pub fn update_score_hud(score: Res<Score>, mut score_hud_query: Query<&mut Text, With<ScoreHUD>>) {
    if let Ok(mut text) = score_hud_query.get_single_mut() {
        for section in &mut text.sections {
            section.value = format!("Score {:0>5}", score.get());
        }
    }
}

pub fn despawn_score_hud(mut commands: Commands, hud_query: Query<Entity, With<ScoreHUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
}
