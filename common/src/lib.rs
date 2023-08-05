pub mod events;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use events::EndGame;
use systems::spawn_camera;
use ui::menu::MenuPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Pause,
    GameOver,
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGame>()
            .add_state::<AppState>()
            .add_systems(Startup, spawn_camera)
            .add_plugins(MenuPlugin);
    }
}
