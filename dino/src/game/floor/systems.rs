use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Floor;
use super::FLOOR_HEIGHT;

pub fn spawn_floor(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.50),
                custom_size: Some(Vec2::new(window.width(), FLOOR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., FLOOR_HEIGHT / 2.0, 0.0),
            ..default()
        },
        Floor {},
    ));
}
