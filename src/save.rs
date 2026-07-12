use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SAVE_PATH: &str = "save.json";

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct SaveData {
    /// Who the player chose to be. `default` keeps old scrolls loading — they
    /// simply travel as the young traveller, unnamed.
    #[serde(default)]
    pub player_char: usize,
    #[serde(default)]
    pub player_name: String,
    pub completed: Vec<u8>,
    pub accepted: Vec<u8>,
    pub hints: BTreeMap<u8, usize>,
    /// Wild runes inscribed in the grimoire. `default` keeps old save
    /// scrolls readable — they simply start with empty pages.
    #[serde(default)]
    pub grimoire: Vec<u8>,
    #[serde(default)]
    pub fish: u32,
    /// The purse: the first stored counter (coins can't be derived). Old
    /// scrolls load with empty pockets, which is exactly what they had.
    #[serde(default)]
    pub coins: u32,
    /// The basket: goods gathered, grown, or cooked, as (good id, count)
    /// pairs (ids from `content::market::Good::id`). Unknown ids are simply
    /// dropped on load, so the format can grow without breaking old scrolls.
    #[serde(default)]
    pub pantry: Vec<(String, u32)>,
    /// World-state flags (side quests, runestones found, opened chests).
    /// `default` again: old scrolls simply have no flags set yet.
    #[serde(default)]
    pub flags: Vec<String>,
    pub zone: usize,
    pub pos: (i32, i32),
    pub play_ticks: u64,
    /// Position in the day/night cycle. `default` keeps old scrolls loading —
    /// they simply wake at dawn (tick 0).
    #[serde(default)]
    pub day_ticks: u32,
    /// Typewriter text speed (0 slow, 1 normal, 2 fast). Old scrolls default
    /// to the middle setting.
    #[serde(default = "default_text_speed")]
    pub text_speed: usize,
    /// The sound option (0 off, 1 quiet, 2 full). Old scrolls default to
    /// full — the world sounded like this all along; they just predate the
    /// dial.
    #[serde(default = "default_sound_level")]
    pub sound_level: usize,
}

fn default_text_speed() -> usize {
    1
}

fn default_sound_level() -> usize {
    2
}

pub fn exists() -> bool {
    Path::new(SAVE_PATH).exists()
}

pub fn save(data: &SaveData) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(SAVE_PATH, json)?;
    Ok(())
}

pub fn load() -> Option<SaveData> {
    let text = std::fs::read_to_string(SAVE_PATH).ok()?;
    serde_json::from_str(&text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_data_round_trips_through_json() {
        let mut hints = BTreeMap::new();
        hints.insert(3u8, 2usize);
        let data = SaveData {
            player_char: 2,
            player_name: "Bramble".to_string(),
            completed: vec![1, 2, 3],
            accepted: vec![1, 2, 3, 4],
            hints,
            grimoire: vec![1, 11],
            fish: 3,
            coins: 17,
            pantry: vec![("mushroom".to_string(), 2), ("turnip-seeds".to_string(), 1)],
            flags: vec!["sorrel.asked".to_string(), "runestone.4".to_string()],
            zone: 1,
            pos: (42, 17),
            play_ticks: 9001,
            day_ticks: 12345,
            text_speed: 2,
            sound_level: 1,
        };
        let json = serde_json::to_string(&data).unwrap();
        let back: SaveData = serde_json::from_str(&json).unwrap();
        assert_eq!(back, data);
    }

    #[test]
    fn old_save_scrolls_without_grimoire_still_load() {
        let old = r#"{"completed":[1],"accepted":[1,2],"hints":{},"zone":0,"pos":[6,38],"play_ticks":100}"#;
        let back: SaveData = serde_json::from_str(old).unwrap();
        assert!(back.grimoire.is_empty());
        assert_eq!(back.fish, 0);
        assert_eq!(back.coins, 0, "old scrolls wake with empty pockets");
        assert!(back.pantry.is_empty(), "and a bare basket");
        assert!(back.flags.is_empty());
        assert_eq!(back.day_ticks, 0);
        assert_eq!(back.player_char, 0);
        assert!(back.player_name.is_empty());
        assert_eq!(back.text_speed, 1, "old scrolls default to normal speed");
        assert_eq!(back.sound_level, 2, "old scrolls default to full sound");
    }
}
