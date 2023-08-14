use super::resources::Lives;
use crate::game::Score;
use bevy::prelude::*;
use common::{events::EndGame, AppState};

pub fn watch_death(
    lives: Res<Lives>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut commands: Commands,
    score: Res<Score>,
) {
    if lives.get() == 0 {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        game_over_event_writer.send(EndGame {
            score: score.score(),
        });
    }
}

pub fn reset_lives(mut lives: ResMut<Lives>) {
    lives.reset();
}
