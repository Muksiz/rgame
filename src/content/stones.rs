//! The eight hidden runestones: old standing stones tucked into quiet corners
//! of the world, each carved with a rune worth rubbing into your journal.
//! Seven stand where `world/zones.rs` planted them (`RUNESTONE_SPOTS`); the
//! eighth sleeps inside the locked chest in the storehouse cellar. Finding
//! one sets the flag `runestone.<id>` — nothing else is stored.

use std::collections::BTreeSet;

pub struct Runestone {
    pub id: u8,
    pub name: &'static str,
    /// Read aloud when the stone is found.
    pub legend: &'static str,
}

pub static RUNESTONES: [Runestone; 8] = [
    Runestone {
        id: 1,
        name: "the Henstone",
        legend: "A squat little stone in the chicken pen, polished glossy by generations of hens using it as a throne. The rune on it, appropriately, means 'home'.",
    },
    Runestone {
        id: 2,
        name: "the Wishing Stone",
        legend: "It leans companionably at the edge of Wren's clearing, covered in tiny scratched-in stars. The rune means 'someday'. Wren denies everything.",
    },
    Runestone {
        id: 3,
        name: "the Hearthstone of the Hollow",
        legend: "Deep in the woods where almost nobody walks, a stone with a rune meaning 'found'. Old Nettle dusts it on Sundays.",
    },
    Runestone {
        id: 4,
        name: "the Tidemark Stone",
        legend: "Half-sunk in Morrow's beach, its rune just above the sand: 'patience'. The river has been editing this stone for centuries and is nearly satisfied.",
    },
    Runestone {
        id: 5,
        name: "the Cragfast Stone",
        legend: "Alone among the northern crags, weathered by mist. Its rune means 'endure', which the stone demonstrates rather than explains.",
    },
    Runestone {
        id: 6,
        name: "the Echo Stone",
        legend: "It hums faintly in the dark of the cave. The rune means 'listen'. If you press an ear to it, it politely presses back.",
    },
    Runestone {
        id: 7,
        name: "the Index Stone",
        legend: "The Great Library catalogued it centuries ago and now cannot move it without re-shelving the world. Its rune means 'remember'.",
    },
    Runestone {
        id: 8,
        name: "the Keystone",
        legend: "Locked in a chest, under a storehouse, behind a lost key — a stone with a rune meaning 'kept'. Somebody wanted this one to be earned.",
    },
];

pub fn stone(id: u8) -> &'static Runestone {
    &RUNESTONES[(id - 1) as usize]
}

/// How many runestones the journal counts as found.
pub fn found(flags: &BTreeSet<String>) -> usize {
    RUNESTONES
        .iter()
        .filter(|s| flags.contains(&super::sides::runestone_flag(s.id)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stones_are_well_formed() {
        for (i, s) in RUNESTONES.iter().enumerate() {
            assert_eq!(s.id as usize, i + 1, "ids are 1-based and in order");
            assert!(!s.name.is_empty() && !s.legend.is_empty());
        }
    }

    #[test]
    fn found_counts_flags() {
        let mut flags = BTreeSet::new();
        assert_eq!(found(&flags), 0);
        flags.insert(super::super::sides::runestone_flag(3));
        flags.insert(super::super::sides::runestone_flag(8));
        assert_eq!(found(&flags), 2);
        flags.insert("not.a.stone".to_string());
        assert_eq!(found(&flags), 2);
    }
}
