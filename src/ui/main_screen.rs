//! Main game screen UI

use bevy::prelude::*;
use crate::game_state::GameState;
use crate::business::{UpgradeState, UpgradeType};
use crate::clicker::ClickEvent;
use super::{UiRoot, NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, DISABLED_BUTTON};

/// Marker for main game screen elements
#[derive(Component)]
pub struct MainScreen;

/// Marker for the "Make Thing" button
#[derive(Component)]
pub struct MakeThingButton;

/// Marker for stats display text
#[derive(Component)]
pub struct StatsText;

/// Marker for money display
#[derive(Component)]
pub struct MoneyText;

/// Marker for things display
#[derive(Component)]
pub struct ThingsText;

/// Marker for reputation display
#[derive(Component)]
pub struct ReputationText;

/// Marker for production rate display
#[derive(Component)]
pub struct ProductionText;

/// Marker for upgrade buttons
#[derive(Component)]
pub struct UpgradeButton(pub UpgradeType);

/// Marker for upgrade cost text
#[derive(Component)]
pub struct UpgradeCostText(pub UpgradeType);

pub fn setup_main_screen(mut commands: Commands, game_state: Res<GameState>) {
    let thing_type = game_state.thing_type.unwrap_or_default();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
            UiRoot,
            MainScreen,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("THING SIMULATOR 2012"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                parent.spawn((
                    Text::new(format!("Your Thing: {}", thing_type.name())),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(thing_type.color()),
                ));
            });

            // Main content area
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    // Left panel - Terry area (will be implemented in terry_box.rs)
                    spawn_terry_panel(parent);

                    // Center panel - Stats and clicker
                    spawn_center_panel(parent, &game_state);

                    // Right panel - Upgrades
                    spawn_upgrades_panel(parent);
                });
        });
}

