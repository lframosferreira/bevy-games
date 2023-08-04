use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use super::components::Dinosaur;
use super::resources::DinoVerticalMovement;
use super::{
    DINO_HEIGHT, DINO_INITIAL_VERTICAL_SPEED, DINO_INITIAL_Y_POS, DINO_WIDTH, DINO_X_POS, GRAVITY,
};
use crate::game::floor::components::Floor;
use crate::game::floor::FLOOR_HEIGHT;

pub fn spawn_dinosaur(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0.2, 0.2),
                custom_size: Some(Vec2::new(DINO_WIDTH, DINO_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(DINO_X_POS, DINO_INITIAL_Y_POS, 0.0),
            ..default()
        },
        Dinosaur {},
    ));
}

pub fn handle_jump(
    mut dinosaur_query: Query<&mut Transform, With<Dinosaur>>,
    floor_query: Query<&Transform, (With<Floor>, Without<Dinosaur>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut dino_vertical_movement: ResMut<DinoVerticalMovement>,
) {
    let window: &Window = window_query.get_single().unwrap();
    if let Ok(floor_transform) = floor_query.get_single() {
        if let Ok(mut dinosaur_transform) = dinosaur_query.get_single_mut() {
            if dino_vertical_movement.moving {
                if collide(
                    floor_transform.translation,
                    Vec2::new(window.width(), FLOOR_HEIGHT),
                    dinosaur_transform.translation,
                    Vec2::new(DINO_WIDTH, DINO_HEIGHT),
                )
                .is_some()
                {
                    dino_vertical_movement.moving = false;
                    dino_vertical_movement.speed = DINO_INITIAL_VERTICAL_SPEED;
                } else {
                    dinosaur_transform.translation.y += dino_vertical_movement.speed;
                    dino_vertical_movement.speed += GRAVITY;
                }
            } else {
                if keyboard_input.pressed(KeyCode::Space) {
                    dino_vertical_movement.moving = true;
                    dinosaur_transform.translation.y += dino_vertical_movement.speed;
                    dino_vertical_movement.speed += GRAVITY;
                }
            }
        }
    }
}
