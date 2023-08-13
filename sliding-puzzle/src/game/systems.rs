use super::resources::*;
use bevy::prelude::*;

const NUMBER_SIZE: f32 = 80.0;

pub fn spawn_blocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Text2dBundle {
        text: Text::from_section(
            "12",
            TextStyle {
                color: Color::WHITE,
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: NUMBER_SIZE,
            },
        ),
        transform: Transform::from_xyz(100.0, 100.0, 0.0),
        ..default()
    },));
}

pub fn insert_grid_status(mut commands: Commands) {
    commands.insert_resource(GridStatus::default());
}

pub fn reset_grid_status(mut grid_status: ResMut<GridStatus>) {
    grid_status.matrix = get_starting_grid(20);
}
