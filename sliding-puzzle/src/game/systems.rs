use super::resources::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const NUMBER_SIZE: f32 = 80.0;

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

pub fn insert_grid_status(mut commands: Commands) {
    commands.insert_resource(GridStatus::default());
}

pub fn reset_grid_status(mut grid_status: ResMut<GridStatus>) {
    grid_status.matrix = get_starting_grid(50);
}
