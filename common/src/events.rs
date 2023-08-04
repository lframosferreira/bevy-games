use bevy::prelude::Event;

#[derive(Event)]
pub struct EndGame {
    pub score: u32,
}
