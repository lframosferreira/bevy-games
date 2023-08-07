use bevy::prelude::Resource;

const DECREASE_AMOUNT: f32 = 20.;
const MAX_SPEED: f32 = 500.;

#[derive(Resource, Default)]
pub struct Gravity {
    pub speed: f32,
}

impl Gravity {
    pub fn decrease(&mut self) {
        self.speed -= DECREASE_AMOUNT
    }
    pub fn set_max(&mut self) {
        self.speed = MAX_SPEED
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}
