use bevy::prelude::*;

use crate::game::components::*;
use crate::game::state::*;

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
