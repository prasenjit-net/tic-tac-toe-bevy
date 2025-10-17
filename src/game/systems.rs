use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::state::*;
use super::utils::*;

// Type aliases to reduce complexity
type MenuButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static MenuButton,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

type GameOverButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        Option<&'static BackToMenuButton>,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

type CleanupEntitiesQuery<'w, 's> = Query<
    'w,
    's,
    Entity,
    Or<(
        With<Mark>,
        With<WinHighlight>,
        With<Grid>,
        With<ScoreboardUI>,
        With<GameOverUI>,
    )>,
>;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ===== MENU SYSTEMS =====

pub fn spawn_menu(mut commands: Commands) {
    let button_style = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextFont {
        font_size: 20.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Tic-Tac-Toe"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Choose Players:"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Human vs Human
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::Human,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Human"),
                        text_style.clone(),
                        TextColor(Color::WHITE),
                    ));
                });

            // Human vs Computer Easy
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::ComputerEasy,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Computer (Easy)"),
                        text_style.clone(),
                        TextColor(Color::WHITE),
                    ));
                });

            // Human vs Computer Hard
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::ComputerHard,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Computer (Hard)"),
                        text_style.clone(),
                        TextColor(Color::WHITE),
                    ));
                });

            // Computer vs Computer
            parent
                .spawn((
                    Button,
                    button_style,
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    MenuButton {
                        x_type: PlayerType::ComputerEasy,
                        o_type: PlayerType::ComputerHard,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Computer vs Computer"),
                        text_style,
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn handle_menu_buttons(
    mut interaction_query: MenuButtonQuery,
    mut player_config: ResMut<PlayerConfig>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                player_config.x_type = menu_button.x_type;
                player_config.o_type = menu_button.o_type;
                next_state.set(AppState::Playing);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.4));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.3));
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ===== SCOREBOARD SYSTEMS =====

pub fn spawn_scoreboard(
    mut commands: Commands,
    player_config: Res<PlayerConfig>,
    score: Res<Score>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            ScoreboardUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("Player X: {}", player_config.x_type.label())),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(X_COLOR),
            ));
            parent.spawn((
                Text::new(format!("Player O: {}", player_config.o_type.label())),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(O_COLOR),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(format!(
                    "X Wins: {} | O Wins: {} | Draws: {}",
                    score.x_wins, score.o_wins, score.draws
                )),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                ScoreText,
            ));
        });
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!(
            "X Wins: {} | O Wins: {} | Draws: {}",
            score.x_wins, score.o_wins, score.draws
        );
    }
}

// ===== GAME SYSTEMS =====

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

// ===== AI SYSTEMS =====

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

fn make_move(state: &mut GameState, row: usize, col: usize) {
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

// ===== SCORE AND GAME OVER SYSTEMS =====

pub fn update_score(mut score: ResMut<Score>, state: Res<GameState>, mut game_ended: Local<bool>) {
    if state.is_changed() && (state.winner.is_some() || state.moves == BOARD_SIZE * BOARD_SIZE) {
        if !*game_ended {
            *game_ended = true;
            match state.winner {
                Some(Player::X) => score.x_wins += 1,
                Some(Player::O) => score.o_wins += 1,
                None => score.draws += 1,
            }
        }
    } else if state.moves == 0 {
        *game_ended = false;
    }
}

pub fn show_game_over_ui(
    mut commands: Commands,
    state: Res<GameState>,
    existing: Query<Entity, With<GameOverUI>>,
) {
    if !state.is_changed() {
        return;
    }

    // Remove existing game over UI
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    // Show game over UI if game ended
    if state.winner.is_some() || state.moves == BOARD_SIZE * BOARD_SIZE {
        let message = match state.winner {
            Some(Player::X) => "Player X Wins!",
            Some(Player::O) => "Player O Wins!",
            None => "It's a Draw!",
        };

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                GameOverUI,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(message),
                    TextFont {
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("New Game (R)"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.6)),
                        BackToMenuButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Back to Menu (Esc)"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
    }
}

pub fn handle_game_over_buttons(
    mut interaction_query: GameOverButtonQuery,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, back_to_menu, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if back_to_menu.is_some() {
                    next_state.set(AppState::Menu);
                } else {
                    state.reset();
                }
            }
            Interaction::Hovered => {
                if back_to_menu.is_some() {
                    *bg_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.7));
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.3, 0.7, 0.3));
                }
            }
            Interaction::None => {
                if back_to_menu.is_some() {
                    *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.6));
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
                }
            }
        }
    }
}

pub fn keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        state.reset();
    }

    if keys.just_pressed(KeyCode::Escape) && *current_state.get() == AppState::Playing {
        next_state.set(AppState::Menu);
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    entities: CleanupEntitiesQuery,
    mut state: ResMut<GameState>,
    mut score: ResMut<Score>,
) {
    for e in entities.iter() {
        commands.entity(e).despawn();
    }

    state.reset();
    *score = Score::default();
}

// ===== RENDERING SYSTEMS =====

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

fn check_winner(board: &[[Option<Player>; BOARD_SIZE]; BOARD_SIZE]) -> Option<WinningLine> {
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
