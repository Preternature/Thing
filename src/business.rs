//! Business simulation systems - money, marketing, reputation, customers

use bevy::prelude::*;
use bevy::ecs::schedule::IntoScheduleConfigs;
use crate::game_state::{AppState, GameState, ThingProducedEvent, MoneyChangedEvent, ReputationChangedEvent};
use crate::thing_type::ThingType;

pub struct BusinessPlugin;

impl Plugin for BusinessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                process_sales,
                update_reputation,
                apply_reputation_decay,
            )
                .run_if(in_state(AppState::Playing)),
        );
    }
}

/// Process sales when Things are produced
fn process_sales(
    mut game_state: ResMut<GameState>,
    mut thing_events: MessageReader<ThingProducedEvent>,
    mut money_events: MessageWriter<MoneyChangedEvent>,
    mut rep_events: MessageWriter<ReputationChangedEvent>,
) {
    for event in thing_events.read() {
        if let Some(thing_type) = game_state.thing_type {
            // Calculate revenue based on Thing type and marketing
            let base_price = thing_type.base_price();
            let marketing_bonus = 1.0 + (game_state.marketing_level as f64 * 0.1);
            let reputation_bonus = game_state.reputation as f64 / 2.5; // 2.5 rep = 1x, 5 rep = 2x

            let revenue = event.amount as f64 * base_price * marketing_bonus * reputation_bonus;
            let _old_money = game_state.money;
            game_state.money += revenue;
            game_state.customers_served += event.amount;

            money_events.write(MoneyChangedEvent {
                new_amount: game_state.money,
                delta: revenue,
            });

            // Update reputation based on Thing type
            let rep_change = thing_type.reputation_per_sale() * event.amount as f32;
            let old_rep = game_state.reputation;
            game_state.reputation = (game_state.reputation + rep_change).clamp(0.0, 5.0);

            if (game_state.reputation - old_rep).abs() > 0.001 {
                rep_events.write(ReputationChangedEvent {
                    new_reputation: game_state.reputation,
                });
            }
        }
    }
}

/// Update reputation based on various factors
fn update_reputation(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if let Some(thing_type) = game_state.thing_type {
        // Natural reputation growth for non-Bad Things when marketing
        if thing_type != ThingType::Bad && game_state.marketing_level > 0 {
            let marketing_rep_gain = 0.001 * game_state.marketing_level as f32 * time.delta_secs();
            game_state.reputation = (game_state.reputation + marketing_rep_gain).clamp(0.0, 5.0);
        }
    }
}

/// Apply reputation decay for Bad Things
fn apply_reputation_decay(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut rep_events: MessageWriter<ReputationChangedEvent>,
) {
    if let Some(thing_type) = game_state.thing_type {
        let decay = thing_type.reputation_decay() * time.delta_secs();
        if decay > 0.0 {
            let old_rep = game_state.reputation;
            game_state.reputation = (game_state.reputation - decay).max(0.0);

            if (game_state.reputation - old_rep).abs() > 0.01 {
                rep_events.write(ReputationChangedEvent {
                    new_reputation: game_state.reputation,
                });
            }
        }
    }
}

/// Upgrade types for the business
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeType {
    BetterTools,      // Increases click power
    HireWorker,       // Increases things per second
    Automation,       // Big boost to things per second
    SocialMedia,      // Marketing level +1
    Billboard,        // Marketing level +2
    InfluencerDeal,   // Marketing level +3
}

impl UpgradeType {
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeType::BetterTools => "Better Tools",
            UpgradeType::HireWorker => "Hire Worker",
            UpgradeType::Automation => "Automation",
            UpgradeType::SocialMedia => "Social Media",
            UpgradeType::Billboard => "Billboard",
            UpgradeType::InfluencerDeal => "Influencer Deal",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UpgradeType::BetterTools => "+1 Thing per click",
            UpgradeType::HireWorker => "+0.5 Things per second",
            UpgradeType::Automation => "+2 Things per second",
            UpgradeType::SocialMedia => "+1 Marketing Level",
            UpgradeType::Billboard => "+2 Marketing Level",
            UpgradeType::InfluencerDeal => "+3 Marketing Level",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            UpgradeType::BetterTools => 50.0,
            UpgradeType::HireWorker => 100.0,
            UpgradeType::Automation => 500.0,
            UpgradeType::SocialMedia => 75.0,
            UpgradeType::Billboard => 300.0,
            UpgradeType::InfluencerDeal => 1000.0,
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self, UpgradeType::BetterTools | UpgradeType::HireWorker | UpgradeType::Automation)
    }

    pub fn is_marketing(&self) -> bool {
        matches!(self, UpgradeType::SocialMedia | UpgradeType::Billboard | UpgradeType::InfluencerDeal)
    }
}

/// Resource tracking upgrade counts
#[derive(Resource, Default)]
pub struct UpgradeState {
    pub better_tools: u32,
    pub workers: u32,
    pub automation: u32,
    pub social_media: u32,
    pub billboards: u32,
    pub influencer_deals: u32,
}

impl UpgradeState {
    pub fn get_count(&self, upgrade: UpgradeType) -> u32 {
        match upgrade {
            UpgradeType::BetterTools => self.better_tools,
            UpgradeType::HireWorker => self.workers,
            UpgradeType::Automation => self.automation,
            UpgradeType::SocialMedia => self.social_media,
            UpgradeType::Billboard => self.billboards,
            UpgradeType::InfluencerDeal => self.influencer_deals,
        }
    }

    pub fn cost(&self, upgrade: UpgradeType) -> f64 {
        let count = self.get_count(upgrade);
        upgrade.base_cost() * 1.15_f64.powi(count as i32)
    }

    pub fn purchase(&mut self, upgrade: UpgradeType, game_state: &mut GameState) -> bool {
        let cost = self.cost(upgrade);
        if game_state.money >= cost {
            game_state.money -= cost;

            match upgrade {
                UpgradeType::BetterTools => {
                    self.better_tools += 1;
                    game_state.click_power += 1;
                }
                UpgradeType::HireWorker => {
                    self.workers += 1;
                    game_state.things_per_second += 0.5;
                }
                UpgradeType::Automation => {
                    self.automation += 1;
                    game_state.things_per_second += 2.0;
                }
                UpgradeType::SocialMedia => {
                    self.social_media += 1;
                    game_state.marketing_level += 1;
                }
                UpgradeType::Billboard => {
                    self.billboards += 1;
                    game_state.marketing_level += 2;
                }
                UpgradeType::InfluencerDeal => {
                    self.influencer_deals += 1;
                    game_state.marketing_level += 3;
                }
            }
            true
        } else {
            false
        }
    }
}
