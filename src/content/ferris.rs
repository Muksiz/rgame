//! Ferris, the very small crab at your heels. You two go way back — long
//! before the road, long before the runes — and unlike most crabs he talks.
//! Press `e` with nothing else in reach and he'll offer whatever is on his
//! mind: which line that is derives from where you're standing and the hour
//! (`hash2`), so playthroughs stay deterministic and every corner of the road
//! has its own remark — and he has settled opinions about each region too.

/// Daytime chatter: road wisdom, snack policy, and light slander of lobsters.
pub const CHAT: &[&str] = &[
    "\"You know,\" Ferris says, \"we've shared a lot of roads, you and I. This one has the best puddles so far. I keep notes.\"",
    "\"The compiler isn't grumpy,\" Ferris says, apropos of nothing. \"It's precise. So am I, and you like me fine.\"",
    "\"Borrow, don't take. Give back what you're lent. Honestly,\" Ferris clicks, \"you land-folk needed a whole book for that?\"",
    "\"Snack report: no snacks.\" Ferris looks up at you. \"I'm telling you as a courtesy. The situation is ongoing.\"",
    "\"If anyone asks,\" Ferris says with dignity, \"I am not a small lobster. We do not speak of lobsters.\"",
    "\"Mind the tall grass. Things rustle in there that ask QUESTIONS.\" Ferris shudders pleasantly. \"I love questions.\"",
    "\"Sideways,\" Ferris says, demonstrating, \"is a perfectly good direction. Most progress is sideways. Look at yours.\"",
    "\"When we reach the Great Library, I want the shelf with the crustacean poetry. There's always one. Don't argue.\"",
    "Ferris watches the horizon a moment. \"The sea's out there somewhere past the last page of this journey. No hurry. Good company here.\"",
    "\"Remember when you couldn't tell a rune from a river stone?\" Ferris clicks fondly. \"Yesterday. It was yesterday.\"",
];

/// What he thinks of each overworld region, folded into the local rotation.
pub const ZONE_CHAT: [&[&str]; 4] = [
    // Emberwick
    &[
        "\"Village air,\" Ferris pronounces, taking some in. \"Bread, woodsmoke, chickens plotting. The good stuff.\"",
        "\"I rate this village's puddles four claws. The one by the well is a five, but I don't want it getting a reputation.\"",
        "\"Everyone here waves at you,\" Ferris observes. \"I've decided some of those waves are for me. Roughly half.\"",
        "\"The fountain and I have an understanding,\" Ferris says, not elaborating in any way.",
        "\"Start small, they say. HA.\" Ferris gestures at himself with both claws. \"Look how far it gets you.\"",
    ],
    // Whispering Woods
    &[
        "\"The trees are whispering,\" Ferris confirms, listening. \"Mostly about moisture. Trees are simple folk.\"",
        "\"It's not that I dislike the woods,\" Ferris says, from very close to your heel. \"I simply prefer to be adjacent to you in them.\"",
        "\"An owl looked at me,\" Ferris reports. \"I looked back. We have agreed to file it under weather.\"",
        "\"All this shade is excellent for the complexion,\" says the permanently orange crab.",
        "\"The mushrooms out here know things,\" Ferris whispers. \"Don't ask them. It only encourages the ones that don't.\"",
    ],
    // Silverford
    &[
        "\"Now THIS,\" Ferris says, claws wide in the rain, \"is weather. Finally. My people summer in this.\"",
        "\"Fresh water,\" Ferris sniffs. \"No salt, no tide, fish with no manners. Still — it's water, and I respect water.\"",
        "\"The river and I talked shop,\" Ferris says. \"It's carrying everything downstream one thing at a time. One owner per parcel. Sensible fellow.\"",
        "\"Piers,\" Ferris says approvingly. \"Underneath every pier is a whole town for folk my size. Prime real estate.\"",
        "\"I waved at the ducks. Sea-cousins of a sort.\" A pause. \"They ruled the wave admissible. Courts, honestly.\"",
    ],
    // Hearthspire
    &[
        "\"Thin air up here.\" Ferris breathes it anyway, on principle. \"A crab of my calibre acclimatizes instantly, of course.\"",
        "\"We climbed a MOUNTAIN,\" Ferris says, awed at the both of you. \"Sideways the whole way, if anyone asks how I managed.\"",
        "\"The mist keeps trying to be mysterious,\" Ferris clicks. \"It's just shy water. I'd know. Family trait.\"",
        "\"A library at the top of the world.\" Ferris nods slowly. \"That's where I'd keep the good books too.\"",
        "\"Look how far the road goes back,\" Ferris says quietly, looking west. \"Every step of it yours. I counted.\"",
    ],
];

