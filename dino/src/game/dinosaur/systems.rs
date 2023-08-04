use bevy::prelude::*;

use super::components::Dinosaur;
use super::resources::DinoVerticalMovement;
use super::{
    DINO_HEIGHT, DINO_INITIAL_VERTICAL_SPEED, DINO_INITIAL_Y_POS, DINO_WIDTH, DINO_X_POS, GRAVITY,
};

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
    keyboard_input: Res<Input<KeyCode>>,
    mut dino_vertical_movement: ResMut<DinoVerticalMovement>,
) {
    if let Ok(mut dinosaur_transform) = dinosaur_query.get_single_mut() {
        if dino_vertical_movement.moving {
            if dinosaur_transform.translation.y < DINO_INITIAL_Y_POS {
                dino_vertical_movement.moving = false;
                dino_vertical_movement.speed = DINO_INITIAL_VERTICAL_SPEED;
                dinosaur_transform.translation.y = DINO_INITIAL_Y_POS;
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
