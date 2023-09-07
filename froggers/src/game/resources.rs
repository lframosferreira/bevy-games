use bevy::prelude::*;

#[derive(Resource)]
pub struct VehicleSpawnTimer(pub Vec<Timer>);

#[derive(Resource)]
pub struct RoundTimer(pub Timer);

#[derive(Resource, Default)]
pub struct MaxHeight(pub f32);

impl MaxHeight {
    pub fn reset(&mut self) {
        self.0 = 0.;
    }
}

#[derive(Resource, Default)]
pub struct HitLeftOverFrogs(pub bool);

#[derive(Resource, Default)]
pub struct HitHaven(pub bool);
