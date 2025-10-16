use bevy::prelude::*;

#[derive(Component)]
pub struct Cell { pub row: usize, pub col: usize }

#[derive(Component)]
pub struct Mark;

#[derive(Component)]
pub struct Grid;

#[derive(Component)]
pub struct WinHighlight;
