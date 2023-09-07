use super::resources::Lives;
use crate::{events::EndGame, game::score::Score, AppState};
use bevy::prelude::*;

pub fn watch_death(
    lives: Res<Lives>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut commands: Commands,
    score: Res<Score>,
) {
    if lives.get() == 0 {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        game_over_event_writer.send(EndGame::new_number(score.get()));
    }
}

pub fn bring_back_to_life(mut commands: Commands) {
    commands.insert_resource(Lives::default());
}
