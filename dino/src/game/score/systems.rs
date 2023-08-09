use crate::game::score::resources::*;
use bevy::prelude::*;

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
