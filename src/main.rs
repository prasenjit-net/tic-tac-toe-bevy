use bevy::prelude::*;

const BOARD_SIZE: usize = 3;
const CELL_SIZE: f32 = 150.0; // world units per cell
const LINE_THICKNESS: f32 = 6.0;
const BG_COLOR: Color = Color::srgb(0.09, 0.10, 0.12);
const GRID_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
const X_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);
const O_COLOR: Color = Color::srgb(0.2, 0.6, 0.95);
const WIN_COLOR: Color = Color::srgb(0.2, 0.95, 0.4);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Player { #[default] X, O }

#[derive(Resource, Default)]
struct GameState {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    turn: Player,
    winner: Option<Player>,
    moves: usize,
    winning_line: Option<WinningLine>,
}

#[derive(Clone, Copy, Debug)]
struct WinningLine { start: UVec2, end: UVec2 }

#[derive(Component)]
struct Cell { row: usize, col: usize }

#[derive(Component)]
struct Mark;

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct WinHighlight;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Tic-Tac-Toe (Bevy 0.17)".into(), resolution: (600u32, 700u32).into(), resizable: false, ..default() }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, spawn_grid))
        .add_systems(Update, (
            handle_clicks,
            draw_marks,
            draw_win_highlight,
            keyboard_reset,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_grid(mut commands: Commands) {
    // Grid background size
    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    // Centered at 0,0 - draw 4 lines (2 vertical, 2 horizontal)
    let half = board_px / 2.0;
    for i in 1..BOARD_SIZE {
        let offset = i as f32 * CELL_SIZE - half;
        // vertical line at x = offset
        commands.spawn((
            Sprite::from_color(GRID_COLOR, Vec2::new(LINE_THICKNESS, board_px + LINE_THICKNESS)),
            Transform::from_translation(Vec3::new(offset, 0.0, 0.0)),
            Grid,
        ));
        // horizontal line at y = offset
        commands.spawn((
            Sprite::from_color(GRID_COLOR, Vec2::new(board_px + LINE_THICKNESS, LINE_THICKNESS)),
            Transform::from_translation(Vec3::new(0.0, offset, 0.0)),
            Grid,
        ));
    }

    // Invisible clickable cells
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let center = cell_center(row, col);
            commands.spawn((
                Cell { row, col },
                // transparent hitbox sprite
                Sprite::from_color(Color::NONE, Vec2::splat(CELL_SIZE)),
                Transform::from_translation(Vec3::new(center.x, center.y, -1.0)),
            ));
        }
    }
}

fn world_cursor_pos(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let (camera, cam_transform) = camera_q.single().ok()?;
    let Some(cursor) = window.cursor_position() else { return None };
    camera.viewport_to_world_2d(cam_transform, cursor).ok()
}

fn handle_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<GameState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }
    if state.winner.is_some() { return; }

    let Some(cursor_world) = world_cursor_pos(windows, camera_q) else { return };

    // Determine which cell was clicked by index math (faster than iterating entities)
    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    let half = board_px / 2.0;
    let x = cursor_world.x + half;
    let y = cursor_world.y + half;
    if x < 0.0 || y < 0.0 || x >= board_px || y >= board_px { return; }
    let col = (x / CELL_SIZE).floor() as usize;
    let row = (y / CELL_SIZE).floor() as usize;

    if state.board[row][col].is_some() { return; }

    // Place mark
    let player = state.turn;
    state.board[row][col] = Some(player);
    state.moves += 1;

    // Check win/draw
    if let Some(line) = check_winner(&state.board) {
        state.winner = Some(player);
        state.winning_line = Some(line);
    } else if state.moves == BOARD_SIZE * BOARD_SIZE {
        state.winner = None; // draw indicated by None + moves full
    } else {
        state.turn = match state.turn { Player::X => Player::O, Player::O => Player::X };
    }

    // Mark entity will be drawn by draw_marks system
    // We issue a short-lived event-like entity if needed; for now, draw_marks uses state only.
}

fn draw_marks(
    mut commands: Commands,
    mut marks: Query<(Entity, &mut Sprite, &mut Transform), With<Mark>>,
    state: Res<GameState>,
) {
    if !state.is_changed() { return; }

    // Clear existing marks and redraw according to state
    for (e, _, _) in marks.iter_mut() { commands.entity(e).despawn(); }

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(player) = state.board[row][col] {
                let center = cell_center(row, col);
                match player {
                    Player::X => spawn_x(&mut commands, center),
                    Player::O => spawn_o(&mut commands, center),
                }
            }
        }
    }
}

