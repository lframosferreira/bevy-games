use crate::game::score::resources::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.value = 0
}

pub fn tick_score_update_timer(mut score_update_timer: ResMut<ScoreUpdateTimer>, time: Res<Time>) {
    score_update_timer.timer.tick(time.delta());
}

pub fn update_score(mut score: ResMut<Score>, score_update_timer: Res<ScoreUpdateTimer>) {
    if score_update_timer.timer.finished() {
        score.value += 1
    }
}

#[derive(Component, Default)]
pub struct ScoreText;

pub fn spawn_score_text(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score 00000",
                TextStyle {
                    color: Color::WHITE,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                },
            ),
            transform: Transform::from_xyz(window.width() - 60.0, window.height() - 20.0, 0.0),
            ..default()
        },
        ScoreText,
    ));
}

pub fn update_score_text(
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    if let Ok(mut text) = score_text_query.get_single_mut() {
        for section in &mut text.sections {
            section.value = format!("Score {:0>5}", score.value);
        }
    }
}
