use super::resources::Score;
use bevy::prelude::*;

pub fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}
