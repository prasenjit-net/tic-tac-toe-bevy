use bevy::prelude::*;

use super::components::PlayerType;

pub const BOARD_SIZE: usize = 3;
pub const CELL_SIZE: f32 = 150.0; // world units per cell
pub const LINE_THICKNESS: f32 = 6.0;

pub const BG_COLOR: Color = Color::srgb(0.09, 0.10, 0.12);
pub const GRID_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
pub const X_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);
pub const O_COLOR: Color = Color::srgb(0.2, 0.6, 0.95);
pub const WIN_COLOR: Color = Color::srgb(0.2, 0.95, 0.4);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Player {
    #[default]
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    pub turn: Player,
    pub winner: Option<Player>,
    pub moves: usize,
    pub winning_line: Option<WinningLine>,
}

impl GameState {
    pub fn reset(&mut self) {
        self.board = [[None; BOARD_SIZE]; BOARD_SIZE];
        self.turn = Player::X;
        self.winner = None;
        self.moves = 0;
        self.winning_line = None;
    }
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub x_type: PlayerType,
    pub o_type: PlayerType,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            x_type: PlayerType::Human,
            o_type: PlayerType::Human,
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub x_wins: u32,
    pub o_wins: u32,
    pub draws: u32,
}

#[derive(Resource)]
pub struct ComputerMoveTimer {
    pub timer: Timer,
}

impl Default for ComputerMoveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WinningLine {
    pub start: UVec2,
    pub end: UVec2,
}
