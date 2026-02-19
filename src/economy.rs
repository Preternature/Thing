//! Economic simulation - the invisible hand that moves everything
//!
//! Most of these variables are invisible to the player but affect everything.
//! The player can only control their own actions; the world moves on without them.

use bevy::prelude::*;

/// The current state of the world - most of this is invisible to the player
#[derive(Resource)]
pub struct WorldState {
    // === TIME ===
    /// Current game date (starts Jan 1, 2012)
    pub date: GameDate,
    /// How many real seconds equal one game day
    pub time_scale: f32,
    /// Accumulated time for day progression
    pub day_accumulator: f32,

    // === INVISIBLE ENVIRONMENTAL FACTORS ===
    /// Current temperature in Fahrenheit (affects consumer behavior)
    pub temperature: f32,
    /// Base temperature for current season (temperature oscillates around this)
    pub seasonal_base_temp: f32,
    /// Random daily temperature variance
    pub temp_variance: f32,

    // === INVISIBLE GLOBAL FACTORS ===
    /// World population (grows slowly, affects total addressable market)
    pub global_population: f64,
    /// Daily population growth rate
    pub population_growth_rate: f64,

    // === INVISIBLE ECONOMIC INDICATORS ===
    /// Consumer confidence index (0.0 - 2.0, 1.0 is neutral)
    pub consumer_confidence: f32,
    /// Unemployment rate (affects spending)
    pub unemployment_rate: f32,
    /// Inflation rate (affects perceived value)
    pub inflation_rate: f32,
    /// Stock market sentiment (-1.0 to 1.0)
    pub market_sentiment: f32,

    // === INVISIBLE SOCIAL FACTORS ===
    /// Current "trend momentum" - how much Things are in vogue
    pub trend_factor: f32,
    /// Viral coefficient - chance of word-of-mouth spread
    pub viral_coefficient: f32,
    /// Media attention level (0.0 - 1.0)
    pub media_buzz: f32,

    // === INVISIBLE COMPETITOR FACTORS ===
    /// How aggressive competitors are being
    pub competitor_pressure: f32,
    /// Market saturation (0.0 = blue ocean, 1.0 = red ocean)
    pub market_saturation: f32,

    // === CYCLICAL INVISIBLE FACTORS ===
    /// Days until Christmas (huge demand modifier)
    pub days_to_christmas: i32,
    /// Is it a weekend? (affects foot traffic)
    pub is_weekend: bool,
    /// Is it a holiday? (various effects)
    pub current_holiday: Option<Holiday>,
    /// Day of week (0 = Sunday)
    pub day_of_week: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct GameDate {
    pub year: i32,
    pub month: u8,  // 1-12
    pub day: u8,    // 1-31
}

impl GameDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Days in the current month
    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => 30,
        }
    }

    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    /// Advance by one day
    pub fn advance(&mut self) {
        self.day += 1;
        if self.day > self.days_in_month() {
            self.day = 1;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
    }

    /// Day of year (1-366)
    pub fn day_of_year(&self) -> u16 {
        let days_before_month: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut day = days_before_month[(self.month - 1) as usize] + self.day as u16;
        if self.month > 2 && self.is_leap_year() {
            day += 1;
        }
        day
    }

    /// Days until December 25th
    pub fn days_until_christmas(&self) -> i32 {
        let christmas_day = if self.month == 12 && self.day > 25 {
            // After Christmas, count to next year
            let days_left_in_year = if self.is_leap_year() { 366 } else { 365 } - self.day_of_year() as i32;
            days_left_in_year + 359 // Jan 1 to Dec 25
        } else {
            // Dec 25 is day 359 (or 360 in leap year)
            let christmas_doy = if self.is_leap_year() && self.month <= 2 { 360 } else { 359 };
            christmas_doy - self.day_of_year() as i32
        };
        christmas_day.max(0)
    }

    /// Calculate day of week (0 = Sunday, using Zeller's congruence)
    pub fn day_of_week(&self) -> u8 {
        let mut y = self.year;
        let mut m = self.month as i32;
        if m < 3 {
            m += 12;
            y -= 1;
        }
        let q = self.day as i32;
        let k = y % 100;
        let j = y / 100;
        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        ((h + 6) % 7) as u8 // Convert to Sunday = 0
    }

    pub fn format(&self) -> String {
        let month_name = match self.month {
            1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
            5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
            9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
            _ => "???",
        };
        format!("{} {}, {}", month_name, self.day, self.year)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holiday {
    NewYears,
    ValentinesDay,
    PresidentsDay,
    Easter,
    MemorialDay,
    IndependenceDay,
    LaborDay,
    Halloween,
    Thanksgiving,
    BlackFriday,
    Christmas,
    NewYearsEve,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            // Start on January 1, 2012
            date: GameDate::new(2012, 1, 1),
            time_scale: 1.0, // 1 real second = 1 game day
            day_accumulator: 0.0,

            // January temperature (cold)
            temperature: 35.0,
            seasonal_base_temp: 35.0,
            temp_variance: 0.0,

            // 2012 world population was ~7.0 billion
            global_population: 7_000_000_000.0,
            population_growth_rate: 1.0001, // ~0.01% daily growth

            // Economic indicators
            consumer_confidence: 1.0,
            unemployment_rate: 0.08, // 8% (2012 was still recovering)
            inflation_rate: 0.02,    // 2%
            market_sentiment: 0.0,

            // Social factors
            trend_factor: 1.0,
            viral_coefficient: 0.01,
            media_buzz: 0.0,

            // Competition
            competitor_pressure: 0.5,
            market_saturation: 0.3,

            // Cyclical
            days_to_christmas: 359, // Will be calculated
            is_weekend: false,      // Jan 1, 2012 was a Sunday
            current_holiday: Some(Holiday::NewYears),
            day_of_week: 0,
        }
    }
}

