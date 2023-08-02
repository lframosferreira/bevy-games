use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}
