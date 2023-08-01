mod pause;

use bevy::prelude::*;

use self::pause::PauseMenuPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PauseMenuPlugin);
    }
}