impl WorldState {
    /// Calculate seasonal base temperature based on month
    fn calculate_seasonal_temp(&self) -> f32 {
        // Northern hemisphere seasonal cycle
        // Coldest in January, warmest in July
        let month = self.date.month as f32;
        let day_of_month = self.date.day as f32;

        // Approximate day of year as continuous value
        let year_progress = (month - 1.0 + day_of_month / 30.0) / 12.0;

        // Temperature oscillates: coldest at year_progress ~= 0.04 (early Jan)
        // Warmest at year_progress ~= 0.54 (mid July)
        let temp_cycle = (std::f32::consts::PI * 2.0 * (year_progress - 0.04)).cos();

        // Range from ~30°F (winter) to ~85°F (summer), centered at ~57.5°F
        57.5 - (temp_cycle * 27.5)
    }

    /// Check what holiday (if any) is today
    fn check_holiday(&self) -> Option<Holiday> {
        let m = self.date.month;
        let d = self.date.day;

        match (m, d) {
            (1, 1) => Some(Holiday::NewYears),
            (2, 14) => Some(Holiday::ValentinesDay),
            (7, 4) => Some(Holiday::IndependenceDay),
            (10, 31) => Some(Holiday::Halloween),
            (12, 25) => Some(Holiday::Christmas),
            (12, 31) => Some(Holiday::NewYearsEve),
            // Approximate floating holidays
            (2, 15..=21) if self.day_of_week == 1 => Some(Holiday::PresidentsDay), // 3rd Monday Feb
            (5, 25..=31) if self.day_of_week == 1 => Some(Holiday::MemorialDay),   // Last Monday May
            (9, 1..=7) if self.day_of_week == 1 => Some(Holiday::LaborDay),        // 1st Monday Sep
            (11, 22..=28) if self.day_of_week == 4 => Some(Holiday::Thanksgiving), // 4th Thursday Nov
            (11, 23..=29) if self.day_of_week == 5 => Some(Holiday::BlackFriday),  // Day after Thanksgiving
            // Easter is complicated, skip for now
            _ => None,
        }
    }

