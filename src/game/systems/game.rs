use bevy::prelude::*;

use crate::game::components::*;
use crate::game::state::*;
use crate::game::utils::*;

use super::ai::make_move;

pub fn spawn_grid(mut commands: Commands) {
    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    let half = board_px / 2.0;
    for i in 1..BOARD_SIZE {
        let offset = i as f32 * CELL_SIZE - half;
        // vertical line at x = offset
        commands.spawn((
            Sprite::from_color(
                GRID_COLOR,
                Vec2::new(LINE_THICKNESS, board_px + LINE_THICKNESS),
            ),
            Transform::from_translation(Vec3::new(offset, 0.0, 0.0)),
            Grid,
        ));
        // horizontal line at y = offset
        commands.spawn((
            Sprite::from_color(
                GRID_COLOR,
                Vec2::new(board_px + LINE_THICKNESS, LINE_THICKNESS),
            ),
            Transform::from_translation(Vec3::new(0.0, offset, 0.0)),
            Grid,
        ));
    }

    // No need to spawn per-cell entities; clicks are mapped to board indices by math.
}

pub fn handle_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<GameState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    player_config: Res<PlayerConfig>,
    interaction_query: Query<&Interaction, With<Button>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if state.winner.is_some() {
        return;
    }

    // Don't process board clicks if interacting with any UI button
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
            return;
        }
    }

    // Check if it's a human player's turn
    let current_player_type = match state.turn {
        Player::X => player_config.x_type,
        Player::O => player_config.o_type,
    };

    if current_player_type != PlayerType::Human {
        return;
    }

    let Some(cursor_world) = world_cursor_pos(windows, camera_q) else {
        return;
    };

    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    let half = board_px / 2.0;
    let x = cursor_world.x + half;
    let y = cursor_world.y + half;
    if x < 0.0 || y < 0.0 || x >= board_px || y >= board_px {
        return;
    }
    let col = (x / CELL_SIZE).floor() as usize;
    let row = (y / CELL_SIZE).floor() as usize;

    if state.board[row][col].is_some() {
        return;
    }

    make_move(&mut state, row, col);
}
