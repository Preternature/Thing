//! Clicker mechanics - click to produce, auto-production

use bevy::prelude::*;
use bevy::ecs::schedule::IntoScheduleConfigs;
use crate::game_state::{AppState, GameState, ThingProducedEvent};

pub struct ClickerPlugin;

impl Plugin for ClickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AutoProductionAccumulator>()
            .add_systems(Update, auto_produce.run_if(in_state(AppState::Playing)));
    }
}

/// Accumulator for fractional production
#[derive(Resource, Default)]
pub struct AutoProductionAccumulator {
    pub accumulated: f64,
}

/// Auto-produce Things over time
fn auto_produce(
    time: Res<Time>,
    mut accumulator: ResMut<AutoProductionAccumulator>,
    mut thing_events: MessageWriter<ThingProducedEvent>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.things_per_second > 0.0 {
        // Apply production multiplier from Thing type
        let multiplier = game_state
            .thing_type
            .map(|t| t.production_multiplier())
            .unwrap_or(1.0);

        let production = game_state.things_per_second * multiplier * time.delta_secs() as f64;
        accumulator.accumulated += production;

        // Convert accumulated to whole Things
        let whole_things = accumulator.accumulated.floor() as u64;
        if whole_things > 0 {
            accumulator.accumulated -= whole_things as f64;
            game_state.things_produced += whole_things;

            thing_events.write(ThingProducedEvent {
                amount: whole_things,
                from_click: false,
            });
        }
    }
}

/// Message to trigger a manual click
#[derive(Event, Message, Clone)]
pub struct ClickEvent;

/// System to handle manual clicks
pub fn handle_click(
    mut click_events: MessageReader<ClickEvent>,
    mut game_state: ResMut<GameState>,
    mut thing_events: MessageWriter<ThingProducedEvent>,
) {
    for _ in click_events.read() {
        if let Some(thing_type) = game_state.thing_type {
            let multiplier = thing_type.production_multiplier();
            let things = (game_state.click_power as f64 * multiplier).ceil() as u64;

            game_state.things_produced += things;

            thing_events.write(ThingProducedEvent {
                amount: things,
                from_click: true,
            });
        }
    }
}
