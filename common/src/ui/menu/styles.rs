use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgba(0.10, 0.10, 0.10, 0.3);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgba(0.10, 0.10, 0.10, 0.5);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgba(0.10, 0.10, 0.10, 0.7);

pub fn get_text_bundle(
    text: &str,
    asset_server: &Res<AssetServer>,
    style: fn(&Res<AssetServer>) -> TextStyle,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(text, style(asset_server))],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

pub fn get_normal_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Style::DEFAULT
        },
        background_color: NORMAL_BUTTON_COLOR.into(),
        ..default()
    }
}

pub fn get_menu_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn get_title() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(300.0),
            height: Val::Px(120.0),
            ..Style::DEFAULT
        },
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 48.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
