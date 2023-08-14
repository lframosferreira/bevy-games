use bevy::prelude::Component;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Pipe {
    pub height: f32,
    pub behind: bool,
}
