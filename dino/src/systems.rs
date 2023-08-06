use bevy::prelude::*;
use common::{events::EndGame, AppState};

pub const WINDOW_X: f32 = 1000.0;
pub const WINDOW_Y: f32 = 600.0;

pub fn pause_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && app_state.get() == &AppState::InGame {
        next_app_state.set(AppState::Pause);
    }
}

pub fn resume_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && app_state.get() == &AppState::Pause {
        next_app_state.set(AppState::InGame);
    }
}

pub fn death_sound_effect(
    mut game_over_event_reader: EventReader<EndGame>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for _ in game_over_event_reader.iter() {
        let sound_effect = asset_server.load("audio/lego-yoda-death-sound-effect.ogg");
        commands.spawn(AudioBundle {
            source: sound_effect,
            ..default()
        });
    }
}