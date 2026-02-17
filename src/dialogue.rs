//! Dialogue system for loading and managing Terry's lines from JSON files

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DialogueDatabase>()
            .add_systems(Startup, load_dialogues);
    }
}

/// A single dialogue line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    pub id: String,
    pub trigger: String,
    pub text: String,
    #[serde(default)]
    pub mood: String,
}

/// Collection of dialogue lines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueFile {
    pub lines: Vec<DialogueLine>,
}

/// Resource containing all loaded dialogues
#[derive(Resource, Default)]
pub struct DialogueDatabase {
    /// All lines indexed by trigger type
    pub by_trigger: HashMap<String, Vec<DialogueLine>>,
    /// All lines indexed by ID
    pub by_id: HashMap<String, DialogueLine>,
}

impl DialogueDatabase {
    /// Get a random line for a trigger
    pub fn get_for_trigger(&self, trigger: &str) -> Option<&DialogueLine> {
        self.by_trigger.get(trigger).and_then(|lines| {
            if lines.is_empty() {
                None
            } else {
                // Simple random selection using current time
                let index = (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as usize)
                    % lines.len();
                Some(&lines[index])
            }
        })
    }

    /// Add a dialogue line
    pub fn add_line(&mut self, line: DialogueLine) {
        let trigger = line.trigger.clone();
        let id = line.id.clone();

        self.by_trigger
            .entry(trigger)
            .or_insert_with(Vec::new)
            .push(line.clone());
        self.by_id.insert(id, line);
    }
}

/// Load all dialogue files
fn load_dialogues(mut dialogue_db: ResMut<DialogueDatabase>) {
    let dialogue_files = [
        "assets/dialogues/terry_generic.json",
        "assets/dialogues/terry_cheap.json",
        "assets/dialogues/terry_good.json",
        "assets/dialogues/terry_expensive.json",
        "assets/dialogues/terry_bad.json",
    ];

    for path_str in dialogue_files {
        let path = Path::new(path_str);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(contents) => match serde_json::from_str::<DialogueFile>(&contents) {
                    Ok(file) => {
                        for line in file.lines {
                            dialogue_db.add_line(line);
                        }
                        info!("Loaded dialogue file: {}", path_str);
                    }
                    Err(e) => {
                        warn!("Failed to parse dialogue file {}: {}", path_str, e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read dialogue file {}: {}", path_str, e);
                }
            }
        } else {
            // File doesn't exist, that's okay - we'll use fallback lines
            info!("Dialogue file not found (will use fallbacks): {}", path_str);
        }
    }

    // If no files loaded, add some default fallback lines
    if dialogue_db.by_id.is_empty() {
        add_fallback_lines(&mut dialogue_db);
    }
}

/// Add fallback dialogue lines if JSON files aren't available
fn add_fallback_lines(db: &mut DialogueDatabase) {
    let fallbacks = vec![
        // Generic lines
        DialogueLine {
            id: "generic_greeting".into(),
            trigger: "game_start".into(),
            text: "Welcome to Thing Simulator 2012! I'm Terry. Yes, I'm a hot dog. Yes, I have an MBA. Your mother asked me to help you with this.".into(),
            mood: "neutral".into(),
        },
        DialogueLine {
            id: "generic_click".into(),
            trigger: "click".into(),
            text: "That's the spirit! Every Thing counts. Your mother would be proud.".into(),
            mood: "happy".into(),
        },
        DialogueLine {
            id: "generic_idle".into(),
            trigger: "idle".into(),
            text: "You know what they say in business school? 'Time is money.' I learned that before they realized I was a hot dog.".into(),
            mood: "thoughtful".into(),
        },
        // Milestone lines
        DialogueLine {
            id: "milestone_10".into(),
            trigger: "things_10".into(),
            text: "10 Things! That's what I call a proof of concept. Your mother will be thrilled.".into(),
            mood: "happy".into(),
        },
        DialogueLine {
            id: "milestone_100".into(),
            trigger: "things_100".into(),
            text: "100 Things! We're really cooking now. Pun absolutely intended.".into(),
            mood: "excited".into(),
        },
        DialogueLine {
            id: "milestone_1000".into(),
            trigger: "things_1000".into(),
            text: "1,000 Things! This is what we call 'scaling' in the business. I'm a scaling hot dog!".into(),
            mood: "excited".into(),
        },
        // Cheap Thing lines
        DialogueLine {
            id: "cheap_select".into(),
            trigger: "select_cheap".into(),
            text: "Cheap Things? Bold strategy. Volume is key. Your mother would approve - she loves a bargain.".into(),
            mood: "skeptical".into(),
        },
        DialogueLine {
            id: "cheap_advice".into(),
            trigger: "cheap_playing".into(),
            text: "Remember: when selling cheap, it's all about turnover. Like a rotisserie. Like... never mind.".into(),
            mood: "helpful".into(),
        },
        // Good Thing lines
        DialogueLine {
            id: "good_select".into(),
            trigger: "select_good".into(),
            text: "A Good Thing! Quality over quantity. Very noble. Very slow. But noble.".into(),
            mood: "approving".into(),
        },
        DialogueLine {
            id: "good_advice".into(),
            trigger: "good_playing".into(),
            text: "Quality builds reputation. Reputation builds trust. Trust builds... the ability to charge more.".into(),
            mood: "wise".into(),
        },
        // Expensive Thing lines
        DialogueLine {
            id: "expensive_select".into(),
            trigger: "select_expensive".into(),
            text: "Expensive Things! Luxury positioning. I learned about this at Wharton. Well, I read about Wharton. In a dumpster behind Wharton.".into(),
            mood: "impressed".into(),
        },
        DialogueLine {
            id: "expensive_advice".into(),
            trigger: "expensive_playing".into(),
            text: "In the luxury market, scarcity creates value. Like hot dogs with business degrees.".into(),
            mood: "sophisticated".into(),
        },
        // Bad Thing lines
        DialogueLine {
            id: "bad_select".into(),
            trigger: "select_bad".into(),
            text: "Bad Things? Oh. Oh no. This is... this is exactly what my ethics professor warned me about. He was a bratwurst.".into(),
            mood: "concerned".into(),
        },
        DialogueLine {
            id: "bad_advice".into(),
            trigger: "bad_playing".into(),
            text: "I'm not saying this is wrong, but I'm definitely taking notes for my parole hearing.".into(),
            mood: "nervous".into(),
        },
        DialogueLine {
            id: "bad_reputation_low".into(),
            trigger: "bad_low_rep".into(),
            text: "Our reputation is tanking. This is fine. Everything is fine. *sweats mustard*".into(),
            mood: "panicked".into(),
        },
    ];

    for line in fallbacks {
        db.add_line(line);
    }

    info!("Loaded {} fallback dialogue lines", db.by_id.len());
}
