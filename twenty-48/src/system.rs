use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::game::{BLOCK_LENGTH, WINDOW_X, WINDOW_Y};

pub fn draw_grid(mut lines: ResMut<DebugLines>) {
    for i in 0..(WINDOW_X / BLOCK_LENGTH) as u32 {
        lines.line(
            Vec3::new(BLOCK_LENGTH * i as f32, 0., 0.),
            Vec3::new(BLOCK_LENGTH * i as f32, WINDOW_Y, 0.),
            0.0,
        )
    }
    for i in 0..(WINDOW_Y / BLOCK_LENGTH) as u32 {
        lines.line(
            Vec3::new(0., BLOCK_LENGTH * i as f32, 0.),
            Vec3::new(WINDOW_X, BLOCK_LENGTH * i as f32, 0.),
            0.0,
        )
    }
}
