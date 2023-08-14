use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Score {
    pub value: usize,
}
