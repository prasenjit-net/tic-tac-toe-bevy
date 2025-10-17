use bevy::prelude::*;
use rand::Rng;

use crate::game::components::*;
use crate::game::state::*;

use super::rendering::check_winner;

pub fn computer_player(
    time: Res<Time>,
    mut timer: ResMut<ComputerMoveTimer>,
    mut state: ResMut<GameState>,
    player_config: Res<PlayerConfig>,
) {
    if state.winner.is_some() {
        return;
    }

    let current_player_type = match state.turn {
        Player::X => player_config.x_type,
        Player::O => player_config.o_type,
    };

    if current_player_type == PlayerType::Human {
        timer.timer.reset();
        return;
    }

    timer.timer.tick(time.delta());

    if !timer.timer.is_finished() {
        return;
    }

    // Make computer move
    if let Some((row, col)) = find_computer_move(&state.board, current_player_type, state.turn) {
        make_move(&mut state, row, col);

        // Reset timer with random delay (200-800ms)
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(0.2..0.8);
        timer.timer = Timer::from_seconds(delay, TimerMode::Once);
    }
}

fn find_computer_move(
    board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    player_type: PlayerType,
    player: Player,
) -> Option<(usize, usize)> {
    match player_type {
        PlayerType::Human => None,
        PlayerType::ComputerEasy => find_easy_move(board),
        PlayerType::ComputerHard => find_hard_move(board, player),
    }
}

fn find_easy_move(board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE]) -> Option<(usize, usize)> {
    let mut empty_cells = Vec::new();
    for (row, row_data) in board.iter().enumerate() {
        for (col, cell) in row_data.iter().enumerate() {
            if cell.is_none() {
                empty_cells.push((row, col));
            }
        }
    }

    if empty_cells.is_empty() {
        return None;
    }

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..empty_cells.len());
    Some(empty_cells[idx])
}

fn find_hard_move(
    board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    player: Player,
) -> Option<(usize, usize)> {
    // Try to win
    if let Some(pos) = find_winning_move(board, player) {
        return Some(pos);
    }

    // Block opponent from winning
    if let Some(pos) = find_winning_move(board, player.other()) {
        return Some(pos);
    }

    // Take center if available
    if board[1][1].is_none() {
        return Some((1, 1));
    }

    // Take a corner
    for &(r, c) in &[(0, 0), (0, 2), (2, 0), (2, 2)] {
        if board[r][c].is_none() {
            return Some((r, c));
        }
    }

    // Take any available position
    find_easy_move(board)
}

fn find_winning_move(
    board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    player: Player,
) -> Option<(usize, usize)> {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[row][col].is_none() {
                let mut test_board = *board;
                test_board[row][col] = Some(player);
                if check_winner(&test_board).is_some() {
                    return Some((row, col));
                }
            }
        }
    }
    None
}

pub fn make_move(state: &mut GameState, row: usize, col: usize) {
    let player = state.turn;
    state.board[row][col] = Some(player);
    state.moves += 1;

    if let Some(line) = check_winner(&state.board) {
        state.winner = Some(player);
        state.winning_line = Some(line);
    } else if state.moves == BOARD_SIZE * BOARD_SIZE {
        // Draw
        state.winner = None;
    } else {
        state.turn = player.other();
    }
}
