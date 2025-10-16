use bevy::prelude::*;

use super::state::{BOARD_SIZE, CELL_SIZE};

pub fn cell_center(row: usize, col: usize) -> Vec2 {
    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    let half = board_px / 2.0;
    Vec2::new(
        col as f32 * CELL_SIZE + CELL_SIZE * 0.5 - half,
        row as f32 * CELL_SIZE + CELL_SIZE * 0.5 - half,
    )
}

pub fn world_cursor_pos(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let (camera, cam_transform) = camera_q.single().ok()?;
    let cursor = window.cursor_position()?;
    camera.viewport_to_world_2d(cam_transform, cursor).ok()
}
