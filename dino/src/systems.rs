use crate::game::dinosaur::components::Dinosaur;
use crate::game::dinosaur::DINO_INITIAL_Y_POS;
use crate::game::obstacle::{
    components::Obstacle, resources::ObstacleSpeed, OBSTACLE_INITIAL_SPEED,
};
use crate::game::score::resources::Score;
use bevy::prelude::*;

pub fn reset_score(mut score: ResMut<Score>) {
    score.value = 0
}

pub fn reset_obstacle_speed(mut obstacle_speed: ResMut<ObstacleSpeed>) {
    obstacle_speed.speed = OBSTACLE_INITIAL_SPEED;
}

pub fn despawn_obstacles(
    mut commands: Commands,
    mut obstacle_query: Query<Entity, With<Obstacle>>,
) {
    for obstacle_entity in obstacle_query.iter_mut() {
        commands.entity(obstacle_entity).despawn();
    }
}

pub fn set_dinosaur_in_initial_position(
    mut dinosaur_query: Query<&mut Transform, With<Dinosaur>>,
) {
    if let Ok(mut dinosaur_transform) = dinosaur_query.get_single_mut() {
        dinosaur_transform.translation.y = DINO_INITIAL_Y_POS;
    }
}
