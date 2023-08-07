use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Component)]
pub struct Obstacle {
    pub kind: ObstacleKind,
}

pub enum ObstacleKind {
    Pterodactyl,
    Cactus,
}

impl Distribution<ObstacleKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ObstacleKind {
        match rng.gen_range(0..2) {
            0 => ObstacleKind::Cactus,
            _ => ObstacleKind::Pterodactyl,
        }
    }
}
