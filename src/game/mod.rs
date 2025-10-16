use bevy::prelude::*;

mod state;
mod components;
mod systems;
mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(state::BG_COLOR))
            .insert_resource(state::GameState::default())
            .add_systems(Startup, (systems::setup_camera, systems::spawn_grid))
            .add_systems(Update, (
                systems::handle_clicks,
                systems::draw_marks,
                systems::draw_win_highlight,
                systems::keyboard_reset,
            ));
    }
}
