use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::Fruit;

pub fn spawn_fruit(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let fruit_x_pos: f32 = random::<f32>() * window.width();
    let fruit_y_pos: f32 = random::<f32>() * window.height();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(fruit_x_pos, fruit_y_pos, 0.0),
            texture: asset_server.load("sprites/fruit.png"),
            ..default()
        },
        Fruit {
            x_pos: fruit_x_pos,
            y_pos: fruit_y_pos,
        },
    ));
}
