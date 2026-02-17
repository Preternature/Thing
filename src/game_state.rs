//! Game state management

use bevy::prelude::*;
use bevy::ecs::schedule::IntoScheduleConfigs;
use crate::thing_type::ThingType;

/// The main game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    ThingSelection,
    Playing,
    Paused,
}

/// Core game state resource
#[derive(Resource, Debug)]
pub struct GameState {
    /// The type of Thing the player is selling
    pub thing_type: Option<ThingType>,
    /// Total Things produced (lifetime)
    pub things_produced: u64,
    /// Current money
    pub money: f64,
    /// Reputation (0.0 to 5.0, like star rating)
    pub reputation: f32,
    /// Marketing level (affects customer flow)
    pub marketing_level: u32,
    /// Things produced per second (auto-production)
    pub things_per_second: f64,
    /// Click power (Things per click)
    pub click_power: u64,
    /// Customers served
    pub customers_served: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            thing_type: None,
            things_produced: 0,
            money: 100.0, // Starting capital (questionable sources)
            reputation: 2.5, // Starting at middle reputation
            marketing_level: 0,
            things_per_second: 0.0,
            click_power: 1,
            customers_served: 0,
        }
    }
}

/// Message fired when the player produces Things
#[derive(Event, Message, Clone)]
pub struct ThingProducedEvent {
    pub amount: u64,
    pub from_click: bool,
}

/// Message fired when money changes
#[derive(Event, Message, Clone)]
pub struct MoneyChangedEvent {
    pub new_amount: f64,
    pub delta: f64,
}

/// Message fired when reputation changes
#[derive(Event, Message, Clone)]
pub struct ReputationChangedEvent {
    pub new_reputation: f32,
}

/// Message for milestone achievements
#[derive(Event, Message, Clone)]
pub struct MilestoneEvent {
    pub milestone_type: MilestoneType,
}

#[derive(Debug, Clone, Copy)]
pub enum MilestoneType {
    ThingsProduced(u64),
    MoneyEarned(u64),
    CustomersServed(u64),
    ReputationReached(u8),
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_message::<ThingProducedEvent>()
            .add_message::<MoneyChangedEvent>()
            .add_message::<ReputationChangedEvent>()
            .add_message::<MilestoneEvent>()
            .add_systems(Update, check_milestones.run_if(in_state(AppState::Playing)));
    }
}

/// Check for milestone achievements
fn check_milestones(
    game_state: Res<GameState>,
    mut milestone_events: MessageWriter<MilestoneEvent>,
    mut last_things: Local<u64>,
    mut last_money: Local<u64>,
) {
    let milestones = [10, 100, 1000, 10000, 100000, 1000000];

    // Check things produced milestones
    for &milestone in &milestones {
        if game_state.things_produced >= milestone && *last_things < milestone {
            milestone_events.write(MilestoneEvent {
                milestone_type: MilestoneType::ThingsProduced(milestone),
            });
        }
    }
    *last_things = game_state.things_produced;

    // Check money milestones
    let money_rounded = game_state.money as u64;
    for &milestone in &milestones {
        if money_rounded >= milestone && *last_money < milestone {
            milestone_events.write(MilestoneEvent {
                milestone_type: MilestoneType::MoneyEarned(milestone),
            });
        }
    }
    *last_money = money_rounded;
}
