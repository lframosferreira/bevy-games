use crate::events::EndGame;
use crate::ui::menu::components::{QuitButton, ResumeButton};
use crate::ui::menu::game_over::components::GameOverMenu;
use crate::ui::menu::layout::spawn_button;
use crate::ui::menu::styles::{get_node_bundle, get_text_bundle, get_title, get_title_text_style};
use bevy::prelude::*;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over_event_reader: EventReader<EndGame>,
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
    mut game_over_event_reader: EventReader<EndGame>,
) {
    if let Some(message) = game_over_event_reader.iter().last() {
        let message = message.string.clone();
        commands
            .spawn((get_node_bundle(), GameOverMenu {}))
            .with_children(|parent| {
                parent.spawn(get_title()).with_children(|parent| {
                    parent.spawn(get_text_bundle(
                        &message,
                        asset_server,
                        get_title_text_style,
                    ));
                });
                spawn_button(parent, asset_server, "Restart", ResumeButton {});
                spawn_button(parent, asset_server, "Quit", QuitButton {});
            });
    }
}
