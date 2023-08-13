mod lives;
mod score;

use bevy::prelude::*;
use lives::LivesHUDPlugin;
use score::ScoreHUDPlugin;

const FONT_SIZE: f32 = 40.;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScoreHUDPlugin, LivesHUDPlugin));
    }
}
