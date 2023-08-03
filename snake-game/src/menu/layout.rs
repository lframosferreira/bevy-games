use super::styles::{get_button_text_style, get_normal_button, get_text_bundle};
use bevy::prelude::*;

pub fn spawn_button<T: Component>(
    parent: &mut ChildBuilder<'_, '_, '_>,
    asset_server: &Res<'_, AssetServer>,
    text: &str,
    button: T,
) {
    parent
        .spawn((get_normal_button(), button))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(text, asset_server, get_button_text_style));
        });
}
