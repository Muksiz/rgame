//! Keepsakes: the things NPCs press into your hands when their quest is done.
//! Owning one is derived from the quests you've completed (no extra save
//! state), and some of them quietly open up parts of the world — the
//! Zelda-style half of the journey.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    /// Bram's storm-lantern (quest 6) — lights the way into dark places.
    StormLantern,
    /// Juniper's spare rod (quest 17) — lets you fish any reedy bank.
    FishingRod,
}

impl Item {
    pub fn name(self) -> &'static str {
        match self {
            Item::StormLantern => "Bram's storm-lantern",
            Item::FishingRod => "Juniper's spare rod",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Item::StormLantern => "Burns steady in any weather. Dark places stop being a problem.",
            Item::FishingRod => "A little bent, well loved. Stand by reedy water and press e.",
        }
    }
}

/// What comes up when you fish with Juniper's rod. Strictly catch-and-release.
pub static CATCHES: &[&str] = &[
    "a silver dace! It winks. You let it go.",
    "an old boot. Empty. You apologize to it and set it back.",
    "a moon-pale trout — it regards you calmly, then slips away.",
    "a very small crab that is definitely not Ferris. Probably a cousin.",
    "a tangle of river-weed shaped exactly like a semicolon.",
    "a stickleback with strong opinions. You release it mid-argument.",
    "nothing at all — but the rain on the water was worth the stop.",
];

/// The keepsake a quest's NPC hands over with their thanks, if any.
pub fn reward(quest_id: u8) -> Option<Item> {
    match quest_id {
        6 => Some(Item::StormLantern),
        17 => Some(Item::FishingRod),
        _ => None,
    }
}

/// Everything currently in the satchel, in quest order.
pub fn satchel(completed: &std::collections::BTreeSet<u8>) -> Vec<Item> {
    completed.iter().filter_map(|&id| reward(id)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewards_come_from_completed_quests() {
        let mut done = std::collections::BTreeSet::new();
        assert!(satchel(&done).is_empty());
        done.extend([1, 2, 3, 4, 5, 6]);
        assert_eq!(satchel(&done), vec![Item::StormLantern]);
        done.insert(17);
        assert_eq!(satchel(&done), vec![Item::StormLantern, Item::FishingRod]);
    }
}
