use crate::events::GameOver;
use crate::menu::components::QuitButton;
use crate::menu::game_over::components::{GameOverMenu, RestartButton};
use crate::menu::styles::{
    get_button_text_style, get_normal_button, get_pause_menu_style, get_text_bundle, get_title,
    get_title_text_style,
};
use bevy::prelude::*;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_event_reader: EventReader<GameOver>,
) {
    build_game_over_menu(&mut commands, &asset_server, game_over_event_reader);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut game_over_event_reader: EventReader<GameOver>,
) -> Entity {
    let mut score: Option<u32> = None;
    for event in game_over_event_reader.iter() {
        score = Some(event.score);
    }
    commands
        .spawn((
            NodeBundle {
                style: get_pause_menu_style(),
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(get_title()).with_children(|parent| {
                parent.spawn(get_text_bundle(
                    &format!("Score {}", score.expect("")),
                    asset_server,
                    get_title_text_style,
                ));
            });
            parent
                .spawn((get_normal_button(), RestartButton {}))
                .with_children(|parent| {
                    parent.spawn(get_text_bundle(
                        "Restart",
                        asset_server,
                        get_button_text_style,
                    ));
                });
            parent
                .spawn((get_normal_button(), QuitButton {}))
                .with_children(|parent| {
                    parent.spawn(get_text_bundle("Quit", asset_server, get_button_text_style));
                });
        })
        .id()
}