    /// Get the combined demand modifier from all invisible factors
    pub fn calculate_demand_modifier(&self) -> f32 {
        let mut modifier = 1.0;

        // Christmas effect (huge!)
        // Peaks in the weeks before Christmas
        if self.days_to_christmas <= 30 && self.days_to_christmas > 0 {
            let christmas_boost = 1.0 + (2.0 * (30 - self.days_to_christmas) as f32 / 30.0);
            modifier *= christmas_boost;
        }

        // Holiday effects
        if let Some(holiday) = &self.current_holiday {
            modifier *= match holiday {
                Holiday::BlackFriday => 3.0,
                Holiday::Christmas => 0.5,      // People are WITH family, not shopping
                Holiday::NewYearsEve => 0.3,
                Holiday::NewYears => 0.4,
                Holiday::Thanksgiving => 0.6,
                Holiday::ValentinesDay => 1.3,
                Holiday::IndependenceDay => 0.8,
                Holiday::Halloween => 1.2,
                Holiday::LaborDay | Holiday::MemorialDay | Holiday::PresidentsDay => 1.4, // Sales!
                Holiday::Easter => 0.7,
            };
        }

        // Weekend effect
        if self.is_weekend {
            modifier *= 1.3; // More shopping on weekends
        }

        // Temperature effects
        // Extreme temps keep people home
        if self.temperature < 20.0 || self.temperature > 95.0 {
            modifier *= 0.7;
        } else if self.temperature > 70.0 && self.temperature < 80.0 {
            modifier *= 1.1; // Nice weather = good mood = more spending
        }

        // Consumer confidence
        modifier *= self.consumer_confidence;

        // Unemployment drags spending
        modifier *= 1.0 - (self.unemployment_rate * 0.5);

        // Market sentiment
        modifier *= 1.0 + (self.market_sentiment * 0.2);

        // Trend factor
        modifier *= self.trend_factor;

        // Competitor pressure reduces your slice
        modifier *= 1.0 - (self.competitor_pressure * 0.3);

        // Market saturation makes it harder
        modifier *= 1.0 - (self.market_saturation * 0.2);

        // Global population scales total addressable market
        // Normalize to 2012 baseline
        let population_factor = (self.global_population / 7_000_000_000.0) as f32;
        modifier *= population_factor;

        modifier.max(0.1) // Never completely zero
    }

    /// Get a "chaos factor" - random daily variance in the economy
    pub fn daily_chaos(&self) -> f32 {
        // Pseudo-random based on date (deterministic but feels random)
        let seed = self.date.year * 10000 + self.date.month as i32 * 100 + self.date.day as i32;
        let chaos = ((seed as f32 * 12.9898).sin() * 43758.5453).fract();
        0.8 + (chaos * 0.4) // Range: 0.8 to 1.2
    }
}

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldState>()
            .add_systems(Update, advance_world_simulation);
    }
}

/// Advances the world simulation each frame
fn advance_world_simulation(
    time: Res<Time>,
    mut world: ResMut<WorldState>,
) {
    // Accumulate time
    world.day_accumulator += time.delta_secs();

    // Advance days based on time scale
    while world.day_accumulator >= world.time_scale {
        world.day_accumulator -= world.time_scale;
        advance_one_day(&mut world);
    }
}

fn advance_one_day(world: &mut WorldState) {
    // Advance the calendar
    world.date.advance();

    // Update day of week
    world.day_of_week = world.date.day_of_week();
    world.is_weekend = world.day_of_week == 0 || world.day_of_week == 6;

    // Update days to Christmas
    world.days_to_christmas = world.date.days_until_christmas();

    // Update holiday
    world.current_holiday = world.check_holiday();

    // Update seasonal temperature
    world.seasonal_base_temp = world.calculate_seasonal_temp();

    // Add daily temperature variance (-10 to +10 degrees)
    let temp_seed = world.date.year * 10000 + world.date.month as i32 * 100 + world.date.day as i32;
    world.temp_variance = ((temp_seed as f32 * 78.233).sin() * 43758.5453).fract() * 20.0 - 10.0;
    world.temperature = world.seasonal_base_temp + world.temp_variance;

    // Grow population
    world.global_population *= world.population_growth_rate;

    // Apply historical events BEFORE random drift
    apply_historical_events(world);

    // Drift economic indicators slightly (random walk)
    let econ_seed = temp_seed + 1;
    let drift = ((econ_seed as f32 * 45.164).sin() * 43758.5453).fract() * 0.02 - 0.01;

    world.consumer_confidence = (world.consumer_confidence + drift).clamp(0.5, 1.5);
    world.market_sentiment = (world.market_sentiment + drift * 2.0).clamp(-0.5, 0.5);

    // Trend factor drifts more dramatically
    let trend_seed = temp_seed + 2;
    let trend_drift = ((trend_seed as f32 * 93.989).sin() * 43758.5453).fract() * 0.1 - 0.05;
    world.trend_factor = (world.trend_factor + trend_drift).clamp(0.5, 2.0);

    // Competitor pressure ebbs and flows
    let comp_seed = temp_seed + 3;
    let comp_drift = ((comp_seed as f32 * 12.345).sin() * 43758.5453).fract() * 0.05 - 0.025;
    world.competitor_pressure = (world.competitor_pressure + comp_drift).clamp(0.2, 0.8);
}

