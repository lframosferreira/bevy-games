use bevy::prelude::*;

#[derive(Component)]
pub struct Frog;

#[derive(Component)]
pub struct Lake;

#[derive(Component, Clone, Copy)]
pub struct Vehicle {
    pub width: f32,
    pub moves_to_left: bool,
    pub is_harmful: bool,
    pub color: Color,
    pub speed: f32,
    pub timer_seconds: f32,
}

impl Vehicle {
    pub const fn new(
        width: f32,
        moves_to_left: bool,
        is_harmful: bool,
        color: Color,
        speed: f32,
        timer_seconds: f32,
    ) -> Self {
        Self {
            width,
            moves_to_left,
            color,
            speed,
            timer_seconds,
            is_harmful,
        }
    }

    const SIZE_HEIGHT: f32 = 45.;

    pub const fn size(&self) -> Vec2 {
        Vec2::new(self.width, Vehicle::SIZE_HEIGHT)
    }
}
