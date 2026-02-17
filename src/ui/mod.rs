//! UI module - all user interface components

mod main_screen;
mod selection;
mod terry_box;

use bevy::prelude::*;
use bevy::ecs::schedule::IntoScheduleConfigs;
use crate::game_state::AppState;
use crate::business::UpgradeState;
use crate::clicker::ClickEvent;

pub use main_screen::*;
pub use selection::*;
pub use terry_box::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UpgradeState>()
            .add_message::<ClickEvent>()
            .add_systems(OnEnter(AppState::ThingSelection), setup_selection_screen)
            .add_systems(OnExit(AppState::ThingSelection), cleanup_selection_screen)
            .add_systems(
                Update,
                (
                    handle_selection_buttons,
                    update_selection_timer,
                ).run_if(in_state(AppState::ThingSelection)),
            )
            .add_systems(OnEnter(AppState::Playing), setup_main_screen)
            .add_systems(OnExit(AppState::Playing), cleanup_main_screen)
            .add_systems(
                Update,
                (
                    update_stats_display,
                    update_terry_dialogue,
                    handle_make_thing_button,
                    handle_upgrade_buttons,
                ).run_if(in_state(AppState::Playing)),
            );
    }
}

/// Marker component for UI elements to clean up
#[derive(Component)]
pub struct UiRoot;

/// Common button colors
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);
pub const DISABLED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
