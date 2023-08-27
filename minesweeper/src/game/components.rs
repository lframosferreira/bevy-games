use bevy::prelude::Component;

#[derive(Component)]
pub struct CellButton(pub bool, pub usize, pub usize);

impl CellButton {
    pub fn new(x: usize, y: usize) -> Self {
        Self(false, x, y)
    }
}
