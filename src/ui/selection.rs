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

/// Marker for Terry's dialogue text (so we can update it)
#[derive(Component)]
pub struct TerryDialogueText;

/// Tracks how long the player has been staring at the selection screen
#[derive(Resource)]
pub struct SelectionTimer {
    pub elapsed: f32,
    pub stage: SelectionStage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectionStage {
    Initial,
    Impatient,   // 60 seconds
    Furious,     // 3600 seconds (1 hour)
    // TODO: Future feature - after certain game condition, player can restart
    // and choose "Hot Dogs", triggering Terry's existential crisis:
    // "Well, hot dogs is two words. And.... I was not aware of your...
    // your mother didn't.... Jesus f.... okay. It's come to this."
}

impl Default for SelectionTimer {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            stage: SelectionStage::Initial,
        }
    }
}

pub fn setup_selection_screen(mut commands: Commands) {
    commands.insert_resource(SelectionTimer::default());

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
            // Terry's introduction (will be updated by timer)
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
                TerryDialogueText,
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

/// Updates Terry's dialogue based on how long the player takes to choose
pub fn update_selection_timer(
    time: Res<Time>,
    mut timer: ResMut<SelectionTimer>,
    mut query: Query<&mut Text, With<TerryDialogueText>>,
) {
    timer.elapsed += time.delta_secs();

    let new_stage = if timer.elapsed >= 3600.0 {
        SelectionStage::Furious
    } else if timer.elapsed >= 60.0 {
        SelectionStage::Impatient
    } else {
        SelectionStage::Initial
    };

    if new_stage != timer.stage {
        timer.stage = new_stage;

        if let Ok(mut text) = query.single_mut() {
            text.0 = match new_stage {
                SelectionStage::Initial => {
                    "\"Hi, I'm Terry. I have an MBA, and I'm a hot dog. But enough about me: your mother told me that you want to start selling something. What is that... thing? Just... tell me what it is, in a word. One word.\"".to_string()
                }
                SelectionStage::Impatient => {
                    "\"Look, it's not fuckin--just... just what the fu--what is it? Word? Say a word?\"".to_string()
                }
                SelectionStage::Furious => {
                    "\"Okay, dipshit, your mom didn't tell me I'd be dealing with a fucking retard here. Okay. So it's NOTHING, huh? Great. Yeah, I can market that--but you know, I'd rather I weren't.\"".to_string()
                }
            };
        }
    }
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
    commands.remove_resource::<SelectionTimer>();
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
