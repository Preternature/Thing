//! Player-controllable economic levers
//!
//! These are the things the player CAN control, unlike the invisible world forces.

use bevy::prelude::*;

/// All the marketing and business levers the player can pull
#[derive(Resource)]
pub struct MarketingState {
    // === ADVERTISING ===
    /// Newspaper ads (cheap, local reach)
    pub newspaper_ads: AdvertisingCampaign,
    /// Radio spots
    pub radio_ads: AdvertisingCampaign,
    /// TV commercials (expensive, massive reach)
    pub tv_ads: AdvertisingCampaign,
    /// Internet ads (targeted, scalable)
    pub internet_ads: AdvertisingCampaign,
    /// Billboard/outdoor advertising
    pub billboard_ads: AdvertisingCampaign,

    // === INFLUENCER MARKETING ===
    /// Local micro-influencers
    pub micro_influencers: InfluencerDeal,
    /// Mid-tier influencers
    pub mid_influencers: InfluencerDeal,
    /// Celebrity endorsements
    pub celebrity_endorsement: InfluencerDeal,

    // === BACKROOM DEALS ===
    /// Retail store placement deals
    pub retail_placement: BackroomDeal,
    /// Distributor kickbacks
    pub distributor_deals: BackroomDeal,
    /// Supplier exclusivity
    pub supplier_exclusivity: BackroomDeal,
    /// "Consulting fees" to decision makers
    pub consulting_fees: BackroomDeal,

    // === MARKET MANIPULATION ===
    /// Artificial scarcity tactics
    pub artificial_scarcity: ManipulationTactic,
    /// Astroturfing (fake grassroots buzz)
    pub astroturfing: ManipulationTactic,
    /// Review manipulation
    pub review_manipulation: ManipulationTactic,
    /// Competitor sabotage
    pub competitor_sabotage: ManipulationTactic,

    // === PRICING STRATEGIES ===
    /// Current price multiplier
    pub price_multiplier: f32,
    /// Loss leader strategy active
    pub loss_leader: bool,
    /// Premium pricing psychology
    pub premium_positioning: bool,

    // === PR & MEDIA ===
    /// Press releases frequency
    pub pr_intensity: f32,
    /// Crisis management fund
    pub crisis_fund: f32,
    /// Media relationship strength
    pub media_relationships: f32,

    // === LOYALTY & RETENTION ===
    /// Loyalty program level
    pub loyalty_program: u8,
    /// Referral bonus amount
    pub referral_bonus: f32,
}

#[derive(Clone, Default)]
pub struct AdvertisingCampaign {
    /// Is this campaign active?
    pub active: bool,
    /// Spending per day
    pub daily_spend: f32,
    /// Campaign effectiveness (improves with experience)
    pub effectiveness: f32,
    /// Total spent historically
    pub lifetime_spend: f32,
    /// Reach multiplier
    pub reach: f32,
}

impl AdvertisingCampaign {
    pub fn contribution(&self) -> f32 {
        if self.active {
            self.daily_spend * self.effectiveness * self.reach
        } else {
            0.0
        }
    }
}

#[derive(Clone, Default)]
pub struct InfluencerDeal {
    /// Is there an active deal?
    pub active: bool,
    /// Cost per post/mention
    pub cost_per_post: f32,
    /// Follower reach
    pub follower_reach: u64,
    /// Authenticity score (affects conversion)
    pub authenticity: f32,
    /// Posts remaining in deal
    pub posts_remaining: u32,
}

impl InfluencerDeal {
    pub fn contribution(&self) -> f32 {
        if self.active && self.posts_remaining > 0 {
            (self.follower_reach as f32 / 1_000_000.0) * self.authenticity
        } else {
            0.0
        }
    }
}

#[derive(Clone, Default)]
pub struct BackroomDeal {
    /// Is the deal active?
    pub active: bool,
    /// Monthly "fee"
    pub monthly_cost: f32,
    /// Strength of the arrangement
    pub strength: f32,
    /// Risk of exposure (0-1)
    pub exposure_risk: f32,
    /// Months remaining
    pub months_remaining: u32,
}

impl BackroomDeal {
    pub fn contribution(&self) -> f32 {
        if self.active {
            self.strength * (1.0 - self.exposure_risk * 0.5)
        } else {
            0.0
        }
    }
}

#[derive(Clone, Default)]
pub struct ManipulationTactic {
    /// Is this tactic in use?
    pub active: bool,
    /// Intensity (0-1)
    pub intensity: f32,
    /// Risk of backlash
    pub backlash_risk: f32,
    /// Accumulated suspicion
    pub suspicion: f32,
}

impl ManipulationTactic {
    pub fn contribution(&self) -> f32 {
        if self.active {
            self.intensity * (1.0 - self.suspicion)
        } else {
            0.0
        }
    }
}

