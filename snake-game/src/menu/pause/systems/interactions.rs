use crate::menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::{menu::pause::components::ResumeButton, AppState};
use bevy::prelude::*;

type WorldQuery<'a> = (&'a Interaction, &'a mut BackgroundColor);

pub fn interact_with_resume_button(
    mut button_query: Query<WorldQuery, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::InGame);
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
