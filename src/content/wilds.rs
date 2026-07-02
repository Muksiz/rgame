//! Wild runes: the little spirits that rustle in the tall grass. Meeting one
//! poses a small Rust question; answer true and the rune is inscribed in your
//! grimoire. Each zone grows its own kinds, themed on that zone's lessons —
//! wandering the grass becomes gentle spaced repetition.

pub struct WildRune {
    pub id: u8,
    /// Which overworld zone's grass it lives in (0-3).
    pub zone: usize,
    pub name: &'static str,
    /// How it announces itself before the question.
    pub stir: &'static str,
    pub prompt: &'static str,
    pub options: [&'static str; 3],
    /// Index into `options`.
    pub answer: usize,
    /// Inscribed in the grimoire once caught.
    pub lore: &'static str,
}

pub fn wild(id: u8) -> &'static WildRune {
    &WILDS[(id - 1) as usize]
}

pub fn in_zone(zone: usize) -> Vec<&'static WildRune> {
    WILDS.iter().filter(|w| w.zone == zone).collect()
}

pub static WILDS: [WildRune; 16] = [
    // ── Emberwick: println!, mut, numeric types ─────────────────────────────
    WildRune {
        id: 1,
        zone: 0,
        name: "the Bang Rune",
        stir: "Something small and excitable pops out of the grass, trailing an exclamation mark!",
        prompt: "Why does println! end with a `!` ?",
        options: [
            "It's a macro, and macros are called with !",
            "It marks the function as very fast",
            "It makes the text print in bold",
        ],
        answer: 0,
        lore: "Macros wear a ! so you always know they're macros. The Bang Rune is simply proud of it.",
    },
    WildRune {
        id: 2,
        zone: 0,
        name: "the Mut Rune",
        stir: "A stubborn little rune plants itself in front of you and refuses to budge.",
        prompt: "What does `let mut x = 5;` allow that `let x = 5;` does not?",
        options: [
            "Sharing x between threads",
            "Changing x's value later",
            "Making x global",
        ],
        answer: 1,
        lore: "Everything in Rust sits still unless asked, kindly, to move. `mut` is the asking.",
    },
    WildRune {
        id: 3,
        zone: 0,
        name: "the Cast Rune",
        stir: "Two runes argue in the grass — one whole, one pointy. They notice you.",
        prompt: "How do you multiply an i32 named `n` with an f64 named `f`?",
        options: [
            "n * f — numbers are numbers",
            "(n as f64) * f",
            "f.to_i32() * n",
        ],
        answer: 1,
        lore: "Rust never mixes number-kinds silently. `as` converts one, then they get along fine.",
    },
    WildRune {
        id: 4,
        zone: 0,
        name: "the Semicolon Sprite",
        stir: "A tiny wisp hangs in the air at the end of the sentence you were about to think;",
        prompt: "What does a trailing semicolon do to an expression in Rust?",
        options: [
            "Nothing, it's decoration",
            "Turns it into a statement (its value is dropped)",
            "Returns it from the function",
        ],
        answer: 1,
        lore: "A semicolon quietly sets a value down. Leave it off the last line, and the value walks out with you.",
    },
    // ── Whispering Woods: functions, if/else, loops ─────────────────────────
    WildRune {
        id: 5,
        zone: 1,
        name: "the Arrow Rune",
        stir: "A slender rune loosed from somewhere flies past → and circles back, curious.",
        prompt: "In `fn double(x: i32) -> i32`, what does the `->` mean?",
        options: [
            "The function type it returns",
            "A pointer dereference",
            "The function runs asynchronously",
        ],
        answer: 0,
        lore: "The arrow points at what comes out of the little machine. Wren would call it the out-door.",
    },
    WildRune {
        id: 6,
        zone: 1,
        name: "the Else Rune",
        stir: "The grass parts two ways at once. A rune sits at the fork, waiting for your answer.",
        prompt: "What does `if` in Rust evaluate to?",
        options: [
            "Nothing — if is only a statement",
            "A value: `let y = if x > 0 { 1 } else { -1 };` works",
            "Always a bool",
        ],
        answer: 1,
        lore: "In Rust the fork in the path is itself a value. Choose a branch, carry home what you find.",
    },
    WildRune {
        id: 7,
        zone: 1,
        name: "the Loop Rune",
        stir: "A rune jogs past you. A moment later it jogs past you. A moment later it jogs—",
        prompt: "Which loop is guaranteed to run at least once?",
        options: ["for x in 0..0", "while false", "loop { ... }"],
        answer: 2,
        lore: "`loop` runs until a `break` sets it free. (The game's rune-checker keeps a ten-second net, just in case.)",
    },
    WildRune {
        id: 8,
        zone: 1,
        name: "the Shadow Rune",
        stir: "Your shadow detaches, stretches, and becomes a second, newer shadow.",
        prompt: "What does shadowing mean: `let x = 5; let x = x + 1;` ?",
        options: [
            "A compile error — x already exists",
            "A brand-new x replaces the old one; even its type may change",
            "It mutates the original x in place",
        ],
        answer: 1,
        lore: "A new binding may quietly stand where an old one stood. The woods do this with trees, too.",
    },
    // ── Silverford: ownership, borrows, String vs &str ──────────────────────
    WildRune {
        id: 9,
        zone: 2,
        name: "the River Rune",
        stir: "A rune drifts by on the current. If you take it, the river won't have it anymore.",
        prompt: "After `let b = a;` where a is a String, what is true?",
        options: [
            "Both a and b own the text",
            "a has moved into b; a can't be used anymore",
            "b is a reference to a",
        ],
        answer: 1,
        lore: "A String has one owner, like a boat has one tiller. Hand it over and your hands are empty.",
    },
    WildRune {
        id: 10,
        zone: 2,
        name: "the Borrow Rune",
        stir: "A polite rune asks whether it might hold your satchel. Just for a moment. It promises.",
        prompt: "How many mutable references to a value may exist at once?",
        options: ["One", "Two", "As many as you like"],
        answer: 0,
        lore: "One borrower may change a thing at a time — that's how the river keeps everyone's letters dry.",
    },
    WildRune {
        id: 11,
        zone: 2,
        name: "the Turbofish",
        stir: "Something silver leaps from the water: ::<> !! The legendary Turbofish!",
        prompt: "What is the turbofish `::<>` for, as in `parse::<i32>()`?",
        options: [
            "Spawning a new thread",
            "Telling a generic function which type you mean",
            "Casting between numeric types",
        ],
        answer: 1,
        lore: "When the compiler can't guess your type, the Turbofish carries the answer upstream. Anglers speak of it in whispers.",
    },
    WildRune {
        id: 12,
        zone: 2,
        name: "the Str Rune",
        stir: "Two runes share one raincoat: a big warm one, and a thin quick one peeking out.",
        prompt: "What's the difference between String and &str?",
        options: [
            "None — they're synonyms",
            "String is owned and growable; &str borrows a view of text",
            "&str is the faster String for ASCII",
        ],
        answer: 1,
        lore: "One owns the letter, one reads it over your shoulder. Both are welcome at the dock.",
    },
    // ── Hearthspire: structs, impl, enums & match ───────────────────────────
    WildRune {
        id: 13,
        zone: 3,
        name: "the Struct Rune",
        stir: "Mist gathers itself into a neat little shape with named parts.",
        prompt: "What is a struct?",
        options: [
            "A type that groups named fields into one value",
            "A loop that never ends",
            "A special kind of module",
        ],
        answer: 0,
        lore: "Give scattered things one roof and a name each, and suddenly they're a home.",
    },
    WildRune {
        id: 14,
        zone: 3,
        name: "the Method Rune",
        stir: "A rune bows: it evidently belongs to something, and knows exactly what.",
        prompt: "In `impl Book { fn open(&self) {...} }`, what is `&self`?",
        options: [
            "A borrow of the Book the method is called on",
            "A global variable",
            "The method's return value",
        ],
        answer: 0,
        lore: "A method borrows the thing it serves. The Great Library approves of borrowing done properly.",
    },
    WildRune {
        id: 15,
        zone: 3,
        name: "the Match Rune",
        stir: "The mist offers you several doors at once, and insists you consider every one.",
        prompt: "What does the compiler require of a `match` on an enum?",
        options: [
            "Arms may cover whichever cases you like",
            "Every possible variant must be handled",
            "At most four arms",
        ],
        answer: 1,
        lore: "A match must answer for every possibility. That's not strictness — it's the librarian counting everyone in at closing.",
    },
    WildRune {
        id: 16,
        zone: 3,
        name: "the Option Rune",
        stir: "A rune holds out a small box. It may contain Some thing. It may contain None.",
        prompt: "What is Option<T> for?",
        options: [
            "Marking code as optional to compile",
            "A value that is either Some(T) or None — no nulls, ever",
            "Choosing compiler optimization levels",
        ],
        answer: 1,
        lore: "Rust has no null lurking in the dark; absence gets a name and a box. The mist finds this very tidy.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wilds_are_well_formed() {
        for (i, w) in WILDS.iter().enumerate() {
            assert_eq!(w.id as usize, i + 1, "ids are 1-based and in order");
            assert!(w.zone <= 3, "{} lives in a non-overworld zone", w.name);
            assert!(w.answer < w.options.len(), "{} has no right answer", w.name);
        }
        for zone in 0..=3 {
            assert!(
                in_zone(zone).len() >= 3,
                "zone {zone} needs at least a few wild runes"
            );
        }
    }
}
