pub struct Quest {
    pub id: u8,
    pub zone: usize,
    pub title: &'static str,
    pub npc: &'static str,
    pub file_name: &'static str,
    pub lesson: &'static str,
    pub template: &'static str,
    pub intro: &'static [&'static str],
    pub reminder: &'static str,
    pub success: &'static [&'static str],
    pub hints: &'static [&'static str],
}

pub fn quest(id: u8) -> &'static Quest {
    &QUESTS[(id - 1) as usize]
}

/// Zone a quest belongs to: quests 1-3 → zone 0, 4-6 → zone 1, and so on.
pub fn quest_zone(id: u8) -> usize {
    ((id - 1) / 3) as usize
}

/// Ferris's gentle one-liners when a rune fizzles.
pub static FIZZLE_LINES: &[&str] = &[
    "Ferris pokes the smoking rune with a claw. \"No harm done! The compiler left you a note — they always mean well.\"",
    "\"A fizzle!\" Ferris says, delighted. \"Every great rune-smith collects thousands of these.\"",
    "Ferris fans away the sparkles. \"Read the note from the bottom up, that's where the friendliest part lives.\"",
    "\"The rune got shy,\" Ferris explains. \"The compiler wrote down exactly why, though. Handy!\"",
    "Ferris pats the scroll. \"Rustc is the politest grump you'll ever meet. See what it says?\"",
];

pub static PASS_LINES: &[&str] = &[
    "Ferris does a little sideways dance. \"It holds! It HOLDS!\"",
    "\"Beautiful runework,\" Ferris whispers, misty-eyed.",
    "Ferris clicks both claws like castanets. \"Flawless!\"",
];

/// What the casting screen mumbles while rustc reads the scroll.
pub const WEAVING: [&str; 4] = [
    "weaving the runes…",
    "coaxing the borrow spirit…",
    "warming the type glyphs…",
    "asking rustc very nicely…",
];

