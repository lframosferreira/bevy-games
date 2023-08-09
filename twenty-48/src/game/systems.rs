use super::{components::Block, BLOCK_LENGTH};
use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};

pub fn spawn_blocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let avalaible: Vec<usize> = (0..16).collect();
    let sample = avalaible.iter().choose_multiple(&mut thread_rng(), 2);
    for index in sample {
        commands.spawn(_spawn_block(Block::new_random(index), &asset_server));
    }
}

fn text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    }
}

fn _spawn_block(block: Block, asset_server: &Res<AssetServer>) -> (Text2dBundle, Block) {
    (
        Text2dBundle {
            text: Text::from_section(format!("{}", block.0), text_style(asset_server))
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(
                block.x() as f32 * BLOCK_LENGTH + BLOCK_LENGTH / 2.,
                block.y() as f32 * BLOCK_LENGTH + BLOCK_LENGTH / 2.,
                0.,
            ),
            ..default()
        },
        block,
    )
}

pub fn update_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut block_query: Query<(Entity, &mut Block, &mut Text), With<Block>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const KEYS: [KeyCode; 4] = [KeyCode::Left, KeyCode::Down, KeyCode::Right, KeyCode::Up];
    let was_pressed: Vec<bool> = KEYS
        .iter()
        .map(|key| keyboard_input.just_pressed(*key))
        .collect();
    let mut number_matrix = block_query.iter().fold([[0; 4]; 4], |mut matrix, block| {
        matrix[block.1.x()][block.1.y()] = block.1.number();
        matrix
    });
    if was_pressed.iter().any(|key| *key) {
        let original_matrix = number_matrix;
        _update_direction(&was_pressed, &mut number_matrix);
        if original_matrix != number_matrix {
            for (entity, mut block, mut text) in block_query.iter_mut() {
                let my_cool_number = number_matrix[block.x()][block.y()];
                if number_matrix[block.x()][block.y()] == 0 {
                    commands.entity(entity).despawn();
                } else if my_cool_number != block.number() {
                    block.set_number(my_cool_number);
                    *text = Text::from_section(format!("{}", block.0), text_style(&asset_server));
                }
            }
            let mut avalaible: Vec<usize> = (0..16).collect();
            for i in 0..number_matrix.len() {
                for j in 0..number_matrix[i].len() {
                    if number_matrix[i][j] != 0 {
                        avalaible.retain(|&x| x != 4 * i + j);
                    }
                    if number_matrix[i][j] != 0 && original_matrix[i][j] == 0 {
                        let block = Block::new(number_matrix[i][j], i, j);
                        commands.spawn(_spawn_block(block, &asset_server));
                    }
                }
            }
            let sample = avalaible.iter().choose(&mut thread_rng());
            if let Some(index) = sample {
                commands.spawn(_spawn_block(Block::new_random(index), &asset_server));
            }
        }
    }
}

fn _update_direction(was_pressed: &[bool], matrix: &mut [[usize; 4]; 4]) {
    let rows = if was_pressed[0] {
        [3, 2, 1, 0]
    } else {
        [0, 1, 2, 3]
    };
    let cols = if was_pressed[1] {
        [3, 2, 1, 0]
    } else {
        [0, 1, 2, 3]
    };
    let is_vertical = was_pressed[0] || was_pressed[2];
    let is_increment = was_pressed[0] || was_pressed[1];

    if was_pressed.iter().any(|key| *key) {
        for &i in rows.iter() {
            for &j in cols.iter() {
                let (x, y) = match common_update(matrix, i, j, is_vertical, is_increment) {
                    Some(value) => value,
                    None => continue,
                };
                if matrix[i][j] == matrix[x][y] {
                    matrix[x][y] *= 2;
                    matrix[i][j] = 0;
                }
            }
        }

        for &i in rows.iter() {
            for &j in cols.iter() {
                common_update(matrix, i, j, is_vertical, is_increment);
            }
        }
    }
}

