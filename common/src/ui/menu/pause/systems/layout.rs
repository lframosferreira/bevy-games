use crate::ui::menu::components::ResumeButton;
use crate::ui::menu::layout::spawn_button;
use crate::ui::menu::pause::components::PauseMenu;
use crate::ui::menu::styles::{get_node_bundle, get_text_bundle, get_title, get_title_text_style};
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
        .spawn((get_node_bundle(), PauseMenu {}))
        .with_children(|parent| {
            parent.spawn(get_title()).with_children(|parent| {
                parent.spawn(get_text_bundle(
                    "Bevy Games",
                    asset_server,
                    get_title_text_style,
                ));
            });
            spawn_button(parent, asset_server, "Resume", ResumeButton {});
        })
        .id()
}