pub static QUESTS: [Quest; 12] = [
    Quest {
        id: 1,
        zone: 0,
        title: "The Festival Lantern",
        npc: "Elder Rowan",
        file_name: "01_the_festival_lantern.rs",
        lesson: "println! and macros",
        template: include_str!("templates/01_the_festival_lantern.rs"),
        intro: &[
            "Oh! A traveler! And with a rune-satchel, no less. Welcome to Emberwick, dear. You've come at a lucky time — or an unlucky one, depending how you feel about chores.",
            "Tonight is the Lantern Festival, but the great lantern has hung dark for years. The lighting-spell is written down somewhere in my kitchen, but I always get the little mark at the end wrong. Macros, you see. They're particular.",
            "Would you take a look? Speak the old words exactly right, and she'll glow like a harvest moon.",
        ],
        reminder: "The spell is nearly right, I'm sure of it. Something small and pointy is missing at the end of the word... a little mark of excitement, perhaps?",
        success: &[
            "The lantern blooms with warm gold light, and the whole square goes 'oooh'. Rowan dabs an eye with her sleeve.",
            "\"There it is. Just as I remember. You've a gift for the old words, traveler — Poppy at the bakery could use a hand too, if you're staying a while.\"",
        ],
        hints: &[
            "In Rust, things like `println!` are macros, and macros are always called with a `!` after their name.",
            "Change `println(...)` to `println!(...)`. That single character is the whole quest. Cozy, right?",
        ],
    },
    Quest {
        id: 2,
        zone: 0,
        title: "The Baker's Ledger",
        npc: "Baker Poppy",
        file_name: "02_the_bakers_ledger.rs",
        lesson: "variables & mutability",
        template: include_str!("templates/02_the_bakers_ledger.rs"),
        intro: &[
            "You're the one who lit the lantern! Wonderful. Then you can surely help with my ledger — it refuses to add today's loaves.",
            "Twelve loaves this morning, nine more this evening, four set aside for the festival. Simple! But the ledger-rune insists the number cannot change once written.",
            "Stubborn thing. There must be a word that gives a number permission to change...",
        ],
        reminder: "The ledger still sulks. It says a value 'cannot be assigned twice'. There's a keyword for letting things change... three letters, very polite.",
        success: &[
            "The ledger flips its own pages and neatly writes '17' at the bottom. Poppy slides a warm honey-oat loaf into your satchel.",
            "\"In Rust-runes everything sits still unless you ask it kindly to move. `mut`, hm? I'll remember that. Bram at the well was muttering about numbers too...\"",
        ],
        hints: &[
            "Variables in Rust are immutable by default — `let loaves = 12;` can never change afterwards.",
            "Add the keyword `mut` after `let`: `let mut loaves = 12;`. Then the evening batch can be added.",
        ],
    },
    Quest {
        id: 3,
        zone: 0,
        title: "The Deep, Deep Well",
        npc: "Well-keeper Bram",
        file_name: "03_the_deep_well.rs",
        lesson: "numeric types & casting",
        template: include_str!("templates/03_the_deep_well.rs"),
        intro: &[
            "Ah, the lantern-lighter! Settle a village argument, would you? How deep is this well? I drop a pebble and count heartbeats until the splash. Two heartbeats, usually.",
            "There's an old falling-stone rune for this: half of nine-point-eight, times the seconds, times the seconds again. But my rune keeps refusing — it says whole numbers and pointy... 'floating' numbers won't multiply together.",
            "Numbers are numbers, I told it! It disagreed. Firmly.",
        ],
        reminder: "The rune still fusses about mixing number-kinds. Apparently you must *convert* one kind into the other before they'll multiply. `as`, was it?",
        success: &[
            "\"NINETEEN AND A BIT!\" Bram bellows down the well, and the well, politely, echoes it back. He looks enormously satisfied.",
            "\"So an i32 and an f64 won't mix unless you convert one. Sensible, if fussy. The whole village thanks you — the road east should be clear now, once the oak's rolled aside!\"",
        ],
        hints: &[
            "Rust never mixes numeric types silently: an `i32` times an `f64` is a compile error, not a guess.",
            "Convert the whole-number seconds with `as f64` before multiplying: `(seconds as f64)`.",
            "One tidy way: `let t = seconds as f64; 0.5 * 9.8 * t * t`",
        ],
    },
    Quest {
        id: 4,
        zone: 1,
        title: "A Spell for Wren",
        npc: "Wren",
        file_name: "04_a_spell_for_wren.rs",
        lesson: "writing functions",
        template: include_str!("templates/04_a_spell_for_wren.rs"),
        intro: &[
            "Hi!! Are you a real rune-smith? I'm going to be one too. I already know the BEST spell idea: a spell that makes every step count double. Twice the walking with the same feet!",
            "I tried to write it but... okay I don't actually know how to write a function yet. It needs a name, and it takes the paces in, and gives the doubled paces back out.",
            "Grandmother says a function is just a little machine with a door in and a door out. Can you build mine? Please please please?",
        ],
        reminder: "Did you build my spell yet?? It's called double_step! Paces go in, DOUBLE paces come out! (Grandmother says the arrow `->` shows what comes out.)",
        success: &[
            "Wren hops in a circle, counting: \"two, four, six, EIGHT — it works, it works!\" A nearby tree rustles, presumably applauding.",
            "\"When I'm big I'll write functions with a HUNDRED doors. Maren's in the mushroom hollow south of here — she likes people who are careful with rules!\"",
        ],
        hints: &[
            "A function needs: `fn`, a name, its inputs in parentheses, and `->` for what comes out.",
            "The shape is: `fn double_step(paces: i32) -> i32 { ... }`",
            "The last line of a function (no semicolon!) is what it returns: `paces * 2`",
        ],
    },
    Quest {
        id: 5,
        zone: 1,
        title: "Mushrooms & Manners",
        npc: "Forager Maren",
        file_name: "05_mushrooms_and_manners.rs",
        lesson: "if / else",
        template: include_str!("templates/05_mushrooms_and_manners.rs"),
        intro: &[
            "Careful where you step — that patch is dinner. You have a deciding-rune in that satchel? Good. I need one that never gets tired, because pickers DO get tired, and tired pickers eat the wrong mushroom.",
            "My rules, in order of importance: if it GLOWS, never. No exceptions, I don't care how pretty. Then: more than four spots, leave it. Otherwise? Straight into the basket.",
            "Write that down exactly. The forest respects people who are precise.",
        ],
        reminder: "Glowing? Never — check that FIRST. More than four spots? No. Otherwise, basket. `if` and `else` should do it.",
        success: &[
            "Maren tests your rune against her whole basket, twice. Every verdict lands right. She nods slowly, which from Maren is a standing ovation.",
            "\"Precise. I like precise. Ambrose is up in the meadow — fair warning, he'll be asleep.\"",
        ],
        hints: &[
            "Check the glow first: `if glows { return false; }` — order of checks matters here.",
            "Then the spots: `if spots > 4 { false } else { true }` — or simply return the comparison.",
            "The whole body can be one expression: `!glows && spots <= 4`. (Both styles are fine runework.)",
        ],
    },
    Quest {
        id: 6,
        zone: 1,
        title: "The Echo Cave",
        npc: "Shepherd Ambrose",
        file_name: "06_the_echo_cave.rs",
        lesson: "loops & building strings",
        template: include_str!("templates/06_the_echo_cave.rs"),
        intro: &[
            "*yawn* ...oh. Hello. Don't mind me, I was resting my eyes in an alert and professional manner. The sheep are fine. Probably. The problem is the echo cave, south of the road.",
            "My sheep love the echo. If it echoes 'baa' back enough times they settle right down for the night. But shouting is *work*, friend, and I am a man of... measured effort.",
            "An echo-rune, that's the thing. Give it a word and a number, and it repeats the word that many times with a space between. 'baa baa baa'. Bliss.",
        ],
        reminder: "The echo-rune, friend. A word, repeated N times, single spaces in between. No trailing space — the echo is fussy about trailing spaces. *yawn*",
        success: &[
            "You cast the rune into the cave. 'baa baa baa' rolls warmly back, and across the meadow every sheep folds itself down into the grass like a little cloud landing.",
            "Ambrose is already horizontal. \"Masterful,\" he murmurs. \"The gate east should open for you now... the woods approve of a well-rested flock.\" *snore*",
        ],
        hints: &[
            "A `for _ in 0..times` loop and a `String` you `push_str` onto will do nicely.",
            "The fiddly bit is the spaces: add a space *before* each word except the first, or trim at the end.",
            "One tidy pattern: collect into a Vec and `.join(\" \")`, or loop with `if !result.is_empty() { result.push(' '); }`",
        ],
    },
    Quest {
        id: 7,
        zone: 2,
        title: "The Ferry Token",
        npc: "Ferryman Wick",
        file_name: "07_the_ferry_token.rs",
        lesson: "ownership & moves",
        template: include_str!("templates/07_the_ferry_token.rs"),
        intro: &[
            "Crossing paperwork. Everyone's favorite. Rules of the Silverford: I inspect your ferry token, I write you a receipt, and — this is the important part — you get the token BACK. It's the only Number Seven in existence.",
            "But my inspection-rune is greedy. Hand it the token and the token is just... gone. Swallowed. 'Moved', the rune says, like that's an apology.",
            "Fix it so the rune can LOOK at the token without KEEPING it. Looking and keeping are different things, friend. Even the river knows that.",
        ],
        reminder: "The rune still swallows the token. It needs to *borrow* the thing, not take it. A little `&` goes a long way, I'm told.",
        success: &[
            "The rune inspects the token, stamps a receipt, and — miracle of miracles — the token is still in your hand afterward. Wick actually smiles, which briefly startles a frog.",
            "\"Ownership, eh? Everything has exactly one owner, and handing it over means it's gone — unless you just lend it. The river would like that rule. Juniper's fishing upstream, by the way.\"",
        ],
        hints: &[
            "Passing a `String` to a function *moves* it — the caller can't use it afterwards.",
            "Let the function borrow instead: change it to take `&str`, and pass `&token`.",
            "(`.clone()` also works — a copy for the rune to keep — but borrowing is the ferryman-approved way.)",
        ],
    },
    Quest {
        id: 8,
        zone: 2,
        title: "The Borrowed Rod",
        npc: "Fisher Juniper",
        file_name: "08_the_borrowed_rod.rs",
        lesson: "mutable references",
        template: include_str!("templates/08_the_borrowed_rod.rs"),
        intro: &[
            "Shhh — you'll scare the trout. There we go. Now: I lend my spare rod to half the riverbank, and I have one rule. Return it *better than you got it*. Sharpened hook, ideally.",
            "So I wrote a sharpening-rune. And it runs! It sharpens beautifully! ...a copy. It sharpens a copy, somewhere, and my actual rod stays dull as a Tuesday.",
            "The rune needs to work on MY rod. The one in my hands. Not some ghost-rod in rune-land.",
        ],
        reminder: "Still sharpening ghost-rods. The rune must borrow the real one — *mutably*, mind you. `&mut`, and a little star to reach through it.",
        success: &[
            "Two passes of the rune and the hook could split a hair lengthwise. Juniper tests it with her thumb and beams.",
            "\"THAT's a proper borrow. Lent, changed, returned — sharpness seven. Morrow's down on the beach, if you're heading that way. Bring patience.\"",
        ],
        hints: &[
            "To let a function change the caller's value, it must take `&mut i32` instead of `i32`.",
            "At the call site, lend it mutably: `sharpen(&mut sharpness);`",
            "Inside the function, reach through the reference with `*`: `*hook += 2;`",
        ],
    },
    Quest {
        id: 9,
        zone: 2,
        title: "A Message in a Bottle",
        npc: "Hermit Morrow",
        file_name: "09_a_message_in_a_bottle.rs",
        lesson: "String vs &str",
        template: include_str!("templates/09_a_message_in_a_bottle.rs"),
        intro: &[
            "The river brought me a letter today. In two pieces, as usual — the river is enthusiastic about delivery, less so about condition.",
            "I have both halves, soggy but readable. I need a mending-rune: take the two halves, join them with a single space, and then my signing-rune adds the farewell.",
            "Word-slices and owned words are different creatures, mind. `&str` is a window onto words; `String` is words you keep. The mending must produce the keeping kind.",
        ],
        reminder: "Two halves in, one whole `String` out, single space between. `format!` mends most things, I find.",
        success: &[
            "The halves knit together across the tear: 'meet me where the river sings — with love, M.' Morrow reads it three times and looks out at the water for a long moment.",
            "\"...An old friend. I should write back. Thank you, rune-smith — truly. The bridge east is Wick's business, but I suspect you've nearly earned it.\"",
        ],
        hints: &[
            "`format!(\"{a} {b}\")` builds a brand-new owned `String` from borrowed pieces.",
            "Or the long way: `let mut s = first_half.to_string(); s.push(' '); s.push_str(second_half); s`",
        ],
    },
    Quest {
        id: 10,
        zone: 3,
        title: "The Lost Book",
        npc: "Archivist Elm",
        file_name: "10_the_lost_book.rs",
        lesson: "structs",
        template: include_str!("templates/10_the_lost_book.rs"),
        intro: &[
            "Halt! ...at a leisurely pace. You carry a book. I can smell overdue ink from thirty yards. Before it comes home to the Library it must be CATALOGUED, and cataloguing requires a record.",
            "A record-rune bundles facts together so they travel as one: the title, the page count, the... ahem... years overdue. We call the bundle a `struct`. Dignified word.",
            "The form is already printed. You need only fill it in. Title: 'A Field Guide to Polite Dragons'. Pages: 312. Years overdue: 58. No judgement. Some judgement.",
        ],
        reminder: "The record awaits: title, pages 312, years overdue 58, all bundled in one `Book`. Field names, then a colon, then the value.",
        success: &[
            "The record-rune seals itself with a satisfying THUMP of invisible ink. Elm inspects it from four angles and finds nothing to correct, which visibly disappoints him.",
            "\"Catalogued. Fifty-eight years... the late fee will be waived, this once. The Golem at the last bend handles admissions — it will need waking. It always needs waking.\"",
        ],
        hints: &[
            "Build a struct value by naming each field: `Book { title: ..., pages: 312, years_overdue: 58 }`",
            "The title field is a `String`, so the text needs `.to_string()` (or `String::from(...)`).",
        ],
    },
    Quest {
        id: 11,
        zone: 3,
        title: "Waking the Golem",
        npc: "The Stone Golem",
        file_name: "11_the_stone_golem.rs",
        lesson: "impl & methods",
        template: include_str!("templates/11_the_stone_golem.rs"),
        intro: &[
            "The golem is asleep. Standing up. Snoring gravel. A small brass plate on its chest reads: 'ADMISSIONS. Wind me thrice. — The Management.'",
            "There's a winding-key and, tucked behind the plate, an instruction rune with three empty slots: one for making a new golem-heart, one for each turn of the key, and one for checking whether it's awake yet.",
            "Methods, the Library calls these — little abilities that belong to a thing. You'll need to carve all three into the `impl` block.",
        ],
        reminder: "Three methods on the golem: `new()` starts at zero, `wind_up` adds one charge (it'll need `&mut self`), and `is_awake` checks for three or more (`&self` will do).",
        success: &[
            "Three turns of the key. Deep in the stone chest something goes *whirr*, then *bong*, and two pebble-eyes grind open. \"ADMISSIONS,\" the golem booms, delighted. \"WELCOME. MIND THE STEP.\"",
            "It shakes your hand with alarming gentleness. \"THE SAGE AWAITS AT THE DOOR. SHE HAS BEEN AWAITING FOR FIFTY-EIGHT YEARS. SHE WILL SAY IT DOES NOT MATTER. IT DOES, A LITTLE.\"",
        ],
        hints: &[
            "Methods live in the `impl Golem { ... }` block. `new` is: `fn new() -> Golem { Golem { charge: 0 } }`",
            "`wind_up` changes the golem, so: `fn wind_up(&mut self) { self.charge += 1; }`",
            "`is_awake` only looks: `fn is_awake(&self) -> bool { self.charge >= 3 }`",
        ],
    },
    Quest {
        id: 12,
        zone: 3,
        title: "The Sorting of Spellbooks",
        npc: "Sage Alderly",
        file_name: "12_the_sorting_of_spellbooks.rs",
        lesson: "enums & match",
        template: include_str!("templates/12_the_sorting_of_spellbooks.rs"),
        intro: &[
            "So. The Field Guide comes home at last — and carried by a rune-smith, no less. Fitting. Before I shelve it, one final matter: the returns cart. Fifty years of books, and my sorting-rune half-finished on my desk.",
            "Every spellbook belongs to a school: Ember, Tide, Gale, or Stone. Each school has its shelf — one through four — and its motto, which the books hum quietly when shelved correctly. It's charming. Slightly eerie. Mostly charming.",
            "A `match` handles it: every school, every case, nothing forgotten — the rune itself refuses to compile if you miss one. The Library finds that very reassuring. Finish it, and your journey's done.",
        ],
        reminder: "Two match-runes: shelf numbers (Ember 1, Tide 2, Gale 3, Stone 4) and mottos. The compiler will insist you cover every school — let it insist.",
        success: &[
            "The cart of spellbooks rises, sorts itself into four neat streams, and settles shelf by shelf. All through the Library, books begin to hum their mottos — warm hands, everything flows, lightly now, patience, patience.",
            "Alderly places the Field Guide in the last empty slot on shelf one. \"There,\" she says. \"Every book home. Every rune learned. Rest a while, rune-smith — the Library has excellent armchairs, and you have earned one.\"",
        ],
        hints: &[
            "A match on an enum lists every variant: `match school { School::Ember => 1, School::Tide => 2, ... }`",
            "For the mottos, match again but return the `&'static str` for each school.",
            "No `_` catch-all needed — with all four variants listed, the compiler knows you've covered everything.",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_ids_and_zones_line_up() {
        for (i, q) in QUESTS.iter().enumerate() {
            assert_eq!(q.id as usize, i + 1);
            assert_eq!(q.zone, quest_zone(q.id));
            assert!(!q.hints.is_empty());
            assert!(q.file_name.ends_with(".rs"));
        }
    }
}
