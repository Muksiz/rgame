use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SAVE_PATH: &str = "save.json";

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct SaveData {
    pub completed: Vec<u8>,
    pub accepted: Vec<u8>,
    pub hints: BTreeMap<u8, usize>,
    /// Wild runes inscribed in the grimoire. `default` keeps old save
    /// scrolls readable — they simply start with empty pages.
    #[serde(default)]
    pub grimoire: Vec<u8>,
    #[serde(default)]
    pub fish: u32,
    /// World-state flags (side quests, runestones found, opened chests).
    /// `default` again: old scrolls simply have no flags set yet.
    #[serde(default)]
    pub flags: Vec<String>,
    pub zone: usize,
    pub pos: (i32, i32),
    pub play_ticks: u64,
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
            completed: vec![1, 2, 3],
            accepted: vec![1, 2, 3, 4],
            hints,
            grimoire: vec![1, 11],
            fish: 3,
            flags: vec!["sorrel.asked".to_string(), "runestone.4".to_string()],
            zone: 1,
            pos: (42, 17),
            play_ticks: 9001,
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
        assert!(back.flags.is_empty());
    }
}
