use bevy::prelude::*;

use crate::game::components::*;
use crate::game::state::*;

// Type aliases to reduce complexity
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
                    TextColor(WIN_COLOR),
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
                        BackgroundColor(UI_BG),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("New Game (R)"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(UI_ACCENT),
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
                        BackgroundColor(UI_BG),
                        BackToMenuButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Back to Menu (Esc)"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(UI_ACCENT),
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
                *bg_color = BackgroundColor(UI_BORDER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(UI_BG);
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
