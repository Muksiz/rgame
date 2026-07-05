//! Side quests: little kindnesses off the main road. Unlike the twelve rune
//! quests these never touch rustc — they live entirely in world state, as
//! flags on `App` (persisted in the save scroll). The dialogue logic for each
//! arc lives here so `app.rs` only has to ask "does this NPC have side
//! business right now?".

use std::collections::BTreeSet;

// ── the flags (namespaced, stable: they live in save.json forever) ──────────

/// Granny Sorrel has asked for a sprig of moon-mint.
pub const SORREL_ASKED: &str = "sorrel.asked";
/// The moon-mint sprig is in the satchel.
pub const SORREL_MINT: &str = "sorrel.mint";
/// The mint is delivered; the tea is perfect.
pub const SORREL_DONE: &str = "sorrel.done";
/// Old Nettle has been found in the deep woods (and handed over her key).
pub const NETTLE_MET: &str = "nettle.met";
/// The chest in the storehouse cellar stands open.
pub const CHEST_OPENED: &str = "cellar.chest";

/// The stray crab's meals — one flag per hour of the day it has eaten at
/// (morning, midday, evening, night, in `DayPhase` order). Three meals at
/// three different hours and it trusts you (`crab_tamed`): from then on the
/// little crab walks at your heels, forever.
pub const CRAB_FED: [&str; 4] = [
    "crab.fed.morning",
    "crab.fed.day",
    "crab.fed.evening",
    "crab.fed.night",
];

/// How many morsels the stray behind the storehouse has taken so far.
pub fn crab_meals(flags: &BTreeSet<String>) -> usize {
    CRAB_FED.iter().filter(|f| flags.contains(**f)).count()
}

/// Fed thrice, trust won: the crab is a companion now, not a stray. Derived
/// from the meal flags, never stored on its own — old saves stay valid.
pub fn crab_tamed(flags: &BTreeSet<String>) -> bool {
    crab_meals(flags) >= 3
}

/// Runestone flags are `runestone.1` .. `runestone.8`.
pub fn runestone_flag(id: u8) -> String {
    format!("runestone.{id}")
}

/// What a side-quest dialogue does when it closes: nothing, or set a flag.
pub struct SideTalk {
    pub pages: Vec<String>,
    pub set: Option<&'static str>,
}

/// Side-quest dialogue for an NPC, given the current world flags — or `None`
/// when they have no side business and their ordinary idle line should play.
pub fn talk(npc: &str, flags: &BTreeSet<String>) -> Option<SideTalk> {
    let has = |f: &str| flags.contains(f);
    match npc {
        "Granny Sorrel" => {
            if has(SORREL_DONE) {
                Some(SideTalk {
                    pages: vec![
                        "The kettle sings, the tea is silver-green, and it's all thanks to you, dear. Do come sit whenever the road gets long.".into(),
                    ],
                    set: None,
                })
            } else if has(SORREL_MINT) {
                Some(SideTalk {
                    pages: vec![
                        "Is that — oh, it IS. Moon-mint, fresh as a secret! Give it here, give it here.".into(),
                        "*The kettle, sensing an occasion, finally commits to boiling. The cottage fills with a smell like cool evenings.*".into(),
                        "You're a treasure, dear. Mind you — if you're ever poking about the old storehouse, my Wynn kept his preserves in the cellar under it. Locked it, then lost the key on some ramble in the deep woods. Men.".into(),
                    ],
                    set: Some(SORREL_DONE),
                })
            } else if has(SORREL_ASKED) {
                Some(SideTalk {
                    pages: vec![
                        "No luck with the moon-mint yet? It grows off the cave path in the Whispering Woods — silvery little leaves, you can't miss it. Well. You can. But try not to.".into(),
                    ],
                    set: None,
                })
            } else {
                Some(SideTalk {
                    pages: vec![
                        "Sit down, sit down. The kettle's just... well, it's thinking about boiling.".into(),
                        "You know what would get it going? Moon-mint. There's a patch in the Whispering Woods, just off the path to the Echo Cave. If your road ever takes you past it, an old lady would be very grateful for a sprig.".into(),
                    ],
                    set: Some(SORREL_ASKED),
                })
            }
        }
        "Old Nettle" => {
            if has(NETTLE_MET) {
                Some(SideTalk {
                    pages: vec![
                        "Still here. So are the trees. We're all very consistent, out this way."
                            .into(),
                    ],
                    set: None,
                })
            } else {
                Some(SideTalk {
                    pages: vec![
                        "Well now. Nobody walks this deep into the woods by accident, and nobody does it twice by choice. I'm Nettle. I whittle.".into(),
                        "The woods bring me things, you know. Burls, feathers, once a whole umbrella. And this — a small rusted key. Turned up under an oak root, years back.".into(),
                        "It's stamped EMBERWICK STOREHOUSE, so it's no use to me; I keep my valuables in plain sight, where nobody believes they're valuable. Take it. Keys ought to meet their locks eventually.".into(),
                        "*Old Nettle presses a small rusted key into your hand and goes back to her whittling.*".into(),
                    ],
                    set: Some(NETTLE_MET),
                })
            }
        }
        _ => None,
    }
}

