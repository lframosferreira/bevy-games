use super::resources::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const NUMBER_SIZE: f32 = 80.0;

pub fn is_outside_grid(pos: (i32, i32)) -> bool {
    pos.0 < 0 || pos.0 > 3 || pos.1 < 0 || pos.1 > 3
}

pub fn get_starting_grid(number_of_shuffles: u32) -> ([[i32; 4]; 4], (i32, i32)) {
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
    (matrix, empty_pos)
}

pub fn spawn_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_status: Res<GridStatus>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    for i in 0..4 {
        for j in 0..4 {
            let transform_x: f32 = window.width() * (i as f32 / 4.0) + window.width() / 8.0;
            let transform_y: f32 = window.height() * (j as f32 / 4.0) + window.height() / 8.0;
            commands.spawn((Text2dBundle {
                text: Text::from_section(
                    grid_status.matrix[i][j].to_string(),
                    TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: NUMBER_SIZE,
                    },
                ),
                transform: Transform::from_xyz(transform_x, transform_y, 0.0),
                ..default()
            },));
        }
    }
}

pub fn handle_movement(block_query: Query<&Transform>, mut grid_status: ResMut<GridStatus>) {
    
}

pub fn insert_grid_status(mut commands: Commands) {
    commands.insert_resource(GridStatus::default());
}

pub fn reset_grid_status(mut grid_status: ResMut<GridStatus>) {
    let (matrix, empty_pos) = get_starting_grid(50);
    grid_status.matrix = matrix;
    grid_status.empty_pos = empty_pos;
}
