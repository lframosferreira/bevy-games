use super::resources::*;
use crate::game::{AppState, EndGame};
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
                empty_pos = future_pos.clone();
                break;
            }
        }
    }
    (matrix, empty_pos)
}

#[derive(Component)]
pub struct Block {
    pub value: i32,
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
            let transform_x: f32 = window.width() * (j as f32 / 4.0) + window.width() / 8.0;
            let transform_y: f32 = window.height() * ((3 - i) as f32 / 4.0) + window.height() / 8.0;
            if grid_status.matrix[i][j] != 0 {
                commands.spawn((
                    Text2dBundle {
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
                    },
                    Block {
                        value: grid_status.matrix[i][j],
                    },
                ));
            }
        }
    }
}

pub fn handle_movement(
    mut block_query: Query<&mut Transform>,
    mut grid_status: ResMut<GridStatus>,
    keyboard_input: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let empty_pos_x: i32 = grid_status.empty_pos.1;
    let empty_pos_y: i32 = grid_status.empty_pos.0;
    let mut future_pos: Option<(i32, i32)> = None;
    if keyboard_input.just_pressed(KeyCode::Up) && !is_outside_grid((empty_pos_x, empty_pos_y + 1))
    {
        future_pos = Some((empty_pos_x, empty_pos_y + 1));
    } else if keyboard_input.just_pressed(KeyCode::Down)
        && !is_outside_grid((empty_pos_x, empty_pos_y - 1))
    {
        future_pos = Some((empty_pos_x, empty_pos_y - 1));
    } else if keyboard_input.just_pressed(KeyCode::Right)
        && !is_outside_grid((empty_pos_x - 1, empty_pos_y))
    {
        future_pos = Some((empty_pos_x - 1, empty_pos_y));
    } else if keyboard_input.just_pressed(KeyCode::Left)
        && !is_outside_grid((empty_pos_x + 1, empty_pos_y))
    {
        future_pos = Some((empty_pos_x + 1, empty_pos_y));
    }
    let window: &Window = window_query.get_single().unwrap();
    if let Some(pos) = future_pos {
        let transform_x: f32 = window.width() * (pos.0 as f32 / 4.0) + window.width() / 8.0;
        let transform_y: f32 = window.height() * ((3 - pos.1) as f32 / 4.0) + window.height() / 8.0;
        if let Some(mut block_transform) = block_query.iter_mut().find(|block_transform| {
            block_transform.translation.x == transform_x
                && block_transform.translation.y == transform_y
        }) {
            let new_transform_x: f32 =
                window.width() * (empty_pos_x as f32 / 4.0) + window.width() / 8.0;
            let new_transform_y: f32 =
                window.height() * ((3 - empty_pos_y) as f32 / 4.0) + window.height() / 8.0;
            block_transform.translation.x = new_transform_x;
            block_transform.translation.y = new_transform_y;
            let aux = grid_status.matrix[pos.1 as usize][pos.0 as usize];
            grid_status.matrix[pos.1 as usize][pos.0 as usize] = 0;
            grid_status.matrix[empty_pos_y as usize][empty_pos_x as usize] = aux;
            grid_status.empty_pos = (pos.1, pos.0);
        }
    }
}

pub fn reset_grid_status(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut grid_status: ResMut<GridStatus>,
    asset_server: Res<AssetServer>,
    mut block_query: Query<Entity, With<Block>>,
) {
    println!("oi");
    let (matrix, empty_pos) = get_starting_grid(500);
    grid_status.matrix = matrix;
    grid_status.empty_pos = empty_pos;
    let window: &Window = window_query.get_single().unwrap();
    for block_entity in block_query.iter_mut() {
        commands.entity(block_entity).despawn();
    }
    for i in 0..4 {
        for j in 0..4 {
            let transform_x: f32 = window.width() * (j as f32 / 4.0) + window.width() / 8.0;
            let transform_y: f32 = window.height() * ((3 - i) as f32 / 4.0) + window.height() / 8.0;
            if grid_status.matrix[i][j] != 0 {
                commands.spawn((
                    Text2dBundle {
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
                    },
                    Block {
                        value: grid_status.matrix[i][j],
                    },
                ));
            }
        }
    }
}

pub fn check_win(
    mut commands: Commands,
    grid_status: Res<GridStatus>,
    mut game_over_event_writer: EventWriter<EndGame>,
) {
    if grid_status.matrix == [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]] {
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        game_over_event_writer.send(EndGame { score: 0 });
    }
}
