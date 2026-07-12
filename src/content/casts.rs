//! Castable grimoire runes: every wild rune caught in the grass can be cast
//! from the casting ring (`r` in the world) — a small, gentle piece of
//! overworld magic keyed to the rune's nature. Charm, not keys: no cast may
//! substitute for a keepsake or open anything gated; the storm-lantern and
//! the rod keep their jobs. Every effect is pure light and motion, derived
//! from the tick — nothing persists, nothing is saved, casting is free and
//! cannot fail.

/// The shape a cast takes over the world. Cosmetic, every one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CastShape {
    /// Flowers bloom in a ring around your feet, then close again.
    Bloom,
    /// Rings ripple outward through the air, as on still water.
    Ripple,
    /// A scatter of startled birds takes wing.
    Birds,
    /// A bright spark and a slow fall of chiming motes.
    Chime,
    /// A moment of warm light. It does not count as the lantern.
    Gleam,
    /// Ferris weighs in.
    Ferris,
    /// Sparks drift toward the nearest unfound runestone, for a breath —
    /// the one small honest utility in the ring.
    Seek,
}

pub struct Cast {
    pub shape: CastShape,
    /// What the world (or Ferris) says when the cast lands.
    pub line: &'static str,
}

/// The cast belonging to a wild rune id (1-based, like `wilds::wild`).
pub fn cast(id: u8) -> &'static Cast {
    &CASTS[(id - 1) as usize]
}

/// One cast per wild rune, in `wilds::WILDS` order — each keyed to the
/// rune's name and nature, never to anything the world gates.
pub static CASTS: [Cast; 20] = [
    // 1 — the Bang Rune: proud of its !.
    Cast {
        shape: CastShape::Chime,
        line: "The Bang Rune goes off with one bright, delighted chime!",
    },
    // 2 — the Mut Rune: change, asked for kindly.
    Cast {
        shape: CastShape::Bloom,
        line: "Asked kindly, the ground agrees to change: flowers, briefly.",
    },
    // 3 — the Arrow Rune: loosed, and something takes wing.
    Cast {
        shape: CastShape::Birds,
        line: "The Arrow Rune whistles past and the hedgerow bursts into wings.",
    },
    // 4 — the Semicolon Sprite: a value, quietly set down.
    Cast {
        shape: CastShape::Ripple,
        line: "The Semicolon Sprite sets the moment down; it ripples, and rests.",
    },
    // 5 — the Move Rune: something rushes off through the grass.
    Cast {
        shape: CastShape::Birds,
        line: "Something small sprints off with an invisible acorn. Every bird objects.",
    },
    // 6 — the Clone Rune: honest copies, each with its own life.
    Cast {
        shape: CastShape::Bloom,
        line: "One flower, then its twin, then theirs — a ring of honest copies.",
    },
    // 7 — the Borrow Rune: it takes a moment of Ferris's attention. Just for
    // a moment. It promises.
    Cast {
        shape: CastShape::Ferris,
        line: "Ferris: \"BORROWED?! Fine. One opinion, returned promptly: the sea is that way.\"",
    },
    // 8 — the Slice Rune: a window cut just wide enough.
    Cast {
        shape: CastShape::Ripple,
        line: "A thin window opens on the air, admires the view, and closes. Nothing taken.",
    },
    // 9 — the Struct Rune: scattered light under one roof.
    Cast {
        shape: CastShape::Gleam,
        line: "The light gathers itself into something home-shaped, just for a heartbeat.",
    },
    // 10 — the Dot Rune: one precise chime, exactly where asked.
    Cast {
        shape: CastShape::Chime,
        line: "One precise little chime, exactly where you asked for it.",
    },
    // 11 — the Update Rune: mostly the old air, one part new.
    Cast {
        shape: CastShape::Ripple,
        line: "The air stays mostly itself, one ring at a time new. The river approves.",
    },
    // 12 — the Derive Rune: someone volunteers to read things aloud.
    Cast {
        shape: CastShape::Ferris,
        line: "Ferris: \"Contents of satchel: crumbs, one (1) travel companion, EXCELLENT. Derived!\"",
    },
    // 13 — the Method Rune: light that belongs to something, and knows what.
    Cast {
        shape: CastShape::Gleam,
        line: "A polite glow settles around you — it evidently belongs here.",
    },
    // 14 — the Winding Rune: tick, tick, chime.
    Cast {
        shape: CastShape::Chime,
        line: "A tiny key turns somewhere. Tick, tick — chime, right on time.",
    },
    // 15 — the Summoning Rune: pop — a flock, summoned by no one.
    Cast {
        shape: CastShape::Birds,
        line: "Pop. A whole flock appears out of nowhere, summoned by no one in particular.",
    },
    // 16 — the Mirror Rune: the mist shows you what's hidden.
    Cast {
        shape: CastShape::Seek,
        line: "The mist tilts like a mirror — sparks lean toward something still hidden.",
    },
    // 17 — the Tide Rune: in, and out, and in.
    Cast {
        shape: CastShape::Ripple,
        line: "The air goes tidal: in, and out, and in. Ebb or Flood, never both.",
    },
    // 18 — the Eightfold Rune: an arm for every case. Ferris counts claws.
    Cast {
        shape: CastShape::Ferris,
        line: "Ferris: \"Eight arms?! I have two claws and I am EXHAUSTIVE with them.\"",
    },
    // 19 — the Maybe Rune: a light in the mist, definitely Some.
    Cast {
        shape: CastShape::Gleam,
        line: "A lantern-light blooms at your shoulder. This one is definitely Some.",
    },
    // 20 — the Letting Rune: the one shape it cares about, circling.
    Cast {
        shape: CastShape::Ripple,
        line: "One ring circles you like the fish you were waiting for. The rest swim by.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::wilds::WILDS;

    #[test]
    fn every_wild_rune_has_a_cast() {
        assert_eq!(CASTS.len(), WILDS.len(), "one cast per wild rune");
        for (w, c) in WILDS.iter().zip(CASTS.iter()) {
            assert!(!c.line.is_empty(), "{} casts silently", w.name);
            if c.shape == CastShape::Ferris {
                assert!(
                    c.line.starts_with("Ferris:"),
                    "{} weighs Ferris in without him speaking",
                    w.name
                );
            }
        }
        // The ring keeps its one honest utility, and Ferris his opinions.
        assert!(CASTS.iter().any(|c| c.shape == CastShape::Seek));
        assert!(CASTS.iter().any(|c| c.shape == CastShape::Ferris));
    }
}
