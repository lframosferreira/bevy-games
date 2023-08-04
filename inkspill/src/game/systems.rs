use super::{
    components::{Block, Heart},
    BLOCK_SIZE, MAX_LIVES, SIZE,
};
use bevy::prelude::*;
use common::{events::EndGame, AppState};

const NUM_BLOCKS: usize = 16;
const COLORS: [Color; 6] = [
    Color::PINK,
    Color::ORANGE_RED,
    Color::GREEN,
    Color::BLUE,
    Color::YELLOW,
    Color::CYAN,
];

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
    // HACK adiciona vida extra pois a função take_life rouba uma vida ao inicializar os blocos
    for i in 0..MAX_LIVES + 1 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(X_OFFSET, i as f32 * BLOCK_SIZE + Y_OFFSET, 0.),
                ..default()
            },
            Heart(i),
        ));
    }
}

pub fn respawn_life_bar(mut commands: Commands, heart_query: Query<Entity, With<Heart>>) {
    for entity in heart_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_life_bar(commands);
}

pub fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    const Y_OFFSET: f32 = 3. * BLOCK_SIZE;
    const X_OFFSET: f32 = 6. * BLOCK_SIZE;
    for (i, color) in COLORS.iter().enumerate() {
        let x = i as f32 * 2. * BLOCK_SIZE + X_OFFSET;
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                format!("{}", i + 1),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            // Precisa estar em cima da cor, por isso 0.1
            transform: Transform::from_xyz(x, Y_OFFSET, 0.1),
            ..default()
        });
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: *color,
                custom_size: Some(Vec2::new(2. * BLOCK_SIZE, 2. * BLOCK_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, Y_OFFSET, 0.),
            ..default()
        },));
    }
}

// TODO check early game win
pub fn take_life(
    mut commands: Commands,
    changed_heart: Query<Entity, Changed<Sprite>>,
    heart_query: Query<Entity, With<Heart>>,
    mut game_over_event_writer: EventWriter<EndGame>,
) {
    if !changed_heart.is_empty() {
        if let Some(entity) = heart_query.iter().last() {
            commands.entity(entity).despawn();
        } else {
            commands.insert_resource(NextState(Some(AppState::GameOver)));
            game_over_event_writer.send(EndGame { score: 0 });
        }
    }
}

pub fn update_color(
    keyboard_input: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Block, &mut Sprite), Without<Heart>>,
) {
    const KEYS: [KeyCode; 6] = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
    ];
    for i in 0..KEYS.len() {
        if keyboard_input.just_pressed(KEYS[i]) {
            _update_color(&mut block_query, COLORS[i])
        }
    }
}

fn _update_color(block_query: &mut Query<(&mut Block, &mut Sprite), Without<Heart>>, color: Color) {
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
    }
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
