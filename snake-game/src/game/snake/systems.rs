use crate::game::snake::components::*;
use crate::game::snake::SNAKE_BODY_PART_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_snake(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    let shape = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(SNAKE_BODY_PART_SIZE),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::BLUE, 10.0),
        Snake::default(),
    ));
}
