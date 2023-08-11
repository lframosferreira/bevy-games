use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Component)]
pub struct Alien;

#[derive(Component)]
pub struct AlienLaser;

#[derive(Component)]
pub struct HitPoints(usize);

impl HitPoints {
    pub fn new(points: usize) -> Self {
        HitPoints(points)
    }

    pub fn hit(&mut self) {
        self.0 -= 1
    }

    pub fn increment(&mut self) {
        self.0 += 1
    }

    pub fn points(&self) -> usize {
        self.0
    }
}
