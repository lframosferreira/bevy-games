use super::resources::Lives;
use bevy::prelude::*;

pub fn reset_lives(mut lives: ResMut<Lives>) {
    lives.reset();
}
