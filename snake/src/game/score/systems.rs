use crate::game::score::resources::*;
use bevy::prelude::*;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.value = 0
}
