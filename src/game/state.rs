use bevy::prelude::*;

pub const BOARD_SIZE: usize = 3;
pub const CELL_SIZE: f32 = 150.0; // world units per cell
pub const LINE_THICKNESS: f32 = 6.0;

pub const BG_COLOR: Color = Color::srgb(0.09, 0.10, 0.12);
pub const GRID_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
pub const X_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);
pub const O_COLOR: Color = Color::srgb(0.2, 0.6, 0.95);
pub const WIN_COLOR: Color = Color::srgb(0.2, 0.95, 0.4);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Player { #[default] X, O }

#[derive(Resource, Default)]
pub struct GameState {
    pub board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    pub turn: Player,
    pub winner: Option<Player>,
    pub moves: usize,
    pub winning_line: Option<WinningLine>,
}

#[derive(Clone, Copy, Debug)]
pub struct WinningLine { pub start: UVec2, pub end: UVec2 }
