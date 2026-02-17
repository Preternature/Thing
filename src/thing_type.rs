//! The four types of Things you can sell

use bevy::prelude::*;

/// The type of Thing the player is selling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThingType {
    /// High volume, low margins, mass market appeal
    #[default]
    Cheap,
    /// Quality focus, builds reputation slowly, loyal customers
    Good,
    /// Luxury positioning, low volume, heavy marketing needed
    Expensive,
    /// Scam mode, quick cash, reputation crashes
    Bad,
}

impl ThingType {
    /// Base price per Thing
    pub fn base_price(&self) -> f64 {
        match self {
            ThingType::Cheap => 0.50,
            ThingType::Good => 5.00,
            ThingType::Expensive => 50.00,
            ThingType::Bad => 10.00,
        }
    }

    /// Production speed multiplier
    pub fn production_multiplier(&self) -> f64 {
        match self {
            ThingType::Cheap => 2.0,
            ThingType::Good => 1.0,
            ThingType::Expensive => 0.5,
            ThingType::Bad => 1.5,
        }
    }

    /// Customer volume multiplier (how many customers come)
    pub fn customer_multiplier(&self) -> f64 {
        match self {
            ThingType::Cheap => 2.0,      // Lots of customers
            ThingType::Good => 1.0,       // Normal flow
            ThingType::Expensive => 0.3,  // Few but wealthy
            ThingType::Bad => 1.5,        // Starts high, will crash
        }
    }

    /// Reputation change per sale
    pub fn reputation_per_sale(&self) -> f32 {
        match self {
            ThingType::Cheap => 0.001,     // Slow reputation gain
            ThingType::Good => 0.01,       // Good reputation gain
            ThingType::Expensive => 0.005, // Medium reputation
            ThingType::Bad => -0.02,       // Reputation LOSS
        }
    }

    /// Passive reputation decay per second
    pub fn reputation_decay(&self) -> f32 {
        match self {
            ThingType::Cheap => 0.0,
            ThingType::Good => 0.0,
            ThingType::Expensive => 0.0,
            ThingType::Bad => 0.005, // Bad Things cause passive decay
        }
    }

    /// Display name
    pub fn name(&self) -> &'static str {
        match self {
            ThingType::Cheap => "Cheap",
            ThingType::Good => "Good",
            ThingType::Expensive => "Expensive",
            ThingType::Bad => "Bad",
        }
    }

    /// Description for selection screen
    pub fn description(&self) -> &'static str {
        match self {
            ThingType::Cheap => "High volume, low margins. The people's Thing.",
            ThingType::Good => "Quality craftsmanship. Slow and steady wins the race.",
            ThingType::Expensive => "Luxury positioning. For the discerning Thing enthusiast.",
            ThingType::Bad => "Quick cash. What could possibly go wrong?",
        }
    }

    /// Color for UI
    pub fn color(&self) -> Color {
        match self {
            ThingType::Cheap => Color::srgb(0.2, 0.7, 0.3),      // Green
            ThingType::Good => Color::srgb(0.3, 0.5, 0.9),       // Blue
            ThingType::Expensive => Color::srgb(0.8, 0.6, 0.1),  // Gold
            ThingType::Bad => Color::srgb(0.8, 0.2, 0.2),        // Red
        }
    }
}