fn draw_win_highlight(
    mut commands: Commands,
    mut existing: Query<(Entity, &mut Sprite), With<WinHighlight>>,
    state: Res<GameState>,
) {
    // Clear previous
    for (e, _) in existing.iter_mut() { commands.entity(e).despawn(); }

    if let Some(line) = state.winning_line {
        let (start, end) = (line.start, line.end);
        let start_center = cell_center(start.y as usize, start.x as usize);
        let end_center = cell_center(end.y as usize, end.x as usize);
        let dir = end_center - start_center;
        let length = dir.length() + CELL_SIZE * 0.6; // extend past cells
        let angle = dir.y.atan2(dir.x);
        let mid = (start_center + end_center) * 0.5;
        commands.spawn((
            Sprite::from_color(WIN_COLOR, Vec2::new(length, LINE_THICKNESS * 2.0)),
            Transform::from_translation(Vec3::new(mid.x, mid.y, 1.0)).with_rotation(Quat::from_rotation_z(angle)),
            WinHighlight,
        ));
    }
}

fn keyboard_reset(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut marks: Query<Entity, With<Mark>>,
    mut wins: Query<Entity, With<WinHighlight>>,
) {
    if !(keys.just_pressed(KeyCode::KeyR) || keys.just_pressed(KeyCode::Escape)) { return; }

    // Clear marks and highlights
    for e in marks.iter_mut() { commands.entity(e).despawn(); }
    for e in wins.iter_mut() { commands.entity(e).despawn(); }

    *state = GameState::default();
}

fn spawn_x(commands: &mut Commands, center: Vec2) {
    // Draw X as two crossing rectangles
    let len = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    for angle in [45f32.to_radians(), -45f32.to_radians()] {
        commands.spawn((
            Mark,
            Sprite::from_color(X_COLOR, Vec2::new(len, thickness)),
            Transform::from_translation(Vec3::new(center.x, center.y, z)).with_rotation(Quat::from_rotation_z(angle)),
        ));
    }
}

fn spawn_o(commands: &mut Commands, center: Vec2) {
    // Draw O as a ring using two sprites (outer and inner punch via background color)
    let outer = CELL_SIZE * 0.6;
    let thickness = LINE_THICKNESS * 1.5;
    let z = 0.5;
    commands.spawn((
        Mark,
        Sprite::from_color(O_COLOR, Vec2::new(outer, outer)),
        Transform::from_translation(Vec3::new(center.x, center.y, z)),
    ));
    commands.spawn((
        Mark,
        Sprite::from_color(BG_COLOR, Vec2::new(outer - thickness, outer - thickness)),
        Transform::from_translation(Vec3::new(center.x, center.y, z + 0.01)),
    ));
}

fn cell_center(row: usize, col: usize) -> Vec2 {
    let board_px = CELL_SIZE * BOARD_SIZE as f32;
    let half = board_px / 2.0;
    Vec2::new(
        col as f32 * CELL_SIZE + CELL_SIZE * 0.5 - half,
        row as f32 * CELL_SIZE + CELL_SIZE * 0.5 - half,
    )
}

fn check_winner(board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE]) -> Option<WinningLine> {
    // rows
    for r in 0..BOARD_SIZE {
        if let Some(p) = board[r][0] {
            if (1..BOARD_SIZE).all(|c| board[r][c] == Some(p)) {
                return Some(WinningLine { start: UVec2::new(0, r as u32), end: UVec2::new((BOARD_SIZE - 1) as u32, r as u32) });
            }
        }
    }
    // cols
    for c in 0..BOARD_SIZE {
        if let Some(p) = board[0][c] {
            if (1..BOARD_SIZE).all(|r| board[r][c] == Some(p)) {
                return Some(WinningLine { start: UVec2::new(c as u32, 0), end: UVec2::new(c as u32, (BOARD_SIZE - 1) as u32) });
            }
        }
    }
    // diag \ 
    if let Some(p) = board[0][0] {
        if (1..BOARD_SIZE).all(|i| board[i][i] == Some(p)) {
            return Some(WinningLine { start: UVec2::new(0, 0), end: UVec2::new((BOARD_SIZE - 1) as u32, (BOARD_SIZE - 1) as u32) });
        }
    }
    // diag /
    if let Some(p) = board[0][BOARD_SIZE - 1] {
        if (1..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - 1 - i] == Some(p)) {
            return Some(WinningLine { start: UVec2::new((BOARD_SIZE - 1) as u32, 0), end: UVec2::new(0, (BOARD_SIZE - 1) as u32) });
        }
    }
    None
}
