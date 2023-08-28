use super::{
    components::{Block, Buttons, ColorIndexer, Heart},
    resources::Lives,
    BLOCK_SIZE, COLORS_NORMAL, MAX_LIVES, SIZE, WINDOW_Y,
};
use bevy::prelude::*;
use common::{events::EndGame, AppState};

const NUM_BLOCKS: usize = 16;

pub fn spawn_blocks(mut commands: Commands) {
    const X_OFFSET: f32 = 4. * BLOCK_SIZE - (BLOCK_SIZE / 2.);
    const Y_OFFSET: f32 = 6. * BLOCK_SIZE - (BLOCK_SIZE / 2.);

    for i in 0..NUM_BLOCKS {
        for j in 0..NUM_BLOCKS {
            let block = Block::new(i, j);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: block.0,
                        custom_size: Some(SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        i as f32 * BLOCK_SIZE + X_OFFSET,
                        j as f32 * BLOCK_SIZE + Y_OFFSET,
                        0.,
                    ),
                    ..default()
                },
                block,
            ));
        }
    }
}

pub fn respawn_blocks(mut commands: Commands, block_query: Query<Entity, With<Block>>) {
    for entity in block_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_blocks(commands);
}

pub fn spawn_life_bar(mut commands: Commands) {
    const X_OFFSET: f32 = 2. * BLOCK_SIZE - (BLOCK_SIZE / 2.);
    const Y_OFFSET: f32 = 2. * BLOCK_SIZE - (BLOCK_SIZE / 2.);
    const LIFE_SIZE: Vec2 = Vec2::new(2. * BLOCK_SIZE, BLOCK_SIZE);
    for i in 0..MAX_LIVES {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    custom_size: Some(LIFE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(X_OFFSET, i as f32 * BLOCK_SIZE + Y_OFFSET, 0.),
                ..default()
            },
            Heart(),
        ));
    }
}

pub fn spawn_buttons(mut commands: Commands) {
    const Y_OFFSET: f32 = (NUM_BLOCKS as f32 + 1.) * BLOCK_SIZE + 35.;
    const X_OFFSET: f32 = 3. * BLOCK_SIZE;
    const BUTTON_SIZE: f32 = 80.;
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    width: Val::Px(BUTTON_SIZE * 6.),
                    height: Val::Px(BUTTON_SIZE),
                    top: Val::Px(Y_OFFSET),
                    left: Val::Px(X_OFFSET),
                    ..default()
                },
                ..default()
            },
            Buttons,
        ))
        .with_children(|parent| {
            for (i, &color) in COLORS_NORMAL.iter().enumerate() {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(BUTTON_SIZE),
                            height: Val::Px(BUTTON_SIZE),
                            ..default()
                        },
                        background_color: color.into(),
                        ..default()
                    },
                    ColorIndexer(i),
                ));
            }
        });
}

pub fn interact_with_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &ColorIndexer),
        Changed<Interaction>,
    >,
    mut block_query: Query<(&mut Block, &mut Sprite)>,
    mut lives: ResMut<Lives>,
) {
    for (interaction, mut background_color, indexer) in button_query.iter_mut() {
        let index = indexer.0;
        const COLOR_RATIO: f32 = 1.25;
        let colors_hover: [Color; 6] = core::array::from_fn(|i| COLORS_NORMAL[i] * COLOR_RATIO);

        match *interaction {
            Interaction::Pressed => {
                if update_color(&mut block_query, COLORS_NORMAL[index]) {
                    lives.decrement();
                }
            }
            Interaction::Hovered => {
                *background_color = colors_hover[index].into();
            }
            Interaction::None => {
                *background_color = COLORS_NORMAL[index].into();
            }
        }
    }
}