impl Default for MarketingState {
    fn default() -> Self {
        Self {
            // Advertising - all start inactive
            newspaper_ads: AdvertisingCampaign {
                effectiveness: 0.5,
                reach: 0.1,
                ..default()
            },
            radio_ads: AdvertisingCampaign {
                effectiveness: 0.7,
                reach: 0.3,
                ..default()
            },
            tv_ads: AdvertisingCampaign {
                effectiveness: 1.5,
                reach: 1.0,
                ..default()
            },
            internet_ads: AdvertisingCampaign {
                effectiveness: 1.0,
                reach: 0.5,
                ..default()
            },
            billboard_ads: AdvertisingCampaign {
                effectiveness: 0.3,
                reach: 0.2,
                ..default()
            },

            // Influencers
            micro_influencers: InfluencerDeal {
                cost_per_post: 50.0,
                follower_reach: 10_000,
                authenticity: 0.9,
                ..default()
            },
            mid_influencers: InfluencerDeal {
                cost_per_post: 500.0,
                follower_reach: 100_000,
                authenticity: 0.7,
                ..default()
            },
            celebrity_endorsement: InfluencerDeal {
                cost_per_post: 50_000.0,
                follower_reach: 10_000_000,
                authenticity: 0.3,
                ..default()
            },

            // Backroom deals
            retail_placement: BackroomDeal {
                monthly_cost: 1_000.0,
                strength: 1.2,
                exposure_risk: 0.05,
                ..default()
            },
            distributor_deals: BackroomDeal {
                monthly_cost: 2_000.0,
                strength: 1.5,
                exposure_risk: 0.1,
                ..default()
            },
            supplier_exclusivity: BackroomDeal {
                monthly_cost: 5_000.0,
                strength: 1.3,
                exposure_risk: 0.15,
                ..default()
            },
            consulting_fees: BackroomDeal {
                monthly_cost: 10_000.0,
                strength: 2.0,
                exposure_risk: 0.3,
                ..default()
            },

            // Manipulation
            artificial_scarcity: ManipulationTactic {
                intensity: 0.5,
                backlash_risk: 0.2,
                ..default()
            },
            astroturfing: ManipulationTactic {
                intensity: 0.7,
                backlash_risk: 0.4,
                ..default()
            },
            review_manipulation: ManipulationTactic {
                intensity: 0.6,
                backlash_risk: 0.5,
                ..default()
            },
            competitor_sabotage: ManipulationTactic {
                intensity: 0.8,
                backlash_risk: 0.7,
                ..default()
            },

            // Pricing
            price_multiplier: 1.0,
            loss_leader: false,
            premium_positioning: false,

            // PR
            pr_intensity: 0.0,
            crisis_fund: 0.0,
            media_relationships: 0.0,

            // Loyalty
            loyalty_program: 0,
            referral_bonus: 0.0,
        }
    }
}

impl MarketingState {
    /// Calculate the total marketing boost to demand
    pub fn calculate_demand_boost(&self) -> f32 {
        let mut boost = 1.0;

        // Advertising contributions
        boost += self.newspaper_ads.contribution() * 0.001;
        boost += self.radio_ads.contribution() * 0.002;
        boost += self.tv_ads.contribution() * 0.005;
        boost += self.internet_ads.contribution() * 0.003;
        boost += self.billboard_ads.contribution() * 0.001;

        // Influencer contributions
        boost += self.micro_influencers.contribution() * 0.05;
        boost += self.mid_influencers.contribution() * 0.1;
        boost += self.celebrity_endorsement.contribution() * 0.3;

        // Backroom deals
        boost *= 1.0 + self.retail_placement.contribution() * 0.1;
        boost *= 1.0 + self.distributor_deals.contribution() * 0.15;
        boost *= 1.0 + self.supplier_exclusivity.contribution() * 0.05;
        boost *= 1.0 + self.consulting_fees.contribution() * 0.2;

        // Manipulation tactics
        boost *= 1.0 + self.artificial_scarcity.contribution() * 0.2;
        boost *= 1.0 + self.astroturfing.contribution() * 0.15;
        boost *= 1.0 + self.review_manipulation.contribution() * 0.1;
        boost *= 1.0 + self.competitor_sabotage.contribution() * 0.25;

        // Pricing psychology
        if self.loss_leader {
            boost *= 1.5; // More sales, less profit per unit
        }
        if self.premium_positioning {
            boost *= 0.7; // Fewer sales, more profit per unit
        }

        // PR
        boost *= 1.0 + self.pr_intensity * 0.1;
        boost *= 1.0 + self.media_relationships * 0.2;

        // Loyalty
        boost *= 1.0 + self.loyalty_program as f32 * 0.05;
        boost *= 1.0 + self.referral_bonus * 0.001;

        boost
    }

    /// Calculate daily marketing costs
    pub fn calculate_daily_costs(&self) -> f32 {
        let mut costs = 0.0;

        if self.newspaper_ads.active { costs += self.newspaper_ads.daily_spend; }
        if self.radio_ads.active { costs += self.radio_ads.daily_spend; }
        if self.tv_ads.active { costs += self.tv_ads.daily_spend; }
        if self.internet_ads.active { costs += self.internet_ads.daily_spend; }
        if self.billboard_ads.active { costs += self.billboard_ads.daily_spend; }

        // Monthly costs converted to daily
        if self.retail_placement.active { costs += self.retail_placement.monthly_cost / 30.0; }
        if self.distributor_deals.active { costs += self.distributor_deals.monthly_cost / 30.0; }
        if self.supplier_exclusivity.active { costs += self.supplier_exclusivity.monthly_cost / 30.0; }
        if self.consulting_fees.active { costs += self.consulting_fees.monthly_cost / 30.0; }

        costs
    }
}

pub struct MarketingPlugin;

impl Plugin for MarketingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketingState>();
    }
}
