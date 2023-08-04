use super::{
    components::{QuitButton, ResumeButton},
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    InteractionAndBackgroundColor,
};
use crate::AppState;
use bevy::{app::AppExit, prelude::*};

pub fn interact_with_button<F, T>(
    mut button_query: Query<InteractionAndBackgroundColor, (Changed<Interaction>, With<T>)>,
    action: F,
) where
    F: FnOnce(),
    T: Component,
{
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                action();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    button_query: Query<InteractionAndBackgroundColor, (Changed<Interaction>, With<QuitButton>)>,
) {
    interact_with_button(button_query, || app_exit_event_writer.send(AppExit))
}

pub fn interact_with_resume_button(
    button_query: Query<InteractionAndBackgroundColor, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    interact_with_button(button_query, || next_app_state.set(AppState::InGame))
}
