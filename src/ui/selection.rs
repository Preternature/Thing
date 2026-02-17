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
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
            UiRoot,
            SelectionScreen,
        ))
        .with_children(|parent| {
            // Terry's introduction
            parent.spawn((
                Text::new("\"Hi, I'm Terry. I have an MBA, and I'm a hot dog. But enough about me: your mother told me that you want to start selling something. What is that... thing? Just... tell me what it is, in a word. One word.\""),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    max_width: Val::Px(700.0),
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
            ));

            // Question prompt
            parent.spawn((
                Text::new("What is your thing?"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Button container - four simple word choices
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
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
                width: Val::Px(140.0),
                height: Val::Px(60.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
            BackgroundColor(NORMAL_BUTTON),
            ThingTypeButton(thing_type),
        ))
        .with_children(|parent| {
            // Just the word - no description, no price
            parent.spawn((
                Text::new(thing_type.name()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