fn spawn_terry_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::right(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        ))
        .with_children(|parent| {
            // Terry placeholder image area
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(200.0),
                    margin: UiRect::bottom(Val::Px(15.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.8, 0.5, 0.2)),
                BackgroundColor(Color::srgb(0.15, 0.12, 0.1)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("[TERRY]\n🌭\nMBA, Hot Dog"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.6, 0.3)),
                    TextLayout {
                        justify: Justify::Center,
                        ..default()
                    },
                ));
            });

            // Terry dialogue label
            parent.spawn((
                Text::new("Terry says:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Terry dialogue box
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(120.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.4)),
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Loading Terry's wisdom..."),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.85, 0.7)),
                        super::terry_box::TerryDialogueText,
                    ));
                });

            // Terry's reason for being here
            parent.spawn((
                Text::new("(Your mother asked him to help)"),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_center_panel(parent: &mut ChildSpawnerCommands, game_state: &GameState) {
    parent
        .spawn((
            Node {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.06, 0.1)),
        ))
        .with_children(|parent| {
            // Stats display
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Things count
                    parent.spawn((
                        Text::new(format!("Things: {}", game_state.things_produced)),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ThingsText,
                    ));

                    // Money
                    parent.spawn((
                        Text::new(format!("${:.2}", game_state.money)),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.9, 0.5)),
                        MoneyText,
                        Node {
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // Production rate
                    parent.spawn((
                        Text::new(format!("{:.1} Things/sec", game_state.things_per_second)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.9)),
                        ProductionText,
                        Node {
                            margin: UiRect::top(Val::Px(5.0)),
                            ..default()
                        },
                    ));

                    // Reputation
                    parent.spawn((
                        Text::new(format!("Reputation: {}", reputation_stars(game_state.reputation))),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.8, 0.3)),
                        ReputationText,
                        Node {
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                    ));
                });

            // Make Thing button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.6, 0.9)),
                    BackgroundColor(NORMAL_BUTTON),
                    MakeThingButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("MAKE"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                        parent.spawn((
                            Text::new("THING"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                        parent.spawn((
                            Text::new(format!("+{} Thing", game_state.click_power)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            Node {
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    });
                });

            // Marketing level indicator
            parent.spawn((
                Text::new(format!("Marketing Level: {}", game_state.marketing_level)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.8, 0.9)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_upgrades_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Px(280.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::left(Val::Px(2.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        ))
        .with_children(|parent| {
            // Production upgrades header
            parent.spawn((
                Text::new("PRODUCTION"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Production upgrade buttons
            for upgrade in [UpgradeType::BetterTools, UpgradeType::HireWorker, UpgradeType::Automation] {
                spawn_upgrade_button(parent, upgrade);
            }

            // Marketing upgrades header
            parent.spawn((
                Text::new("MARKETING"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(10.0)),
                    ..default()
                },
            ));

            // Marketing upgrade buttons
            for upgrade in [UpgradeType::SocialMedia, UpgradeType::Billboard, UpgradeType::InfluencerDeal] {
                spawn_upgrade_button(parent, upgrade);
            }
        });
}

fn spawn_upgrade_button(parent: &mut ChildSpawnerCommands, upgrade: UpgradeType) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(70.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(if upgrade.is_production() {
                Color::srgb(0.3, 0.5, 0.8)
            } else {
                Color::srgb(0.8, 0.5, 0.3)
            }),
            BackgroundColor(NORMAL_BUTTON),
            UpgradeButton(upgrade),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(upgrade.name()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new(upgrade.description()),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            parent.spawn((
                Text::new(format!("${:.0}", upgrade.base_cost())),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.9, 0.5)),
                UpgradeCostText(upgrade),
            ));
        });
}

fn reputation_stars(reputation: f32) -> String {
    let full_stars = reputation.floor() as usize;
    let has_half = reputation.fract() >= 0.5;
    let empty_stars = 5 - full_stars - if has_half { 1 } else { 0 };

    let mut stars = "★".repeat(full_stars);
    if has_half {
        stars.push('☆');
    }
    stars.push_str(&"☆".repeat(empty_stars));
    stars
}

pub fn update_stats_display(
    game_state: Res<GameState>,
    mut things_query: Query<&mut Text, (With<ThingsText>, Without<MoneyText>, Without<ReputationText>, Without<ProductionText>)>,
    mut money_query: Query<&mut Text, (With<MoneyText>, Without<ThingsText>, Without<ReputationText>, Without<ProductionText>)>,
    mut rep_query: Query<&mut Text, (With<ReputationText>, Without<ThingsText>, Without<MoneyText>, Without<ProductionText>)>,
    mut prod_query: Query<&mut Text, (With<ProductionText>, Without<ThingsText>, Without<MoneyText>, Without<ReputationText>)>,
) {
    for mut text in &mut things_query {
        **text = format!("Things: {}", game_state.things_produced);
    }

    for mut text in &mut money_query {
        **text = format!("${:.2}", game_state.money);
    }

    for mut text in &mut rep_query {
        **text = format!("Reputation: {}", reputation_stars(game_state.reputation));
    }

    for mut text in &mut prod_query {
        let multiplier = game_state.thing_type.map(|t| t.production_multiplier()).unwrap_or(1.0);
        let actual_rate = game_state.things_per_second * multiplier;
        **text = format!("{:.1} Things/sec", actual_rate);
    }
}

pub fn handle_make_thing_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MakeThingButton>),
    >,
    _click_events: MessageWriter<ClickEvent>,
    mut game_state: ResMut<GameState>,
    mut thing_events: MessageWriter<crate::game_state::ThingProducedEvent>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON.into();
                // Directly handle click here since we need mutable access
                if let Some(thing_type) = game_state.thing_type {
                    let multiplier = thing_type.production_multiplier();
                    let things = (game_state.click_power as f64 * multiplier).ceil() as u64;
                    game_state.things_produced += things;
                    thing_events.write(crate::game_state::ThingProducedEvent {
                        amount: things,
                        from_click: true,
                    });
                }
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn handle_upgrade_buttons(
    mut interaction_query: Query<
        (&Interaction, &UpgradeButton, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    mut game_state: ResMut<GameState>,
    mut upgrade_state: ResMut<UpgradeState>,
    mut cost_text_query: Query<(&mut Text, &UpgradeCostText)>,
) {
    for (interaction, upgrade_button, mut bg_color, _border_color) in &mut interaction_query {
        let upgrade = upgrade_button.0;
        let cost = upgrade_state.cost(upgrade);
        let can_afford = game_state.money >= cost;

        match *interaction {
            Interaction::Pressed => {
                if can_afford {
                    *bg_color = PRESSED_BUTTON.into();
                    upgrade_state.purchase(upgrade, &mut game_state);

                    // Update cost display
                    let new_cost = upgrade_state.cost(upgrade);
                    for (mut text, cost_text) in &mut cost_text_query {
                        if cost_text.0 == upgrade {
                            **text = format!("${:.0}", new_cost);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = if can_afford { HOVERED_BUTTON } else { DISABLED_BUTTON }.into();
            }
            Interaction::None => {
                *bg_color = if can_afford { NORMAL_BUTTON } else { DISABLED_BUTTON }.into();
            }
        }
    }
}

pub fn cleanup_main_screen(
    mut commands: Commands,
    query: Query<Entity, With<MainScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
