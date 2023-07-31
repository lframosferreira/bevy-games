use bevy::prelude::*;

use crate::game::score::resources::*;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}
