pub mod events;
pub mod game;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use events::EndGame;
use systems::*;
use ui::menu::MenuPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Pause,
    GameOver,
}

#[derive(Default)]
pub struct CommonPlugin {
    is_background_light: bool,
}

impl CommonPlugin {
    pub fn new_light() -> Self {
        Self {
            is_background_light: true,
        }
    }
    /// Change text color to black
    pub fn set_text_dark(mut text_query: Query<&mut Text>) {
        for mut text in text_query.iter_mut() {
            let mut sections = text.sections.clone();
            for section in &mut sections {
                section.style.color = Color::BLACK;
            }
            *text = Text::from_sections(sections)
        }
    }
}

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        if self.is_background_light {
            app.add_systems(Update, CommonPlugin::set_text_dark);
        }
        app.add_event::<EndGame>()
            .add_state::<AppState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (pause_game, resume_game))
            .add_plugins(MenuPlugin);
    }
}
