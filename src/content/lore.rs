//! Campfire lore: little true things about Rust, told the way travellers
//! swap stories over embers. Shorter than the Library's books (`books.rs`) —
//! a single thought you can carry off to sleep. Pressing `e` by a campfire
//! draws one at random, then the night (or the morning) rolls in.

pub struct Lore {
    /// A speaker, for flavor — an ember, a passing thought, the fire itself.
    pub voice: &'static str,
    pub text: &'static str,
}

/// One campfire snippet, chosen by `seed` (any hash works; the caller supplies
/// the day-tick and position so the same fire on the same night tells the same
/// tale, but a later night tells another).
pub fn snippet(seed: u32) -> &'static Lore {
    &LORE[seed as usize % LORE.len()]
}

pub static LORE: [Lore; 18] = [
    Lore {
        voice: "An ember pops",
        text: "They say Rust owns nothing twice. Every value has one keeper; when the keeper goes to sleep, the value is tidied away — no midnight broom, no forgotten crumbs.",
    },
    Lore {
        voice: "The fire mutters",
        text: "A borrow is a loan of a thing, not the thing. &it lends a look, &mut it lends the pen — and only ever one pen at a time. That is why nothing here is ever scribbled over twice at once.",
    },
    Lore {
        voice: "A traveller recalls",
        text: "There is no null in these lands. A thing that might be missing wears a small box called Option: Some treasure inside, or an honest, empty None. No trapdoors.",
    },
    Lore {
        voice: "Smoke curls upward",
        text: "When a spell may fail, it hands back a Result — Ok this way, Err that way — and the little ? passes the trouble politely up the road. Failure, named kindly, stops being frightening.",
    },
    Lore {
        voice: "An old hand says",
        text: "Match every door. The compiler will not let a match close until you've answered for every possibility — not strictness, the locals insist, just the librarian counting everyone in at dusk.",
    },
    Lore {
        voice: "The coals whisper",
        text: "Two threads may share a thing only if the type carries the right ticket: Send to cross, Sync to be shared. Data races aren't caught out here — they simply cannot be written.",
    },
    Lore {
        voice: "A moth circles",
        text: "A trait names what a type can *do* — Display, Clone, Iterator. Ask a function for 'anything that can fly' and it never needs to know it was a moth.",
    },
    Lore {
        voice: "The fire settles",
        text: "Iterators here are lazy in the best way: map and filter promise the work but do none of it, until a for-loop or a collect finally asks. Cheap to describe, paid for only once.",
    },
    Lore {
        voice: "A spark drifts",
        text: "Anything ending in ! is a macro — println!, vec!, format! — code that writes code before the compiler ever reads it. The mark is a courtesy, so you always know when cleverness is afoot.",
    },
    Lore {
        voice: "An ember glows",
        text: "The lifetimes you sometimes see, those little 'a marks, are not decoration — they are map annotations, naming how long a borrow may safely wander. A reference here never outlives what it points at.",
    },
    Lore {
        voice: "The night hums",
        text: "cargo is a tireless shipwright: it builds, it tests, it fetches your crates from the harbor at crates.io, and it never once sighs about it. You describe the vessel in Cargo.toml; cargo does the rest.",
    },
    Lore {
        voice: "A log shifts",
        text: "Rust began as one programmer's side project at Mozilla in 2006, named after a fungus admired for being resilient and thoroughly over-engineered for survival. It kept growing.",
    },
    Lore {
        voice: "The fire remembers",
        text: "On the fifteenth of May, 2015, Rust 1.0 shipped with a quiet promise: stability without stagnation. Code that compiled that day compiles still, while a fresh release leaves the station every six weeks, like a very punctual ferry.",
    },
    Lore {
        voice: "Sparks rise",
        text: "'Zero-cost abstractions', the locals say with the smugness of people who have measured: the trait, the generic, the iterator — all woven together at build time, so the elegance costs nothing when the program actually runs.",
    },
    Lore {
        voice: "A cinder ticks",
        text: "unsafe is not a forbidden word, only a deliberate one. It says 'trust me' to the compiler and takes raw pointers in hand — so that someone can build the safe foundations the rest of us stand on.",
    },
    Lore {
        voice: "The embers agree",
        text: "Clippy is a friend with several hundred gentle opinions about your code, and rustfmt ends every argument about indentation by having it once, for everyone, forever.",
    },
    Lore {
        voice: "A drowsy voice",
        text: "Every few years brings an edition — 2015, 2018, 2021, 2024 — a bundle of refinements a crate may opt into whenever it pleases. They interoperate, so the barn is never burned to repaint a wall.",
    },
    Lore {
        voice: "The fire yawns",
        text: "Rustaceans, they call themselves, after Ferris the little crab. Some carry a Ferris of their own, who naps between compile errors and claims — unverifiably — to be a distant cousin of the original.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lore_snippets_are_well_formed() {
        for l in &LORE {
            assert!(!l.voice.is_empty());
            assert!(
                (40..=380).contains(&l.text.chars().count()),
                "a campfire tale won't fit the rest screen: {}",
                l.voice
            );
        }
        // The picker wraps, and different seeds can land on different tales.
        assert_eq!(snippet(0).text, LORE[0].text);
        assert_eq!(snippet(LORE.len() as u32).text, LORE[0].text);
    }
}
