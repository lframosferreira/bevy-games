use bevy::prelude::Component;

#[derive(Component)]
pub struct Overlay;

#[derive(Component)]
pub struct BombNumber;

#[derive(Component)]
pub struct CellButton(pub bool, pub usize, pub usize);

impl CellButton {
    pub fn new(x: usize, y: usize) -> Self {
        Self(false, x, y)
    }
}
