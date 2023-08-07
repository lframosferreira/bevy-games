use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Component)]
pub struct Obstacle {
    pub kind: ObstacleKind,
}

pub enum ObstacleKind {
    Pterodactyl,
    CactusSmall,
    CactusLarge,
}

impl Distribution<ObstacleKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ObstacleKind {
        match rng.gen_range(0..3) {
            0 => ObstacleKind::CactusSmall,
            1 => ObstacleKind::CactusLarge,
            _ => ObstacleKind::Pterodactyl,
        }
    }
}