/// Historical events from 2012-2026 that affect the economy
/// These are invisible to the player but shape the world
fn apply_historical_events(world: &mut WorldState) {
    let y = world.date.year;
    let m = world.date.month;
    let d = world.date.day;

    match (y, m, d) {
        // === 2012 ===
        // Obama re-elected - November 6, 2012 (must come before Sandy range)
        (2012, 11, 6) => {
            world.market_sentiment += 0.05;
        }
        // Hurricane Sandy - late October 2012
        (2012, 10, 29..=31) | (2012, 11, 1..=7) => {
            world.consumer_confidence *= 0.85;
            world.market_sentiment -= 0.1;
        }

        // === 2013 ===
        // Boston Marathon bombing - April 15, 2013
        (2013, 4, 15..=22) => {
            world.consumer_confidence *= 0.92;
        }
        // Government shutdown - October 2013
        (2013, 10, 1..=16) => {
            world.consumer_confidence *= 0.9;
            world.market_sentiment -= 0.15;
        }

        // === 2014 ===
        // Russia annexes Crimea - March 2014
        (2014, 3, 18..=31) => {
            world.market_sentiment -= 0.1;
        }
        // Ferguson protests - August 2014
        (2014, 8, 9..=31) => {
            world.consumer_confidence *= 0.95;
        }

        // === 2015 ===
        // Same-sex marriage legalized - June 26, 2015
        (2015, 6, 26..=30) => {
            world.trend_factor *= 1.05;
        }
        // Paris attacks - November 13, 2015
        (2015, 11, 13..=20) => {
            world.consumer_confidence *= 0.9;
            world.market_sentiment -= 0.1;
        }

        // === 2016 ===
        // Brexit vote - June 23, 2016
        (2016, 6, 23..=30) => {
            world.market_sentiment -= 0.2;
            world.consumer_confidence *= 0.92;
        }
        // Trump elected - November 8, 2016
        (2016, 11, 8..=15) => {
            world.market_sentiment += 0.1; // Markets initially rallied
            world.trend_factor *= 1.1;
        }

        // === 2017 ===
        // Trump inaugurated - January 20, 2017
        (2017, 1, 20) => {
            world.trend_factor *= 1.05;
        }
        // Hurricane Harvey - late August 2017
        (2017, 8, 25..=31) | (2017, 9, 1..=5) => {
            world.consumer_confidence *= 0.88;
        }
        // Hurricane Maria - September 2017
        (2017, 9, 20..=30) => {
            world.consumer_confidence *= 0.9;
        }
        // Bitcoin mania peaks - December 2017
        (2017, 12, 1..=20) => {
            world.trend_factor *= 1.15;
            world.market_sentiment += 0.1;
        }

        // === 2018 ===
        // Bitcoin crash - January-February 2018
        (2018, 1, 15..=31) | (2018, 2, 1..=10) => {
            world.market_sentiment -= 0.15;
        }
        // Trade war begins - March 2018
        (2018, 3, 22..=31) | (2018, 4, 1..=15) => {
            world.market_sentiment -= 0.1;
            world.consumer_confidence *= 0.95;
        }
        // Midterms - Democrats take House - November 6, 2018
        (2018, 11, 6..=10) => {
            world.market_sentiment -= 0.05;
        }

        // === 2019 ===
        // Government shutdown ends - January 2019 (longest ever)
        (2019, 1, 1..=25) => {
            world.consumer_confidence *= 0.92;
        }
        // Trump impeachment vote - December 18, 2019
        (2019, 12, 18..=31) => {
            world.market_sentiment -= 0.05;
        }

        // === 2020 - THE BIG ONE ===
        // COVID becomes serious - March 2020
        (2020, 3, 11..=31) => {
            world.consumer_confidence *= 0.6;
            world.market_sentiment -= 0.4;
            world.unemployment_rate = 0.15; // Massive spike
        }
        // George Floyd protests - May 25 onward, 2020 (must come before general May)
        (2020, 5, 25..=31) | (2020, 6, 1..=15) => {
            world.consumer_confidence = 0.5;
            world.consumer_confidence *= 0.85;
        }
        // COVID lockdowns continue - April-May 2020
        (2020, 4, _) | (2020, 5, 1..=24) => {
            world.consumer_confidence = 0.5;
            world.unemployment_rate = 0.14;
        }
        // Slow recovery - Summer 2020
        (2020, 6, 16..=30) | (2020, 7, _) | (2020, 8, _) => {
            world.consumer_confidence = 0.7;
            world.unemployment_rate = 0.11;
        }
        // Biden elected - November 3, 2020
        (2020, 11, 3..=10) => {
            world.market_sentiment += 0.15;
        }
        // Vaccine approved - December 2020
        (2020, 12, 11..=31) => {
            world.consumer_confidence *= 1.1;
            world.market_sentiment += 0.2;
        }

        // === 2021 ===
        // January 6 Capitol riot - 2021
        (2021, 1, 6..=10) => {
            world.consumer_confidence *= 0.9;
            world.market_sentiment -= 0.1;
        }
        // Biden inaugurated - January 20, 2021
        (2021, 1, 20) => {
            world.market_sentiment += 0.05;
        }
        // Stimulus checks - March 2021
        (2021, 3, 12..=31) => {
            world.consumer_confidence *= 1.15;
            world.trend_factor *= 1.1;
        }
        // Meme stock mania - January 2021
        (2021, 1, 25..=31) => {
            world.trend_factor *= 1.2;
            world.market_sentiment += 0.15;
        }
        // Recovery continues through 2021
        (2021, 4, _) | (2021, 5, _) | (2021, 6, _) => {
            world.unemployment_rate = 0.06;
            world.consumer_confidence = 1.1;
        }
        // Inflation worries begin - late 2021
        (2021, 10, _) | (2021, 11, _) | (2021, 12, _) => {
            world.inflation_rate = 0.07;
            world.consumer_confidence *= 0.95;
        }

        // === 2022 ===
        // Russia invades Ukraine - February 24, 2022
        (2022, 2, 24..=28) | (2022, 3, 1..=15) => {
            world.consumer_confidence *= 0.85;
            world.market_sentiment -= 0.2;
            world.inflation_rate = 0.085;
        }
        // Inflation peaks - June 2022 (9.1%)
        (2022, 6, _) | (2022, 7, _) => {
            world.inflation_rate = 0.091;
            world.consumer_confidence *= 0.9;
        }
        // Queen Elizabeth II dies - September 8, 2022
        (2022, 9, 8..=19) => {
            world.trend_factor *= 0.95; // Somber mood
        }
        // Midterms - November 2022
        (2022, 11, 8..=12) => {
            world.market_sentiment += 0.05;
        }

        // === 2023 ===
        // Banking crisis (SVB collapse) - March 2023
        (2023, 3, 10..=20) => {
            world.market_sentiment -= 0.25;
            world.consumer_confidence *= 0.85;
        }
        // AI boom (ChatGPT mania) - throughout 2023
        (2023, 1, _) | (2023, 2, _) | (2023, 3, _) | (2023, 4, _) | (2023, 5, _) => {
            world.trend_factor *= 1.05;
        }
        // Inflation cooling - late 2023
        (2023, 10, _) | (2023, 11, _) | (2023, 12, _) => {
            world.inflation_rate = 0.035;
            world.consumer_confidence *= 1.05;
        }

        // === 2024 ===
        // Election year uncertainty - most of 2024
        (2024, 6, _) | (2024, 7, _) | (2024, 8, _) | (2024, 9, _) | (2024, 10, _) => {
            world.market_sentiment -= 0.05;
        }
        // Trump wins election - November 5, 2024
        (2024, 11, 5..=12) => {
            world.market_sentiment += 0.15;
            world.trend_factor *= 1.1;
        }

        // === 2025 ===
        // Trump inaugurated again - January 20, 2025
        (2025, 1, 20) => {
            world.trend_factor *= 1.05;
        }
        // Tariff announcements begin - early 2025
        (2025, 2, _) | (2025, 3, _) => {
            world.market_sentiment -= 0.1;
            world.consumer_confidence *= 0.95;
        }

        // === 2026 ===
        // Current day: February 19, 2026
        // The game catches up to "now" - things get weird
        (2026, 2, 19..) | (2026, 3.., _) => {
            // Beyond the known timeline - maximum chaos
            world.trend_factor *= 1.0 + (world.daily_chaos() - 1.0) * 2.0;
        }

        // Default - no special event
        _ => {}
    }

    // Clamp values after historical adjustments
    world.consumer_confidence = world.consumer_confidence.clamp(0.3, 1.8);
    world.market_sentiment = world.market_sentiment.clamp(-0.8, 0.8);
    world.unemployment_rate = world.unemployment_rate.clamp(0.03, 0.25);
    world.inflation_rate = world.inflation_rate.clamp(0.01, 0.15);
}
