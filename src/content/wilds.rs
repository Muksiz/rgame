//! Wild runes: the little spirits that rustle in the tall grass. Meeting one
//! poses a small Rust question; answer true and the rune is inscribed in your
//! grimoire. Each zone grows its own kinds, themed on that zone's chapter of
//! the book — wandering the grass becomes gentle spaced repetition, and no
//! zone's grass ever quizzes past its chapter.

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
    // ── Emberwick, ch. 3: println!, mut, functions, statements & expressions ─
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
        name: "the Arrow Rune",
        stir: "A slender rune loosed from somewhere flies past → and circles back, curious.",
        prompt: "In `fn double(x: i32) -> i32`, what does the `->` mean?",
        options: [
            "The type the function returns",
            "A pointer dereference",
            "The function runs asynchronously",
        ],
        answer: 0,
        lore: "The arrow points at what comes out of the little machine — the out-door, as the village children call it.",
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
    // ── Whispering Woods, ch. 4: ownership, borrows, slices ─────────────────
    WildRune {
        id: 5,
        zone: 1,
        name: "the Move Rune",
        stir: "A rune scurries off with an acorn. The acorn's former keeper looks at their empty paws.",
        prompt: "After `let b = a;` where a is a String, what is true?",
        options: [
            "Both a and b own the text",
            "a has moved into b; a can't be used anymore",
            "b is a reference to a",
        ],
        answer: 1,
        lore: "A String has one owner, like a basket has one handle. Hand it over and your hands are empty.",
    },
    WildRune {
        id: 6,
        zone: 1,
        name: "the Clone Rune",
        stir: "The rune splits into two identical runes. They wave at each other, delighted.",
        prompt: "What does `spell.clone()` give you, for a String named spell?",
        options: [
            "A second name for the same String",
            "A read-only view of spell",
            "An independent copy — both are usable afterwards",
        ],
        answer: 2,
        lore: "Two of a thing, honestly made, each with its own life. The woods approve of twins.",
    },
    WildRune {
        id: 7,
        zone: 1,
        name: "the Borrow Rune",
        stir: "A polite rune asks whether it might hold your satchel. Just for a moment. It promises.",
        prompt: "How many mutable references to a value may exist at once?",
        options: ["One", "Two", "As many as you like"],
        answer: 0,
        lore: "One borrower may change a thing at a time — that's how the woods keep everyone's letters dry.",
    },
    WildRune {
        id: 8,
        zone: 1,
        name: "the Slice Rune",
        stir: "A thin rune peeks at just the corner of your map, and swears that's all it needs.",
        prompt: "What does `&s[..3]` give you, for a String named s?",
        options: [
            "A new String holding three letters",
            "A borrowed view of the first three letters — nothing copied",
            "The last three letters",
        ],
        answer: 1,
        lore: "To see a part, you needn't take the whole. A slice is a window cut just wide enough.",
    },
    // ── Silverford, ch. 5.1–5.2: structs, fields, update syntax, Debug ──────
    WildRune {
        id: 9,
        zone: 2,
        name: "the Struct Rune",
        stir: "River-fog gathers itself into a neat little shape with named parts.",
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
        id: 10,
        zone: 2,
        name: "the Dot Rune",
        stir: "A small rune hops from crate to crate along the dock, tapping each label precisely.",
        prompt: "How do you read the `pages` field of a struct value named book?",
        options: ["pages(book)", "book::pages", "book.pages"],
        answer: 2,
        lore: "One small dot, and the bundle opens to exactly the fact you asked for. Dockhands dream of such filing.",
    },
    WildRune {
        id: 11,
        zone: 2,
        name: "the Update Rune",
        stir: "Something silver leaps from the water wearing last year's scales — the same fish, one part new.",
        prompt: "What does `Rod { sharpness: 9, ..old }` build?",
        options: [
            "A new Rod: sharpness 9, every other field from `old`",
            "It changes `old` in place",
            "A Rod with only a sharpness field",
        ],
        answer: 0,
        lore: "Mostly the old thing, one part new. The river does this with itself constantly, and calls it staying the same.",
    },
    WildRune {
        id: 12,
        zone: 2,
        name: "the Derive Rune",
        stir: "A rune clears its throat and reads your satchel's entire contents aloud, unprompted.",
        prompt: "What lets `{:?}` print your struct?",
        options: [
            "Structs print with {:?} automatically",
            "#[derive(Debug)] above the definition",
            "Adding a to_string field",
        ],
        answer: 1,
        lore: "Opt in with one line, and the compiler writes the whole reading-aloud spell for you. It's very good at it.",
    },
    // ── Hearthspire, ch. 5.3: methods, &mut self, associated functions ──────
    WildRune {
        id: 13,
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
        id: 14,
        zone: 3,
        name: "the Winding Rune",
        stir: "A rune turns a tiny key in its own back, winding itself tighter and tighter.",
        prompt: "A method that changes the struct it's called on takes...?",
        options: ["&self", "no self at all", "&mut self"],
        answer: 2,
        lore: "To change a thing you must hold the pen, not just read the page. One winder at a time, says the Management.",
    },
    WildRune {
        id: 15,
        zone: 3,
        name: "the Summoning Rune",
        stir: "With a pop, a rune appears out of nowhere — summoned, apparently, by no one in particular.",
        prompt: "How is an associated function like `new` called?",
        options: [
            "golem.new() — on a value, with a dot",
            "Golem::new() — on the type, with ::",
            "new(Golem) — plain and simple",
        ],
        answer: 1,
        lore: "Some abilities belong to the kind, not the individual. You knock on the type's own door: two colons, twice.",
    },
    WildRune {
        id: 16,
        zone: 3,
        name: "the Mirror Rune",
        stir: "The mist shows you a rune admiring its reflection. The reflection is, of course, also a rune.",
        prompt: "Inside `impl Book`, what does `Self` (capital S) mean?",
        options: [
            "The type Book itself",
            "The instance the method was called on",
            "The parent module",
        ],
        answer: 0,
        lore: "Inside its own impl block, a type may call itself Self — every home needs a name for 'here'.",
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
