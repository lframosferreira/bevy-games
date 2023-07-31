use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::Fruit;
use super::FRUIT_SIZE;

pub fn spawn_fruit(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();
    let random_x_index: f32 = rng.gen_range(0..((window.width() / FRUIT_SIZE) as u32)) as f32;
    let fruit_x_pos: f32 = random_x_index * FRUIT_SIZE + FRUIT_SIZE / 2.0;
    let random_y_index: f32 = rng.gen_range(0..((window.height() / FRUIT_SIZE) as u32)) as f32;
    let fruit_y_pos: f32 = random_y_index * FRUIT_SIZE + FRUIT_SIZE / 2.0;
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
