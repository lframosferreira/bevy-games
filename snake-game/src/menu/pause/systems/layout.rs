use crate::menu::components::QuitButton;
use crate::menu::layout::spawn_button;
use crate::menu::pause::components::{PauseMenu, ResumeButton};
use crate::menu::styles::{get_pause_menu_style, get_text_bundle, get_title, get_title_text_style};
use bevy::prelude::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(mut commands: Commands, main_menu_query: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = main_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: get_pause_menu_style(),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(get_title()).with_children(|parent| {
                parent.spawn(get_text_bundle("SNAKE", asset_server, get_title_text_style));
            });
            spawn_button(parent, asset_server, "Resume", ResumeButton {});
            spawn_button(parent, asset_server, "Quit", QuitButton {});
        })
        .id()
}
