use super::systems::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GridStatus {
    pub matrix: [[i32; 4]; 4],
    pub empty_pos: (i32, i32),
}

impl Default for GridStatus {
    fn default() -> Self {
        let (matrix, empty_pos) = get_starting_grid(50);
        println!("{:?}", matrix);
        println!("{:?}", empty_pos);
        Self {
            matrix: matrix,
            empty_pos: empty_pos,
        }
    }
}
