use bevy::prelude::*;
use rand::prelude::*;

pub fn is_outside_grid(pos: (i32, i32)) -> bool {
    pos.0 < 0 || pos.0 > 3 || pos.1 < 0 || pos.1 > 3
}

pub fn get_starting_grid(number_of_shuffles: u32) -> [[i32; 4]; 4] {
    let mut matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];
    let mut empty_pos: (i32, i32) = (3, 3);
    for _ in 0..number_of_shuffles {
        let mut movements = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        movements.shuffle(&mut thread_rng());
        for movement in movements.iter() {
            let future_pos = (empty_pos.0 + movement.0, empty_pos.1 + movement.1);
            if !is_outside_grid(future_pos) {
                matrix[empty_pos.0 as usize][empty_pos.1 as usize] =
                    matrix[future_pos.0 as usize][future_pos.1 as usize];
                matrix[future_pos.0 as usize][future_pos.1 as usize] = 0;
                empty_pos = future_pos;
                break;
            }
        }
    }
    matrix
}

#[derive(Resource)]
pub struct GridStatus {
    pub matrix: [[i32; 4]; 4],
}

impl Default for GridStatus {
    fn default() -> Self {
        Self {
            matrix: get_starting_grid(50),
        }
    }
}