/// The stray crab taking a morsel at the hour `fed_flag`. The caller has
/// already checked this hour's morsel hasn't been taken; the flag lands when
/// the dialogue closes, like every side arc. The third meal is the one that
/// matters: trust, a satchel-side family reunion, and a friend for the road.
pub fn crab_talk(flags: &BTreeSet<String>, fed_flag: &'static str) -> SideTalk {
    match crab_meals(flags) {
        0 => SideTalk {
            pages: vec![
                "Curled in the gap behind the storehouse: a very small crab, orange as a coal. It is doing its best to look like a pebble. The pebbles are not fooled.".into(),
                "You crouch and hold out a corner of your travel loaf. A long, careful pause. Then — snip — the morsel is gone, and the crab is a pebble again.".into(),
                "*From somewhere in your satchel comes the faintest approving click.*".into(),
            ],
            set: Some(fed_flag),
        },
        1 => SideTalk {
            pages: vec![
                "The small crab is where you left it — but this time it sidles out to meet you, claws raised like a tiny fanfare.".into(),
                "It takes the morsel straight from your fingers, watching you the whole while with eyes like polished seeds. Progress, measured in crumbs.".into(),
            ],
            set: Some(fed_flag),
        },
        _ => SideTalk {
            pages: vec![
                "The small crab is not behind the storehouse. It is beside your boot. It has, apparently, been walking with you for a while already, waiting to be noticed.".into(),
                "It accepts the third morsel like a signature on a treaty. Then it climbs onto your boot, settles sideways, and faces the road.".into(),
                "Ferris leans out of your satchel for a long, careful look, and gives one solemn click. Family, apparently — a very small, very distant cousin.".into(),
                "*From here on, the little crab walks at your heels. The road gains eight small legs and two proud claws.*".into(),
            ],
            set: Some(fed_flag),
        },
    }
}

/// Little things carried because of side quests, shown in the journal's
/// satchel — derived from flags, never stored, like every keepsake.
pub fn carried(flags: &BTreeSet<String>) -> Vec<&'static str> {
    let mut v = Vec::new();
    if flags.contains(SORREL_MINT) && !flags.contains(SORREL_DONE) {
        v.push("a sprig of moon-mint");
    }
    if flags.contains(NETTLE_MET) && !flags.contains(CHEST_OPENED) {
        v.push("a small rusted key");
    }
    v
}

/// Journal memory-aids for side business currently underway.
pub fn journal_notes(flags: &BTreeSet<String>) -> Vec<&'static str> {
    let mut v = Vec::new();
    if flags.contains(SORREL_ASKED) && !flags.contains(SORREL_MINT) {
        v.push("Granny Sorrel hopes for moon-mint - it grows off the Echo Cave path.");
    }
    if flags.contains(SORREL_MINT) && !flags.contains(SORREL_DONE) {
        v.push("Bring the moon-mint back to Granny Sorrel's kettle.");
    }
    if flags.contains(NETTLE_MET) && !flags.contains(CHEST_OPENED) {
        v.push("Old Nettle's rusted key is stamped EMBERWICK STOREHOUSE.");
    }
    match crab_meals(flags) {
        1 => v.push(
            "The small crab behind the Emberwick storehouse took a morsel. Another, at a different hour, might go down well.",
        ),
        2 => v.push(
            "Two morsels down - the small crab all but waves now. One more, at yet another hour of the day.",
        ),
        _ => {}
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorrel_arc_walks_through_its_flags() {
        let mut flags = BTreeSet::new();
        let ask = talk("Granny Sorrel", &flags).unwrap();
        assert_eq!(ask.set, Some(SORREL_ASKED));
        flags.insert(SORREL_ASKED.to_string());
        assert_eq!(talk("Granny Sorrel", &flags).unwrap().set, None);
        flags.insert(SORREL_MINT.to_string());
        assert_eq!(
            talk("Granny Sorrel", &flags).unwrap().set,
            Some(SORREL_DONE)
        );
        flags.insert(SORREL_DONE.to_string());
        assert_eq!(talk("Granny Sorrel", &flags).unwrap().set, None);
    }

    #[test]
    fn carried_trinkets_come_and_go_with_the_arcs() {
        let mut flags = BTreeSet::new();
        assert!(carried(&flags).is_empty());
        flags.insert(SORREL_MINT.to_string());
        flags.insert(NETTLE_MET.to_string());
        assert_eq!(carried(&flags).len(), 2);
        flags.insert(SORREL_DONE.to_string());
        flags.insert(CHEST_OPENED.to_string());
        assert!(carried(&flags).is_empty());
    }

    #[test]
    fn three_meals_at_three_hours_tame_the_crab() {
        let mut flags = BTreeSet::new();
        assert_eq!(crab_meals(&flags), 0);
        assert!(!crab_tamed(&flags));
        // Each morsel's talk sets that hour's flag when the dialogue closes.
        let first = crab_talk(&flags, CRAB_FED[0]);
        assert_eq!(first.set, Some(CRAB_FED[0]));
        flags.insert(CRAB_FED[0].to_string());
        flags.insert(CRAB_FED[1].to_string());
        assert_eq!(crab_meals(&flags), 2);
        assert!(!crab_tamed(&flags), "two meals is not yet trust");
        assert_eq!(journal_notes(&flags).len(), 1, "the journal keeps a note");
        // Any three of the four hours will do — night counts.
        let third = crab_talk(&flags, CRAB_FED[3]);
        assert_eq!(third.set, Some(CRAB_FED[3]));
        flags.insert(CRAB_FED[3].to_string());
        assert!(crab_tamed(&flags));
        assert!(
            journal_notes(&flags).is_empty(),
            "a tamed friend needs no reminder"
        );
    }

    #[test]
    fn quest_npcs_have_no_side_business() {
        let flags = BTreeSet::new();
        assert!(talk("Elder Rowan", &flags).is_none());
        assert!(talk("Wren", &flags).is_none());
    }
}