/// Under a roof he reviews the accommodations instead.
pub const INDOOR_CHAT: &[&str] = &[
    "\"Cozy,\" Ferris rules, testing the floor with one claw. \"Solid boards. A crab could winter here.\"",
    "\"I've inspected the corners,\" Ferris reports. \"All four accounted for. You may relax.\"",
    "\"Indoors is just a cave with manners,\" Ferris observes, settling in. \"I mean that as high praise.\"",
    "\"Notice how the rain isn't landing on us?\" Ferris says. \"Architecture. Marvelous spell, that.\"",
];

/// After dark he keeps his voice down — mostly.
pub const NIGHT_CHAT: &[&str] = &[
    "\"Stars,\" Ferris murmurs, \"are just very patient lanterns. A crab can respect that.\"",
    "Ferris produces one small, sleepy click. \"...five more minutes.\"",
    "\"Night air smells like the sea if you try hard enough.\" A pause. \"I'm trying.\"",
    "\"If you mean to walk till dawn,\" Ferris yawns, \"wake me for the good parts.\"",
    "\"The moon's a rune somebody cast ages ago and never cleaned up,\" Ferris murmurs. \"Sloppy. Beautiful, but sloppy.\"",
    "Ferris counts something on his claws, loses his place, and starts over. \"Sheep don't work for crabs,\" he explains.",
];

/// The remark for this spot and hour. `seed` should come from `hash2`, so a
/// given square at a given time of day always says the same thing. By day the
/// general pool and the local region's remarks rotate together (`region` from
/// `zones::region_of`; `None` means indoors); night keeps its own quiet set.
pub fn chat(seed: u32, night: bool, region: Option<usize>) -> &'static str {
    if night {
        return NIGHT_CHAT[seed as usize % NIGHT_CHAT.len()];
    }
    let local: &[&str] = match region.and_then(|r| ZONE_CHAT.get(r)) {
        Some(lines) => lines,
        None => INDOOR_CHAT,
    };
    let i = seed as usize % (CHAT.len() + local.len());
    if i < CHAT.len() {
        CHAT[i]
    } else {
        local[i - CHAT.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_seed_finds_a_line_day_and_night() {
        for region in (0..8).map(Some).chain([None]) {
            for seed in 0..64 {
                assert!(!chat(seed, false, region).is_empty());
                assert!(!chat(seed, true, region).is_empty());
            }
        }
        // Different seeds do reach different lines.
        assert_ne!(chat(0, false, Some(0)), chat(1, false, Some(0)));
    }

    #[test]
    fn every_region_gets_its_own_remarks() {
        // Some seed in each region (and indoors) lands on a local line — one
        // that no other pool contains.
        let pools = ZONE_CHAT.iter().map(|p| Some(*p)).chain([None]);
        for (i, pool) in pools.enumerate() {
            let (region, local) = match pool {
                Some(p) => (Some(i), p),
                None => (None, INDOOR_CHAT),
            };
            let heard: Vec<&str> = (0..64).map(|s| chat(s, false, region)).collect();
            assert!(
                local.iter().any(|l| heard.contains(l)),
                "region {region:?} never says its local lines"
            );
        }
    }
}
