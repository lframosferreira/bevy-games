use super::{
    components::CellButton,
    resources::{Grid, Visited},
    BOMB, GRID_SIZE,
};
use bevy::prelude::*;

const BLOCK_SIZE: f32 = 60.;
const BLOCK_OFFSET: f32 = BLOCK_SIZE / 2.;
const GAP_SIZE: f32 = 0.;
const COLORS: [Color; 9] = [
    Color::AQUAMARINE,
    Color::TURQUOISE,
    Color::CYAN,
    Color::TEAL,
    Color::SEA_GREEN,
    Color::LIME_GREEN,
    Color::DARK_GREEN,
    Color::GOLD,
    Color::BLACK,
];

pub fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<Grid>) {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = BLOCK_OFFSET + j as f32 * (BLOCK_SIZE + GAP_SIZE);
            let y = BLOCK_OFFSET + i as f32 * (BLOCK_SIZE + GAP_SIZE);
            let number = if grid.0[i][j] >= BOMB {
                BOMB - 1
            } else {
                grid.0[i][j]
            };
            let text = if number == BOMB - 1 {
                " ".to_string()
            } else {
                format!("{}", number)
            };
            if grid.0[i][j] != 0 {
                commands.spawn(Text2dBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/IosevkaNerdFont-Bold.ttf"),
                            font_size: 60.0,
                            color: COLORS[number],
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..default()
                });
            }
        }
    }
}

pub fn build_overlay(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for i in 0..GRID_SIZE {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for j in 0..GRID_SIZE {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(BLOCK_SIZE),
                                        height: Val::Px(BLOCK_SIZE),
                                        ..Style::DEFAULT
                                    },
                                    background_color: Color::DARK_GRAY.into(),
                                    ..default()
                                },
                                CellButton::new(j, i),
                            ));
                        }
                    });
            }
        });
}

type InteractionBackgroundButton<'a> =
    (&'a Interaction, &'a mut BackgroundColor, &'a mut CellButton);

pub fn interact_with_button(
    mut button_query: Query<InteractionBackgroundButton, (Changed<Interaction>, With<CellButton>)>,
    grid: Res<Grid>,
    mut visited: ResMut<Visited>,
) {
    for (interaction, mut background_color, button) in button_query.iter_mut() {
        if !button.0 {
            // NOTE
            // We can't detect right-click button interactions
            // see https://github.com/bevyengine/bevy/issues/2322
            //
            // Therefore, we can't handle marks
            match *interaction {
                Interaction::Pressed => {
                    // TODO Handle clicking on Bomb
                    let mut visits = [[false; GRID_SIZE]; GRID_SIZE];
                    dfs((button.1, button.2), &mut visits, &grid.0);
                    visited.0 = visits;
                }
                Interaction::Hovered => {
                    *background_color = Color::WHITE.into();
                }
                Interaction::None => {
                    *background_color = Color::DARK_GRAY.into();
                }
            }
        }
    }
}

pub fn reveal(
    mut button_query: Query<(&mut CellButton, &mut BackgroundColor)>,
    visited: Res<Visited>,
) {
    if visited.is_changed() {
        for (mut button, mut bg) in button_query.iter_mut() {
            if visited.0[button.1][button.2] {
                *bg = Color::NONE.into();
                button.0 = true;
            }
        }
    }
}

fn dfs(
    position: (usize, usize),
    visited: &mut [[bool; GRID_SIZE]; GRID_SIZE],
    lattice: &[[usize; GRID_SIZE]; GRID_SIZE],
) {
    if visited[position.0][position.1] {
        return;
    }
    visited[position.0][position.1] = true;

    if lattice[position.0][position.1] != 0 {
        return;
    }

    if position.0 != 0 {
        dfs((position.0 - 1, position.1), visited, lattice);
    }
    if position.1 != 0 {
        dfs((position.0, position.1 - 1), visited, lattice);
    }
    if position.0 != GRID_SIZE - 1 {
        dfs((position.0 + 1, position.1), visited, lattice);
    }
    if position.1 != GRID_SIZE - 1 {
        dfs((position.0, position.1 + 1), visited, lattice);
    }
    if position.0 != 0 && position.1 != 0 {
        dfs((position.0 - 1, position.1 - 1), visited, lattice);
    }
    if position.0 != GRID_SIZE - 1 && position.1 != 0 {
        dfs((position.0 + 1, position.1 - 1), visited, lattice);
    }
    if position.0 != 0 && position.1 != GRID_SIZE - 1 {
        dfs((position.0 - 1, position.1 + 1), visited, lattice);
    }
    if position.0 != GRID_SIZE - 1 && position.1 != GRID_SIZE - 1 {
        dfs((position.0 + 1, position.1 + 1), visited, lattice);
    }
}
