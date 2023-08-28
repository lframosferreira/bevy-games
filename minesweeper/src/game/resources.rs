use super::{BOMB, GRID_SIZE, NUM_BOMBS};
use bevy::prelude::Resource;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Resource, Default)]
pub struct Visited(pub [[bool; GRID_SIZE]; GRID_SIZE]);

#[derive(Resource, Clone, Copy)]
pub struct Grid(pub [[usize; GRID_SIZE]; GRID_SIZE]);

impl Grid {
    fn gen_grid() -> [[usize; GRID_SIZE]; GRID_SIZE] {
        let grid_flattened: [usize; GRID_SIZE * GRID_SIZE] = core::array::from_fn(|i| i);
        let bomb_indexes = grid_flattened.choose_multiple(&mut thread_rng(), NUM_BOMBS);
        let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
        // OMG dealing with usize sucks ass
        for bomb in bomb_indexes {
            let row = bomb / GRID_SIZE;
            let col = bomb % GRID_SIZE;
            grid[row][col] = BOMB;
            let i_start = if row > 0 { row - 1 } else { 0 };
            let i_end = if row < GRID_SIZE - 1 {
                row + 1
            } else {
                GRID_SIZE - 1
            };
            let j_start = if col > 0 { col - 1 } else { 0 };
            let j_end = if col < GRID_SIZE - 1 {
                col + 1
            } else {
                GRID_SIZE - 1
            };
            (i_start..=i_end).for_each(|i| {
                (j_start..=j_end).for_each(|j| {
                    if i != row || j != col {
                        grid[i][j] += 1;
                    }
                });
            });
        }
        grid
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self(Grid::gen_grid())
    }
}
