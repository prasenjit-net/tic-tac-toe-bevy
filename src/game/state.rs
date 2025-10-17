use bevy::prelude::*;

use super::components::PlayerType;

pub const BOARD_SIZE: usize = 3;
pub const CELL_SIZE: f32 = 150.0; // world units per cell
pub const LINE_THICKNESS: f32 = 6.0;

// Modern dark theme color palette
pub const BG_COLOR: Color = Color::srgb(0.04, 0.05, 0.10); // Deep space blue
pub const GRID_COLOR: Color = Color::srgb(0.31, 0.80, 0.77); // Cyan glow
pub const GRID_GLOW: Color = Color::srgba(0.31, 0.80, 0.77, 0.15); // Subtle cyan glow

// Player colors with vibrant style
pub const X_COLOR: Color = Color::srgb(1.0, 0.42, 0.42); // Hot red-orange
pub const X_GLOW: Color = Color::srgba(1.0, 0.42, 0.42, 0.3); // Red glow

pub const O_COLOR: Color = Color::srgb(0.31, 0.80, 0.77); // Electric cyan
pub const O_GLOW: Color = Color::srgba(0.31, 0.80, 0.77, 0.3); // Cyan glow

pub const WIN_COLOR: Color = Color::srgb(1.0, 0.85, 0.24); // Golden yellow
pub const WIN_GLOW: Color = Color::srgba(1.0, 0.85, 0.24, 0.5); // Gold pulse

// UI colors for glass morphism
pub const UI_BG: Color = Color::srgba(0.10, 0.12, 0.18, 0.85); // Glass background
pub const UI_BORDER: Color = Color::srgba(0.31, 0.80, 0.77, 0.4); // Cyan border
pub const UI_ACCENT: Color = Color::srgb(0.44, 1.0, 0.91); // Bright cyan

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
