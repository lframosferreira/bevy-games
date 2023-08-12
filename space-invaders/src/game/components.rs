use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Stats {
    width: f32,
    height: f32,
    size: Vec2,
    speed: f32,
    color: Color,
}

impl Stats {
    pub fn new(width: f32, height: f32, size: Vec2, speed: f32, color: Color) -> Self {
        Self {
            width,
            height,
            size,
            speed,
            color,
        }
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn size(&self) -> Vec2 {
        self.size
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Component)]
pub struct HitPoints(usize);

impl HitPoints {
    pub fn new(points: usize) -> Self {
        HitPoints(points)
    }

    pub fn hit(&mut self) {
        self.0 -= 1
    }

    pub fn points(&self) -> usize {
        self.0
    }
}

#[derive(Component)]
pub struct ScoreHUD;

#[derive(Component)]
pub struct LivesHUD;
