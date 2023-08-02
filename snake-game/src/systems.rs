use crate::game::BLOCK_SIZE;
use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_debug_lines::*;

pub const WINDOW_X: f32 = 1000.0;
pub const WINDOW_Y: f32 = 600.0;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
        ..default()
    });
}

pub fn pause_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && app_state.get() != &AppState::Pause {
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

pub fn draw_grid(mut lines: ResMut<DebugLines>) {
    for i in 0..(WINDOW_X / BLOCK_SIZE) as u32 {
        lines.line(
            Vec3::new(BLOCK_SIZE * i as f32, 0., 0.),
            Vec3::new(BLOCK_SIZE * i as f32, WINDOW_Y, 0.),
            0.0,
        )
    }
    for i in 0..(WINDOW_Y / BLOCK_SIZE) as u32 {
        lines.line(
            Vec3::new(0., BLOCK_SIZE * i as f32, 0.),
            Vec3::new(WINDOW_X, BLOCK_SIZE * i as f32, 0.),
            0.0,
        )
    }
}
