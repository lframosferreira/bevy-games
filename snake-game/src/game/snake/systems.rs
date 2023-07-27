use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(spawn_rect(
        window.width() / 2.,
        window.height() / 2.,
        400.,
        40.,
        Color::GREEN,
    ));

    commands.spawn(spawn_rect(
        window.width() / 2. + 200. - 8.,
        window.height() / 2. + 15.,
        15.,
        4.,
        Color::BLACK,
    ));

    commands.spawn(spawn_rect(
        window.width() / 2. + 200. - 8.,
        window.height() / 2. - 15.,
        15.,
        4.,
        Color::BLACK,
    ));

    commands.spawn(spawn_rect(
        window.width() / 2. + 200. + 12.,
        window.height() / 2.,
        24.,
        7.,
        Color::RED,
    ));
}

/// Spawn a rectangle with the given parameters
/// See <https://bevyengine.org/examples/2D%20Rendering/2d-shapes/>
fn spawn_rect(x: f32, y: f32, length: f32, width: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(length, width)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
        ..default()
    }
}
