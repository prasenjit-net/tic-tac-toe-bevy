use bevy::prelude::*;

mod components;
mod state;
mod systems;
mod utils;

use state::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(state::BG_COLOR))
            .init_state::<AppState>()
            .init_resource::<state::GameState>()
            .init_resource::<state::PlayerConfig>()
            .init_resource::<state::Score>()
            .init_resource::<state::ComputerMoveTimer>()
            .add_systems(Startup, systems::setup_camera)
            // Menu state systems
            .add_systems(OnEnter(AppState::Menu), systems::spawn_menu)
            .add_systems(
                Update,
                systems::handle_menu_buttons.run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), systems::cleanup_menu)
            // Playing state systems
            .add_systems(
                OnEnter(AppState::Playing),
                (systems::spawn_grid, systems::spawn_scoreboard),
            )
            .add_systems(
                Update,
                (
                    systems::handle_clicks,
                    systems::computer_player,
                    systems::draw_marks,
                    systems::draw_win_highlight,
                    systems::update_score,
                    systems::update_scoreboard,
                    systems::show_game_over_ui,
                    systems::handle_game_over_buttons,
                    systems::keyboard_controls,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(AppState::Playing), systems::cleanup_game);
    }
}
