use super::components::Fruit;
use crate::game::BLOCK_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub fn spawn_fruit(commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    {
        let mut commands = commands;
        let window: &Window = window_query.get_single().unwrap();
        let mut rng = rand::thread_rng();
        let random_x_index: f32 = rng.gen_range(0..((window.width() / BLOCK_SIZE) as u32)) as f32;
        let fruit_x_pos: f32 = random_x_index * BLOCK_SIZE + BLOCK_SIZE / 2.0;
        let random_y_index: f32 = rng.gen_range(0..((window.height() / BLOCK_SIZE) as u32)) as f32;
        let fruit_y_pos: f32 = random_y_index * BLOCK_SIZE + BLOCK_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 0.2, 0.2),
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(fruit_x_pos, fruit_y_pos, 0.0),
                ..default()
            },
            Fruit {
                x_pos: fruit_x_pos,
                y_pos: fruit_y_pos,
            },
        ));
    };
}

pub fn respawn_fruit(
    mut commands: Commands,
    fruit_query: Query<Entity, With<Fruit>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(entity) = fruit_query.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_fruit(commands, window_query);
}
