//! Thing Simulator 2012
//! A comedy business simulator featuring Terry, an anthropomorphic hot dog with an MBA

mod business;
mod clicker;
mod dialogue;
mod game_state;
mod terry;
mod thing_type;
mod ui;

use bevy::prelude::*;
use game_state::{AppState, GameStatePlugin};
use business::BusinessPlugin;
use clicker::ClickerPlugin;
use dialogue::DialoguePlugin;
use terry::TerryPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Thing Simulator 2012".into(),
                resolution: (1024, 768).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins((
            GameStatePlugin,
            DialoguePlugin,
            TerryPlugin,
            BusinessPlugin,
            ClickerPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
