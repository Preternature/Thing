//! Terry's dialogue box UI component

use bevy::prelude::*;
use crate::terry::TerryState;

/// Marker for Terry's dialogue text
#[derive(Component)]
pub struct TerryDialogueText;

/// Update Terry's dialogue display
pub fn update_terry_dialogue(
    terry_state: Res<TerryState>,
    mut query: Query<&mut Text, With<TerryDialogueText>>,
) {
    for mut text in &mut query {
        if let Some(ref line) = terry_state.current_line {
            **text = format!("\"{}\"", line.text);
        } else {
            **text = String::from("\"...\"");
        }
    }
}
