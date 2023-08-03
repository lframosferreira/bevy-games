use bevy::prelude::Event;

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}
