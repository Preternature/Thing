//! Thing type selection screen

use bevy::prelude::*;
use crate::game_state::{AppState, GameState};
use crate::thing_type::ThingType;
use super::{UiRoot, NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON};

/// Marker for selection screen elements
#[derive(Component)]
pub struct SelectionScreen;

/// Marker for thing type buttons
#[derive(Component)]
pub struct ThingTypeButton(pub ThingType);

pub fn setup_selection_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
            UiRoot,
            SelectionScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("THING SIMULATOR 2012"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Tell Terry about your Thing..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Terry intro
            parent.spawn((
                Text::new("\"Hello! I'm Terry, your business advisor. Yes, I'm a hot dog.\nYes, I have an MBA. Now, what kind of Thing are you selling?\""),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    max_width: Val::Px(600.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));

            // Button container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    for thing_type in [ThingType::Cheap, ThingType::Good, ThingType::Expensive, ThingType::Bad] {
                        spawn_thing_button(parent, thing_type);
                    }
                });
        });
}

fn spawn_thing_button(parent: &mut ChildSpawnerCommands, thing_type: ThingType) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(180.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BorderColor::all(thing_type.color()),
            BackgroundColor(NORMAL_BUTTON),
            ThingTypeButton(thing_type),
        ))
        .with_children(|parent| {
            // Thing type name
            parent.spawn((
                Text::new(thing_type.name()),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(thing_type.color()),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Description
            parent.spawn((
                Text::new(thing_type.description()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));

            // Price info
            parent.spawn((
                Text::new(format!("${:.2}/Thing", thing_type.base_price())),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.9, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

pub fn handle_selection_buttons(
    mut interaction_query: Query<
        (&Interaction, &ThingTypeButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<GameState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, thing_button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON.into();
                game_state.thing_type = Some(thing_button.0);
                next_state.set(AppState::Playing);
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

pub fn cleanup_selection_screen(
    mut commands: Commands,
    query: Query<Entity, With<SelectionScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
