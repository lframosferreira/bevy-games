use crate::game::{BLOCK_SIZE, MAX_LIVES};
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub const WINDOW_X: f32 = 600.0;
pub const WINDOW_Y: f32 = 660.0;

pub fn draw_grid(mut lines: ResMut<DebugLines>) {
    let x_left_offset = 3;
    let x_right_offset = 1;
    let y_bottom_offset = 5;
    let y_top_offset = 1;

    for i in x_left_offset..(WINDOW_X / BLOCK_SIZE) as u32 - x_right_offset + 1 {
        lines.line(
            Vec3::new(
                BLOCK_SIZE * i as f32,
                y_bottom_offset as f32 * BLOCK_SIZE,
                0.,
            ),
            Vec3::new(
                BLOCK_SIZE * i as f32,
                WINDOW_Y - y_top_offset as f32 * BLOCK_SIZE,
                0.,
            ),
            0.,
        )
    }
    for i in y_bottom_offset..(WINDOW_Y / BLOCK_SIZE) as u32 - y_top_offset + 1 {
        lines.line(
            Vec3::new(x_left_offset as f32 * BLOCK_SIZE, BLOCK_SIZE * i as f32, 0.),
            Vec3::new(
                WINDOW_X - x_right_offset as f32 * BLOCK_SIZE,
                BLOCK_SIZE * i as f32,
                0.,
            ),
            0.,
        )
    }

    for i in 0..MAX_LIVES + 1 {
        lines.line(
            Vec3::new(BLOCK_SIZE, BLOCK_SIZE * (i as f32 + 1.), 0.),
            Vec3::new(2. * BLOCK_SIZE, BLOCK_SIZE * (i as f32 + 1.), 0.),
            0.,
        )
    }
    lines.line(
        Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 0.),
        Vec3::new(BLOCK_SIZE, (MAX_LIVES as f32 + 1.) * BLOCK_SIZE, 0.),
        0.,
    );
    lines.line(
        Vec3::new(2. * BLOCK_SIZE, BLOCK_SIZE, 0.),
        Vec3::new(2. * BLOCK_SIZE, (MAX_LIVES as f32 + 1.) * BLOCK_SIZE, 0.),
        0.,
    );
}
