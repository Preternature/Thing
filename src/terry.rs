//! Terry the MBA Hot Dog - dialogue reactions and personality

use bevy::prelude::*;
use bevy::ecs::schedule::IntoScheduleConfigs;
use crate::dialogue::{DialogueDatabase, DialogueLine};
use crate::game_state::{AppState, GameState, MilestoneEvent, MilestoneType, ThingProducedEvent};
use crate::thing_type::ThingType;

pub struct TerryPlugin;

impl Plugin for TerryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerryState>()
            .add_message::<TerryDialogueEvent>()
            .add_systems(OnEnter(AppState::Playing), terry_greet_on_start)
            .add_systems(
                Update,
                (
                    react_to_milestones,
                    react_to_clicks,
                    periodic_commentary,
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

/// Terry's current state
#[derive(Resource)]
pub struct TerryState {
    /// Current dialogue being displayed
    pub current_line: Option<DialogueLine>,
    /// Timer for how long current line has been shown
    pub line_timer: f32,
    /// Duration to show each line
    pub line_duration: f32,
    /// Timer for periodic commentary
    pub commentary_timer: f32,
    /// Clicks since last reaction
    pub clicks_since_reaction: u32,
}

impl Default for TerryState {
    fn default() -> Self {
        Self {
            current_line: None,
            line_timer: 0.0,
            line_duration: 5.0,
            commentary_timer: 0.0,
            clicks_since_reaction: 0,
        }
    }
}

/// Message to trigger Terry saying something
#[derive(Event, Message, Clone)]
pub struct TerryDialogueEvent {
    pub trigger: String,
}

/// Greet player when game starts
fn terry_greet_on_start(
    game_state: Res<GameState>,
    dialogue_db: Res<DialogueDatabase>,
    mut terry_state: ResMut<TerryState>,
) {
    // First, say hello
    if let Some(line) = dialogue_db.get_for_trigger("game_start") {
        terry_state.current_line = Some(line.clone());
        terry_state.line_timer = 0.0;
    }

    // Then queue up thing-type-specific greeting
    if let Some(thing_type) = game_state.thing_type {
        let trigger = match thing_type {
            ThingType::Cheap => "select_cheap",
            ThingType::Good => "select_good",
            ThingType::Expensive => "select_expensive",
            ThingType::Bad => "select_bad",
        };

        // This will be the next line after the greeting times out
        if let Some(_line) = dialogue_db.get_for_trigger(trigger) {
            // We'll handle this in the periodic commentary
            terry_state.commentary_timer = terry_state.line_duration + 1.0;
        }
    }
}

/// React to milestone achievements
fn react_to_milestones(
    mut milestone_events: MessageReader<MilestoneEvent>,
    dialogue_db: Res<DialogueDatabase>,
    mut terry_state: ResMut<TerryState>,
) {
    for event in milestone_events.read() {
        let trigger = match event.milestone_type {
            MilestoneType::ThingsProduced(10) => "things_10",
            MilestoneType::ThingsProduced(100) => "things_100",
            MilestoneType::ThingsProduced(1000) => "things_1000",
            MilestoneType::ThingsProduced(10000) => "things_10000",
            MilestoneType::MoneyEarned(100) => "money_100",
            MilestoneType::MoneyEarned(1000) => "money_1000",
            _ => continue,
        };

        if let Some(line) = dialogue_db.get_for_trigger(trigger) {
            terry_state.current_line = Some(line.clone());
            terry_state.line_timer = 0.0;
        }
    }
}

/// React to player clicks
fn react_to_clicks(
    mut thing_events: MessageReader<ThingProducedEvent>,
    dialogue_db: Res<DialogueDatabase>,
    mut terry_state: ResMut<TerryState>,
) {
    for event in thing_events.read() {
        if event.from_click {
            terry_state.clicks_since_reaction += 1;

            // React every 10 clicks
            if terry_state.clicks_since_reaction >= 10 {
                terry_state.clicks_since_reaction = 0;

                if let Some(line) = dialogue_db.get_for_trigger("click") {
                    terry_state.current_line = Some(line.clone());
                    terry_state.line_timer = 0.0;
                }
            }
        }
    }
}

/// Periodic commentary based on game state
fn periodic_commentary(
    time: Res<Time>,
    game_state: Res<GameState>,
    dialogue_db: Res<DialogueDatabase>,
    mut terry_state: ResMut<TerryState>,
) {
    terry_state.line_timer += time.delta_secs();
    terry_state.commentary_timer += time.delta_secs();

    // Only give commentary if current line has timed out
    if terry_state.line_timer >= terry_state.line_duration {
        // Commentary every 15-20 seconds
        if terry_state.commentary_timer >= 15.0 {
            terry_state.commentary_timer = 0.0;

            // Pick contextual commentary based on Thing type
            let trigger = match game_state.thing_type {
                Some(ThingType::Cheap) => "cheap_playing",
                Some(ThingType::Good) => "good_playing",
                Some(ThingType::Expensive) => "expensive_playing",
                Some(ThingType::Bad) => {
                    if game_state.reputation < 1.5 {
                        "bad_low_rep"
                    } else {
                        "bad_playing"
                    }
                }
                None => "idle",
            };

            if let Some(line) = dialogue_db.get_for_trigger(trigger) {
                terry_state.current_line = Some(line.clone());
                terry_state.line_timer = 0.0;
            }
        }
    }
}
