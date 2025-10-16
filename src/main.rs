use bevy::prelude::*;

mod game;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Tic-Tac-Toe (Bevy 0.17)".into(), resolution: (600u32, 700u32).into(), resizable: false, ..default() }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
