use bevy::prelude::*;

#[derive(Resource)]
pub struct VehicleSpawnTimer(pub Vec<Timer>);
