//! Ferris, the very small crab at your heels. You two go way back — long
//! before the road, long before the runes — and unlike most crabs he talks.
//! Press `e` with nothing else in reach and he'll offer whatever is on his
//! mind: which line that is derives from where you're standing and the hour
//! (`hash2`), so playthroughs stay deterministic and every corner of the road
//! has its own remark.

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

/// After dark he keeps his voice down — mostly.
pub const NIGHT_CHAT: &[&str] = &[
    "\"Stars,\" Ferris murmurs, \"are just very patient lanterns. A crab can respect that.\"",
    "Ferris produces one small, sleepy click. \"...five more minutes.\"",
    "\"Night air smells like the sea if you try hard enough.\" A pause. \"I'm trying.\"",
    "\"If you mean to walk till dawn,\" Ferris yawns, \"wake me for the good parts.\"",
];

/// The remark for this spot and hour. `seed` should come from `hash2`, so a
/// given square at a given time of day always says the same thing.
pub fn chat(seed: u32, night: bool) -> &'static str {
    let pool = if night { NIGHT_CHAT } else { CHAT };
    pool[seed as usize % pool.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_seed_finds_a_line_day_and_night() {
        for seed in 0..64 {
            assert!(!chat(seed, false).is_empty());
            assert!(!chat(seed, true).is_empty());
        }
        // Different seeds do reach different lines.
        assert_ne!(chat(0, false), chat(1, false));
    }
}
