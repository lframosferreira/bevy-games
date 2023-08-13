use bevy::prelude::*;
use ndarray::prelude::*;
use rand::prelude::*;

pub fn is_outside_grid(pos: (i8, i8)) -> bool {
    true
}

pub fn get_starting_grid(number_of_shuffles: u32) -> Array2<u8> {
    let mut matrix: Array2<u8> = Array::from_shape_vec(
        (4, 4),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
    )
    .unwrap();
    let mut empty_pos: (u8, u8) = (0, 0);
    for _ in 0..number_of_shuffles {
        let movements = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        movements.shuffle(&mut thread_rng());
        for movement in movements.iter() {}
    }
    println!("{:?}", matrix);
    matrix
}

#[derive(Resource)]
pub struct GridStatus {
    pub matrix: Array2<u8>,
}

impl Default for GridStatus {
    fn default() -> Self {
        Self {
            matrix: get_starting_grid(20),
        }
    }
}
