use super::{
    components::{BombNumber, CellButton, Overlay},
    resources::{Grid, Visited},
    BOMB, GRID_SIZE, NUM_BOMBS,
};
use bevy::prelude::*;
use common::{events::EndGame, AppState};

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

pub fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let new_grid = Grid::default();
    commands.insert_resource(new_grid);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = BLOCK_OFFSET + j as f32 * (BLOCK_SIZE + GAP_SIZE);
            let y = BLOCK_OFFSET + i as f32 * (BLOCK_SIZE + GAP_SIZE);
            let number = if new_grid.0[i][j] >= BOMB {
                BOMB - 1
            } else {
                new_grid.0[i][j]
            };
            let text = if number == BOMB - 1 {
                " ".to_string()
            } else {
                format!("{}", number)
            };
            if new_grid.0[i][j] != 0 {
                commands.spawn((
                    Text2dBundle {
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
                    },
                    BombNumber,
                ));
            }
        }
    }
}

pub fn despawn_grid(mut commands: Commands, query: Query<Entity, With<BombNumber>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_overlay(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            Overlay,
        ))
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

pub fn despawn_overlay(mut commands: Commands, overlay_query: Query<Entity, With<Overlay>>) {
    if let Ok(entity) = overlay_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

type InteractionBackgroundButton<'a> =
    (&'a Interaction, &'a mut BackgroundColor, &'a mut CellButton);

pub fn interact_with_overlay(
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
    mut button_query: Query<(&mut CellButton, &mut Visibility)>,
    visited: Res<Visited>,
    grid: Res<Grid>,
    mut game_over_event_writer: EventWriter<EndGame>,
    mut commands: Commands,
) {
    if visited.is_changed() && !visited.is_added() {
        let mut count_unrevealed = 0;
        for (mut button, mut visibility) in button_query.iter_mut() {
            if visited.0[button.1][button.2] {
                button.0 = true;
                if grid.0[button.1][button.2] >= BOMB {
                    commands.insert_resource(NextState(Some(AppState::GameOver)));
                    game_over_event_writer.send(EndGame::new_bool(false));
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
            if *visibility != Visibility::Hidden {
                count_unrevealed += 1;
            }
        }
        if count_unrevealed == NUM_BOMBS {
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(EndGame::new_bool(true));
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
