use bevy::prelude::*;

use crate::game::components::*;
use crate::game::state::*;
use crate::game::utils::*;

pub fn draw_marks(
    mut commands: Commands,
    marks: Query<Entity, With<Mark>>,
    state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !state.is_changed() {
        return;
    }
    for e in marks.iter() {
        commands.entity(e).despawn();
    }

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(player) = state.board[row][col] {
                let center = cell_center(row, col);
                match player {
                    Player::X => spawn_x(&mut commands, center, &mut meshes, &mut materials),
                    Player::O => spawn_o(&mut commands, center, &mut meshes, &mut materials),
                }
            }
        }
    }
}

pub fn draw_win_highlight(
    mut commands: Commands,
    mut existing: Query<(Entity, &mut Sprite), With<WinHighlight>>,
    state: Res<GameState>,
) {
    for (e, _) in existing.iter_mut() {
        commands.entity(e).despawn();
    }

    if let Some(line) = state.winning_line {
        let (start, end) = (line.start, line.end);
        let start_center = cell_center(start.y as usize, start.x as usize);
        let end_center = cell_center(end.y as usize, end.x as usize);
        let dir = end_center - start_center;
        let length = dir.length() + CELL_SIZE * 0.6;
        let angle = dir.y.atan2(dir.x);
        let mid = (start_center + end_center) * 0.5;
        commands.spawn((
            Sprite::from_color(WIN_COLOR, Vec2::new(length, LINE_THICKNESS * 2.0)),
            Transform::from_translation(Vec3::new(mid.x, mid.y, 1.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            WinHighlight,
        ));
    }
}

fn spawn_x(
    commands: &mut Commands,
    center: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let len = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    for angle in [45f32.to_radians(), -45f32.to_radians()] {
        commands.spawn((
            Mark,
            Mesh2d(meshes.add(Rectangle::new(len, thickness))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(X_COLOR))),
            Transform::from_translation(Vec3::new(center.x, center.y, z))
                .with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}

fn spawn_o(
    commands: &mut Commands,
    center: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let radius = CELL_SIZE * 0.3;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;

    // Outer circle
    commands.spawn((
        Mark,
        Mesh2d(meshes.add(Circle::new(radius))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(O_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
    ));

    // Inner circle (background color to create ring effect)
    commands.spawn((
        Mark,
        Mesh2d(meshes.add(Circle::new(radius - thickness))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BG_COLOR))),
        Transform::from_translation(Vec3::new(center.x, center.y, z + 0.01)),
    ));
}

pub fn check_winner(board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE]) -> Option<WinningLine> {
    // rows
    for (r, row) in board.iter().enumerate() {
        if let Some(p) = row[0] {
            if row.iter().skip(1).all(|&cell| cell == Some(p)) {
                return Some(WinningLine {
                    start: UVec2::new(0, r as u32),
                    end: UVec2::new((BOARD_SIZE - 1) as u32, r as u32),
                });
            }
        }
    }
    // cols
    for c in 0..BOARD_SIZE {
        if let Some(p) = board[0][c] {
            if board.iter().skip(1).all(|row| row[c] == Some(p)) {
                return Some(WinningLine {
                    start: UVec2::new(c as u32, 0),
                    end: UVec2::new(c as u32, (BOARD_SIZE - 1) as u32),
                });
            }
        }
    }
    // diag \
    if let Some(p) = board[0][0] {
        if (1..BOARD_SIZE).all(|i| board[i][i] == Some(p)) {
            return Some(WinningLine {
                start: UVec2::new(0, 0),
                end: UVec2::new((BOARD_SIZE - 1) as u32, (BOARD_SIZE - 1) as u32),
            });
        }
    }
    // diag /
    if let Some(p) = board[0][BOARD_SIZE - 1] {
        if (1..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - 1 - i] == Some(p)) {
            return Some(WinningLine {
                start: UVec2::new((BOARD_SIZE - 1) as u32, 0),
                end: UVec2::new(0, (BOARD_SIZE - 1) as u32),
            });
        }
    }
    None
}
