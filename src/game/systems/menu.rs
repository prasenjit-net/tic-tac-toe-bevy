use bevy::prelude::*;

use crate::game::components::*;
use crate::game::state::*;

// Type alias to reduce complexity
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
            BackgroundColor(BG_COLOR),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Tic-Tac-Toe"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(UI_ACCENT),
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
                TextColor(UI_ACCENT),
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
                    BackgroundColor(UI_BG),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::Human,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Human"),
                        text_style.clone(),
                        TextColor(UI_ACCENT),
                    ));
                });

            // Human vs Computer Easy
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(UI_BG),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::ComputerEasy,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Computer (Easy)"),
                        text_style.clone(),
                        TextColor(UI_ACCENT),
                    ));
                });

            // Human vs Computer Hard
            parent
                .spawn((
                    Button,
                    button_style.clone(),
                    BackgroundColor(UI_BG),
                    MenuButton {
                        x_type: PlayerType::Human,
                        o_type: PlayerType::ComputerHard,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Human vs Computer (Hard)"),
                        text_style.clone(),
                        TextColor(UI_ACCENT),
                    ));
                });

            // Computer vs Computer
            parent
                .spawn((
                    Button,
                    button_style,
                    BackgroundColor(UI_BG),
                    MenuButton {
                        x_type: PlayerType::ComputerEasy,
                        o_type: PlayerType::ComputerHard,
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Computer vs Computer"),
                        text_style,
                        TextColor(UI_ACCENT),
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
                *bg_color = BackgroundColor(UI_BORDER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(UI_BG);
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
