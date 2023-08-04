use bevy::prelude::*;

#[derive(Component)]
pub struct Obstacle {
    pub kind: ObstacleKind,
    pub height: f32
}

pub enum ObstacleKind {
    Pterodactyl,
    Cactus
}