fn common_update(
    matrix: &mut [[usize; 4]; 4],
    i: usize,
    j: usize,
    is_vertical: bool,
    is_increment: bool,
) -> Option<(usize, usize)> {
    if matrix[i][j] == 0 {
        return None;
    }
    let control = if is_vertical { i } else { j };
    if (is_increment && control == 0) || (!is_increment && control == 3) {
        return None;
    }
    let delta = if is_increment {
        control - 1
    } else {
        control + 1
    };
    let (x, y) = if is_vertical { (delta, j) } else { (i, delta) };
    if matrix[x][y] == 0 {
        matrix[x][y] = matrix[i][j];
        matrix[i][j] = 0;
    }
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    // _update_direction test
    // Columns | Rows
    // 1 - Multiple Combinations
    // 2 - No combinations (move different values without overriding)
    // 3 - Single "gapless" "moveless" (overrides previous value)
    // 4 - Single gap move

    #[test]
    fn _update_direction_up_test() {
        const UP: [bool; 4] = [true, false, false, false];

        let mut matrix = [[2, 0, 4, 0], [2, 0, 4, 8], [2, 2, 0, 0], [2, 4, 2, 8]];

        const EXPECTED_SINGLE_MOVE: [[usize; 4]; 4] =
            [[4, 2, 8, 16], [4, 4, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        _update_direction(&UP, &mut matrix);
        assert_eq!(matrix, EXPECTED_SINGLE_MOVE);

        const EXPECTED_SECOND_MOVE: [[usize; 4]; 4] =
            [[8, 2, 8, 16], [0, 4, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        _update_direction(&UP, &mut matrix);
        assert_eq!(matrix, EXPECTED_SECOND_MOVE);
    }

    #[test]
    fn _update_direction_down_test() {
        const DOWN: [bool; 4] = [false, false, true, false];

        let mut matrix = [[2, 4, 0, 8], [2, 2, 0, 0], [2, 0, 4, 8], [2, 0, 4, 0]];

        const EXPECTED_SINGLE_MOVE: [[usize; 4]; 4] =
            [[0, 0, 0, 0], [0, 0, 0, 0], [4, 4, 0, 0], [4, 2, 8, 16]];
        _update_direction(&DOWN, &mut matrix);
        assert_eq!(matrix, EXPECTED_SINGLE_MOVE);

        const EXPECTED_SECOND_MOVE: [[usize; 4]; 4] =
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 4, 0, 0], [8, 2, 8, 16]];
        _update_direction(&DOWN, &mut matrix);
        assert_eq!(matrix, EXPECTED_SECOND_MOVE);
    }

    #[test]
    fn _update_direction_left_test() {
        const LEFT: [bool; 4] = [false, true, false, false];

        let mut matrix = [[2, 2, 2, 2], [0, 0, 2, 4], [4, 4, 0, 2], [0, 8, 0, 8]];

        const EXPECTED_SINGLE_MOVE: [[usize; 4]; 4] =
            [[4, 4, 0, 0], [2, 4, 0, 0], [8, 2, 0, 0], [16, 0, 0, 0]];
        _update_direction(&LEFT, &mut matrix);
        assert_eq!(matrix, EXPECTED_SINGLE_MOVE);

        const EXPECTED_SECOND_MOVE: [[usize; 4]; 4] = [
            [8, 0, 0, 0],
            EXPECTED_SINGLE_MOVE[1],
            EXPECTED_SINGLE_MOVE[2],
            EXPECTED_SINGLE_MOVE[3],
        ];
        _update_direction(&LEFT, &mut matrix);
        assert_eq!(matrix, EXPECTED_SECOND_MOVE);
    }

    #[test]
    fn _update_direction_right_test() {
        const RIGHT: [bool; 4] = [false, false, false, true];

        let mut matrix = [[2, 2, 2, 2], [4, 2, 0, 0], [2, 0, 4, 4], [8, 0, 8, 0]];

        const EXPECTED_SINGLE_MOVE: [[usize; 4]; 4] =
            [[0, 0, 4, 4], [0, 0, 4, 2], [0, 0, 2, 8], [0, 0, 0, 16]];
        _update_direction(&RIGHT, &mut matrix);
        assert_eq!(matrix, EXPECTED_SINGLE_MOVE);

        const EXPECTED_SECOND_MOVE: [[usize; 4]; 4] = [
            [0, 0, 0, 8],
            EXPECTED_SINGLE_MOVE[1],
            EXPECTED_SINGLE_MOVE[2],
            EXPECTED_SINGLE_MOVE[3],
        ];
        _update_direction(&RIGHT, &mut matrix);
        assert_eq!(matrix, EXPECTED_SECOND_MOVE);
    }
}
