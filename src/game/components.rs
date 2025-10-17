use bevy::prelude::*;

#[derive(Component)]
pub struct Mark;

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct WinHighlight;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct ScoreboardUI;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct MenuButton {
    pub x_type: PlayerType,
    pub o_type: PlayerType,
}

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerType {
    Human,
    ComputerEasy,
    ComputerHard,
}

impl PlayerType {
    pub fn label(&self) -> &str {
        match self {
            PlayerType::Human => "Human",
            PlayerType::ComputerEasy => "Computer Easy",
            PlayerType::ComputerHard => "Computer Hard",
        }
    }
}