pub fn despawn_buttons(mut commands: Commands, buttons_query: Query<Entity, With<Buttons>>) {
    for entity in buttons_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_win(
    block_query: Query<&Block>,
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<EndGame>,
) {
    if let Some(head) = block_query.iter().next() {
        for block in block_query.iter() {
            if block.0 != head.0 {
                return;
            }
        }

        commands.insert_resource(NextState(Some(AppState::GameOver)));
        game_over_event_writer.send(EndGame::new_number(NUM_BLOCKS * NUM_BLOCKS));
    }
}

fn calculate_score(block_query: Query<&Block>) -> usize {
    let color_matrix = block_query.iter().fold(
        [[Color::NONE; MAX_LIVES]; MAX_LIVES],
        |mut matrix, block| {
            matrix[block.1][block.2] = block.0;
            matrix
        },
    );
    let mut modify: Vec<(usize, usize)> = Vec::new();
    let mut visited = [[false; MAX_LIVES]; MAX_LIVES];

    dfs(&mut visited, &mut modify, &color_matrix);

    let mut score = 0;
    if let Some(head) = block_query.iter().next() {
        for i in 0..visited.len() {
            for j in 0..visited.len() {
                if visited[i][j] && color_matrix[i][j] == head.0 {
                    score += 1;
                }
            }
        }
    }
    score
}

pub fn take_life(
    mut commands: Commands,
    lives: Res<Lives>,
    heart_query: Query<Entity, With<Heart>>,
    mut game_over_event_writer: EventWriter<EndGame>,
    block_query: Query<&Block>,
) {
    if lives.is_changed() && !lives.is_added() {
        if let Some(entity) = heart_query.iter().last() {
            commands.entity(entity).despawn();
        } else {
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(EndGame::new_number(calculate_score(block_query)));
        }
    }
}

fn update_color(block_query: &mut Query<(&mut Block, &mut Sprite)>, color: Color) -> bool {
    let color_matrix = block_query.iter().fold(
        [[Color::NONE; MAX_LIVES]; MAX_LIVES],
        |mut matrix, block| {
            matrix[block.0 .1][block.0 .2] = block.0 .0;
            matrix
        },
    );
    if color_matrix[0][0] != color {
        let mut modify: Vec<(usize, usize)> = Vec::new();
        let mut visited = [[false; MAX_LIVES]; MAX_LIVES];

        dfs(&mut visited, &mut modify, &color_matrix);

        for (mut block, mut sprite) in block_query.iter_mut() {
            for i in modify.iter() {
                if block.1 == i.0 && block.2 == i.1 {
                    block.0 = color;
                    sprite.color = color;
                }
            }
        }
        return true;
    }
    false
}

fn dfs(
    visited: &mut [[bool; MAX_LIVES]; MAX_LIVES],
    modifiable: &mut Vec<(usize, usize)>,
    color_matrix: &[[Color; MAX_LIVES]; MAX_LIVES],
) {
    _dfs(
        &Block(color_matrix[0][0], 0, 0),
        visited,
        modifiable,
        color_matrix,
    );
}

fn _dfs(
    block: &Block,
    visited: &mut [[bool; MAX_LIVES]; MAX_LIVES],
    modifiable: &mut Vec<(usize, usize)>,
    color_matrix: &[[Color; MAX_LIVES]; MAX_LIVES],
) {
    if visited[block.1][block.2] {
        return;
    }
    visited[block.1][block.2] = true;

    if block.0 != color_matrix[0][0] {
        return;
    } else {
        modifiable.push((block.1, block.2));
    }

    if block.1 != 0 {
        _dfs(
            &Block(color_matrix[block.1 - 1][block.2], block.1 - 1, block.2),
            visited,
            modifiable,
            color_matrix,
        );
    }
    if block.2 != 0 {
        _dfs(
            &Block(color_matrix[block.1][block.2 - 1], block.1, block.2 - 1),
            visited,
            modifiable,
            color_matrix,
        );
    }
    if block.1 != NUM_BLOCKS - 1 {
        _dfs(
            &Block(color_matrix[block.1 + 1][block.2], block.1 + 1, block.2),
            visited,
            modifiable,
            color_matrix,
        );
    }
    if block.2 != NUM_BLOCKS - 1 {
        _dfs(
            &Block(color_matrix[block.1][block.2 + 1], block.1, block.2 + 1),
            visited,
            modifiable,
            color_matrix,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _dfs_test() {
        let mut visited = [[false; MAX_LIVES]; MAX_LIVES];
        let mut modify: Vec<(usize, usize)> = Vec::new();

        let mut matrix = [[Color::NONE; MAX_LIVES]; MAX_LIVES];
        matrix[0][0] = Color::RED;
        matrix[0][1] = Color::RED;
        matrix[0][2] = Color::RED;
        matrix[1][0] = Color::RED;
        matrix[2][2] = Color::RED;

        dfs(&mut visited, &mut modify, &matrix);

        assert_eq!(modify, vec![(0, 0), (1, 0), (0, 1), (0, 2)])
    }
}
