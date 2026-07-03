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

pub static QUESTS: [Quest; 23] = [
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
            "\"There it is. Just as I remember. You've a gift for the old words, traveler — there's a girl minding the market stall who could use a hand with her sign, if you're staying a while.\"",
        ],
        hints: &[
            "In Rust, things like `println!` are macros, and macros are always called with a `!` after their name.",
            "Change `println(...)` to `println!(...)`. That single character is the whole quest. Cozy, right?",
        ],
    },
    Quest {
        id: 2,
        zone: 0,
        title: "The Market Sign",
        npc: "Tansy",
        file_name: "02_the_market_sign.rs",
        lesson: "shadowing & multi-value formatting",
        template: include_str!("templates/02_the_market_sign.rs"),
        intro: &[
            "Oh! You're the one who lit Rowan's lantern. I heard the whole square cheer from here. I've got a much SMALLER problem, but it's driving me up the wall.",
            "Nine more apples came in from the orchard cart this morning, and my sign-rune won't say so — it's stuck on this morning's count. And it's forgotten the pears exist entirely. One thing at a time, I keep telling it. ONE THING AT A TIME.",
            "You can bind a name again with `let` to quietly swap in a new value — Grandmother calls it 'shadowing'. And the sign should just... say both fruits. Can you fix it?",
        ],
        reminder: "Still says twelve apples and not a word about pears. Shadow `apples` with the new count, and give the sign a second `{}` for the pears.",
        success: &[
            "The chalk rearranges itself mid-air: '21 apples and 7 pears on the stall today.' Tansy claps once, delighted.",
            "\"PERFECT. Oh — Poppy's ledger is being just as stubborn, if you've got another minute. She's right through that door.\"",
        ],
        hints: &[
            "Shadowing: bind the same name again with a new `let` to replace the old value — `let apples = apples + 9;`.",
            "`format!`/`println!` can hold more than one value: add a second placeholder, `{pears}`, right in the string.",
        ],
    },
    Quest {
        id: 3,
        zone: 0,
        title: "The Baker's Ledger",
        npc: "Baker Poppy",
        file_name: "03_the_bakers_ledger.rs",
        lesson: "variables & mutability",
        template: include_str!("templates/03_the_bakers_ledger.rs"),
        intro: &[
            "You're the one who lit the lantern! Wonderful. Then you can surely help with my ledger — it refuses to add today's loaves.",
            "Twelve loaves this morning, nine more this evening, four set aside for the festival. Simple! But the ledger-rune insists the number cannot change once written.",
            "Stubborn thing. There must be a word that gives a number permission to change...",
        ],
        reminder: "The ledger still sulks. It says a value 'cannot be assigned twice'. There's a keyword for letting things change... three letters, very polite.",
        success: &[
            "The ledger flips its own pages and neatly writes '17' at the bottom. Poppy slides a warm honey-oat loaf into your satchel.",
            "\"In Rust-runes everything sits still unless you ask it kindly to move. `mut`, hm? I'll remember that. Watchman Fitch was grumbling about his gate-rune too — try the east archway.\"",
        ],
        hints: &[
            "Variables in Rust are immutable by default — `let loaves = 12;` can never change afterwards.",
            "Add the keyword `mut` after `let`: `let mut loaves = 12;`. Then the evening batch can be added.",
        ],
    },
    Quest {
        id: 4,
        zone: 0,
        title: "Open or Closed",
        npc: "Watchman Fitch",
        file_name: "04_open_or_closed.rs",
        lesson: "booleans & comparisons",
        template: include_str!("templates/04_open_or_closed.rs"),
        intro: &[
            "Halt. ...Oh, it's you, the lantern-and-ledger one. Word travels fast in a village this size. I've got a gate-rune that's supposed to wave people through for free — children, and anyone who lives here — but it just stands there. Silently judging everyone. Even the children.",
            "It needs to answer one honest question: true or false, waived or not. In Rust that's a `bool` — and you build one straight out of comparisons, joined with `&&` for 'and' or `||` for 'or'.",
            "Under twelve, OR a local. Whichever fits. No `if` required — the comparison itself IS the answer, once you write it right.",
        ],
        reminder: "The gate-rune still charges everybody. `age < 12 || is_local` is the shape of it — just needs writing in.",
        success: &[
            "The gate-rune hums once and waves a passing child through with a little chime. Fitch actually smiles, which unsettles a nearby pigeon.",
            "\"Clean and honest, that. Old Hobb's toll-board could use the same treatment — he's just past the well, muttering at a sign that won't hold a number.\"",
        ],
        hints: &[
            "A `bool` is just `true` or `false` — comparisons like `age < 12` already produce one directly.",
            "Combine two conditions with `||` (or) or `&&` (and): `age < 12 || is_local`.",
        ],
    },
    Quest {
        id: 5,
        zone: 0,
        title: "The Toll Board",
        npc: "Toll-keeper Hobb",
        file_name: "05_the_toll_sign.rs",
        lesson: "constants",
        template: include_str!("templates/05_the_toll_sign.rs"),
        intro: &[
            "Four coins, four slots on the board, every wagon, every day — has been for thirty years and I intend for it to stay that way. But my board-rune keeps asking ME what the numbers are, like I might have changed my mind overnight.",
            "I haven't. I won't. I want it CARVED in, not chalked on — a value the rune isn't even allowed to wonder about.",
            "There's a word for that in the old books: `const`. Always uppercase, so you can spot one at a glance. Declare the price and the slot-count as constants, and the board will finally hold still.",
        ],
        reminder: "The board still can't find TOLL_PRICE or TOLL_SLOTS anywhere. `const NAME: Type = value;`, carved in above the rune.",
        success: &[
            "The board resets itself with a satisfying click — four coins, four slots, and not the faintest flicker of doubt. Hobb nods once, gruffly pleased.",
            "\"Now THAT'LL hold. Reed's over by the map table if you're looking for more work — always fussing over some pin or other.\"",
        ],
        hints: &[
            "`const` declares a value that can never change, checked at compile time: `const TOLL_PRICE: u32 = 4;`.",
            "A `const` can size a fixed array where a plain `let` can't: `const TOLL_SLOTS: usize = 4;` then `[u32; TOLL_SLOTS]`.",
        ],
    },
    Quest {
        id: 6,
        zone: 0,
        title: "The Deep, Deep Well",
        npc: "Well-keeper Bram",
        file_name: "06_the_deep_well.rs",
        lesson: "numeric types & casting",
        template: include_str!("templates/06_the_deep_well.rs"),
        intro: &[
            "Ah, the lantern-lighter! Settle a village argument, would you? How deep is this well? I drop a pebble and count heartbeats until the splash. Two heartbeats, usually.",
            "There's an old falling-stone rune for this: half of nine-point-eight, times the seconds, times the seconds again. But my rune keeps refusing — it says whole numbers and pointy... 'floating' numbers won't multiply together.",
            "Numbers are numbers, I told it! It disagreed. Firmly.",
        ],
        reminder: "The rune still fusses about mixing number-kinds. Apparently you must *convert* one kind into the other before they'll multiply. `as`, was it?",
        success: &[
            "\"NINETEEN AND A BIT!\" Bram bellows down the well, and the well, politely, echoes it back. He looks enormously satisfied.",
            "\"So an i32 and an f64 won't mix unless you convert one. Sensible, if fussy. Reed's up at the map table, if you haven't met her yet — pins on the village map, apparently very particular ones.\"",
        ],
        hints: &[
            "Rust never mixes numeric types silently: an `i32` times an `f64` is a compile error, not a guess.",
            "Convert the whole-number seconds with `as f64` before multiplying: `(seconds as f64)`.",
            "One tidy way: `let t = seconds as f64; 0.5 * 9.8 * t * t`",
        ],
    },
    Quest {
        id: 7,
        zone: 0,
        title: "The Map Pins",
        npc: "Cartographer Reed",
        file_name: "07_the_map_pins.rs",
        lesson: "tuples",
        template: include_str!("templates/07_the_map_pins.rs"),
        intro: &[
            "You're the one straightening out the village's little troubles, I hear. Good — I've got a mapping-rune that can't keep a location straight. A landmark is two numbers, column and row, and they belong TOGETHER, never split up, never swapped.",
            "In the old tongue that's a tuple — `(12, 4)`, one thing holding two. Pin the well where it belongs, and read a pin back out for me: column first, then row, plain as a signpost.",
            "Get that right, and this map is finally trustworthy — which matters more than you'd think, once you're past the village gate.",
        ],
        reminder: "The well still isn't pinned. Build the tuple `(12, 4)`, then pull it apart — `pin.0`/`pin.1`, or `let (x, y) = pin;` — to write the label.",
        success: &[
            "The map redraws itself, pins snapping neatly into place. Reed studies it from three angles, finds nothing loose, and looks almost disappointed about that.",
            "\"There. A map I can trust. Yours, too, from here — the road east is clear, and the Whispering Woods are patient, but not THAT patient. Go on.\"",
        ],
        hints: &[
            "A tuple bundles values together with parentheses: `(12, 4)` is a value of type `(i32, i32)`.",
            "Pull the pieces back out by position (`pin.0`, `pin.1`) or by destructuring: `let (column, row) = pin;`.",
        ],
    },
    Quest {
        id: 8,
        zone: 1,
        title: "Counting Fireflies",
        npc: "Pip",
        file_name: "08_counting_fireflies.rs",
        lesson: "while loops",
        template: include_str!("templates/08_counting_fireflies.rs"),
        intro: &[
            "Whoa — you're the one who fixed Rowan's lantern AND sorted out half the market? Wren said a rune-smith might come through. I'm Pip. I've got fireflies to catch and a jar that just won't fill itself.",
            "One at a time, though — they won't be rushed, and neither will my counting-rune, apparently, because right now it just... doesn't count at all.",
            "It needs a `while` loop — keep going as long as there's room in the jar, catch one, check again. Can you teach it that?",
        ],
        reminder: "The jar's still empty. A `while caught < capacity` loop, catching one firefly each time through, should do it.",
        success: &[
            "The jar fills, firefly by firefly, until it's glowing like a tiny lantern of its own. Pip presses their nose to the glass, thrilled.",
            "\"IT WORKS! Wren's just up the path if you want to see a REAL spell — they made a whole function, all by themselves. Well. Almost by themselves.\"",
        ],
        hints: &[
            "`while condition { ... }` keeps running for as long as `condition` stays true.",
            "Catch one each pass and check again: `while caught < capacity { caught += 1; }`.",
        ],
    },
    Quest {
        id: 9,
        zone: 1,
        title: "A Spell for Wren",
        npc: "Wren",
        file_name: "09_a_spell_for_wren.rs",
        lesson: "writing functions",
        template: include_str!("templates/09_a_spell_for_wren.rs"),
        intro: &[
            "Hi!! Pip said you'd be by — are you a real rune-smith? I'm going to be one too. I already know the BEST spell idea: a spell that makes every step count double. Twice the walking with the same feet!",
            "I tried to write it but... okay I don't actually know how to write a function yet. It needs a name, and it takes the paces in, and gives the doubled paces back out.",
            "Grandmother says a function is just a little machine with a door in and a door out. Can you build mine? Please please please?",
        ],
        reminder: "Did you build my spell yet?? It's called double_step! Paces go in, DOUBLE paces come out! (Grandmother says the arrow `->` shows what comes out.)",
        success: &[
            "Wren hops in a circle, counting: \"two, four, six, EIGHT — it works, it works!\" A nearby tree rustles, presumably applauding.",
            "\"When I'm big I'll write functions with a HUNDRED doors. Briar's just past the hollow, weaving baskets — she likes people who count carefully!\"",
        ],
        hints: &[
            "A function needs: `fn`, a name, its inputs in parentheses, and `->` for what comes out.",
            "The shape is: `fn double_step(paces: i32) -> i32 { ... }`",
            "The last line of a function (no semicolon!) is what it returns: `paces * 2`",
        ],
    },
    Quest {
        id: 10,
        zone: 1,
        title: "The Standard Baskets",
        npc: "Basket-weaver Briar",
        file_name: "10_the_standard_baskets.rs",
        lesson: "fixed-size arrays",
        template: include_str!("templates/10_the_standard_baskets.rs"),
        intro: &[
            "You'll be the one Wren's been chattering about. Good timing — every stall in the woods gets four baskets, always the same four sizes, always in the same order. Simple, if you keep your counting straight.",
            "Only my counting-rune doesn't. I ask it for the THIRD basket and it hands me the fourth, every single time. Off by one, and it won't say which one.",
            "Rust counts baskets starting from zero — the first basket is index `0`, not `1`. Find the third one properly, would you?",
        ],
        reminder: "Still reaching for the wrong basket. Remember: the first basket is index 0, so the third is index... not 3.",
        success: &[
            "The rune finally hands over the right basket — the seven-capacity one, snug in the middle of the row. Briar checks it twice, out of habit.",
            "\"There we are. Maren's just through the ferns if you're after more forest work — mind the glowing mushrooms, she's VERY particular about those.\"",
        ],
        hints: &[
            "Array indices start at zero: `basket[0]` is the first, `basket[1]` the second, and so on.",
            "The THIRD basket is at index 2, not 3.",
        ],
    },
    Quest {
        id: 11,
        zone: 1,
        title: "Mushrooms & Manners",
        npc: "Forager Maren",
        file_name: "11_mushrooms_and_manners.rs",
        lesson: "if / else",
        template: include_str!("templates/11_mushrooms_and_manners.rs"),
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
        id: 12,
        zone: 1,
        title: "The Echo Cave",
        npc: "Shepherd Ambrose",
        file_name: "12_the_echo_cave.rs",
        lesson: "loops & building strings",
        template: include_str!("templates/12_the_echo_cave.rs"),
        intro: &[
            "*yawn* ...oh. Hello. Don't mind me, I was resting my eyes in an alert and professional manner. The sheep are fine. Probably. The problem is the echo cave, south of the road.",
            "My sheep love the echo. If it echoes 'baa' back enough times they settle right down for the night. But shouting is *work*, friend, and I am a man of... measured effort.",
            "An echo-rune, that's the thing. Give it a word and a number, and it repeats the word that many times with a space between. 'baa baa baa'. Bliss.",
        ],
        reminder: "The echo-rune, friend. A word, repeated N times, single spaces in between. No trailing space — the echo is fussy about trailing spaces. *yawn*",
        success: &[
            "You cast the rune into the cave. 'baa baa baa' rolls warmly back, and across the meadow every sheep folds itself down into the grass like a little cloud landing.",
            "Ambrose is already horizontal. \"Masterful,\" he murmurs. \"Yew keeps the old hollow, past the fallen log... good with a hoard, that one. Tell them I said the sheep are fine.\" *snore*",
        ],
        hints: &[
            "A `for _ in 0..times` loop and a `String` you `push_str` onto will do nicely.",
            "The fiddly bit is the spaces: add a space *before* each word except the first, or trim at the end.",
            "One tidy pattern: collect into a Vec and `.join(\" \")`, or loop with `if !result.is_empty() { result.push(' '); }`",
        ],
    },
    Quest {
        id: 13,
        zone: 1,
        title: "The Winter Hollow",
        npc: "Hollow-keeper Yew",
        file_name: "13_the_winter_hollow.rs",
        lesson: "Vec<T> and .push()",
        template: include_str!("templates/13_the_winter_hollow.rs"),
        intro: &[
            "Ambrose sent you? Good, good — sit, if you like, the hollow's patient. I keep the winter stores here, and my counting-rune has a stubborn opinion: it wants to know how MANY acorns before I've finished gathering them.",
            "An array wants its size decided before it's born. My hoard doesn't work that way — some years three good gathering days, some years thirty. I need something that GROWS as the acorns come in.",
            "`Vec::new()` starts empty, and `.push()` adds one more, as many times as the season allows. Would you set that right?",
        ],
        reminder: "The hollow's still bare. Start with `Vec::new()`, then `.push()` each day's count in as it comes.",
        success: &[
            "The hoard fills in, one push at a time, until it's a proper winter's worth. Yew turns it over slowly, satisfied down to their roots.",
            "\"Good. That'll see us through. Sable's out past the ridgeline most evenings — looking for something they lost, if you've a moment to spare.\"",
        ],
        hints: &[
            "`Vec::new()` makes an empty, growable list; `.push(value)` adds one more to the end.",
            "Push each day's count in turn: `hoard.push(day1); hoard.push(day2); hoard.push(day3);`.",
        ],
    },
    Quest {
        id: 14,
        zone: 1,
        title: "The Lost Bell",
        npc: "Woodward Sable",
        file_name: "14_the_lost_bell.rs",
        lesson: "Option<T>",
        template: include_str!("templates/14_the_lost_bell.rs"),
        intro: &[
            "Yew sent you. Fair enough — I could use the help. There's an old bell I lost years back, and some evenings I find it, and some evenings I don't. My searching-rune doesn't know how to be honest about that.",
            "It keeps returning a number, even on the evenings I find nothing — which is worse than useless, it's a LIE. I need a rune that can say 'nothing today' and mean it.",
            "Rust has a word for exactly that shape of maybe: `Option<T>`. `Some(value)` when it's there, `None` when it isn't — right there in the type, impossible to forget.",
        ],
        reminder: "The rune's still lying on empty evenings. `Some(7)` if `found_today`, `None` otherwise — no number pretending to be something it isn't.",
        success: &[
            "The rune finally admits it, plain as anything: some evenings, nothing. Sable looks almost relieved to hear it said honestly.",
            "\"Huh. That's... better, actually. Thank you. The river's just past the ridge — Silverford, they call it. Whatever's past there, you've more than earned the crossing.\"",
        ],
        hints: &[
            "`Option<T>` is `Some(value)` or `None` — Rust's built-in way of saying 'maybe'.",
            "`if found_today { Some(7) } else { None }` is the whole shape of it.",
        ],
    },
    Quest {
        id: 15,
        zone: 2,
        title: "The Dock Ledger",
        npc: "Dockhand Fenn",
        file_name: "15_the_dock_ledger.rs",
        lesson: "ownership & .clone()",
        template: include_str!("templates/15_the_dock_ledger.rs"),
        intro: &[
            "New face! You'll want the ferry, most likely — everyone does. First, though, if you don't mind: every crate that comes through gets logged TWICE, once in, once out. Simple bookkeeping.",
            "Except my logging-rune only lets me write the manifest down once. Second time, it's just... gone. 'Moved', the rune says, like that explains anything.",
            "In Rust, a value's got exactly one owner, and handing it to a function moves it there for good. The plain fix: give the second logging its OWN copy, with `.clone()`.",
        ],
        reminder: "Still only one log gets written. `manifest` was moved into the first `log_manifest` call — clone it before that, so the original survives for the second.",
        success: &[
            "Both entries land in the ledger, word for word the same, and the crate finally has a proper paper trail. Fenn stamps it twice, for good measure.",
            "\"That'll do it. Cloning's the honest way, but I hear Wick down the landing has some cleverer trick — doesn't copy a thing, just... looks. Ask him.\"",
        ],
        hints: &[
            "Passing a `String` into a function *moves* it — the caller can't use it again afterward.",
            "`.clone()` makes an independent copy before the move: `log_manifest(manifest.clone())`.",
        ],
    },
    Quest {
        id: 16,
        zone: 2,
        title: "The Ferry Token",
        npc: "Ferryman Wick",
        file_name: "16_the_ferry_token.rs",
        lesson: "ownership & moves",
        template: include_str!("templates/16_the_ferry_token.rs"),
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
            "(`.clone()` also works — a copy for the rune to keep, same as Fenn showed you at the dock — but borrowing is the ferryman-approved way.)",
        ],
    },
    Quest {
        id: 17,
        zone: 2,
        title: "The Borrowed Rod",
        npc: "Fisher Juniper",
        file_name: "17_the_borrowed_rod.rs",
        lesson: "mutable references",
        template: include_str!("templates/17_the_borrowed_rod.rs"),
        intro: &[
            "Shhh — you'll scare the trout. There we go. Now: I lend my spare rod to half the riverbank, and I have one rule. Return it *better than you got it*. Sharpened hook, ideally.",
            "So I wrote a sharpening-rune. And it runs! It sharpens beautifully! ...a copy. It sharpens a copy, somewhere, and my actual rod stays dull as a Tuesday.",
            "The rune needs to work on MY rod. The one in my hands. Not some ghost-rod in rune-land.",
        ],
        reminder: "Still sharpening ghost-rods. The rune must borrow the real one — *mutably*, mind you. `&mut`, and a little star to reach through it.",
        success: &[
            "Two passes of the rune and the hook could split a hair lengthwise. Juniper tests it with her thumb and beams.",
            "\"THAT's a proper borrow. Lent, changed, returned — sharpness seven. Sil mends nets just downstream, if you're heading that way. Sharp eyes, that one.\"",
        ],
        hints: &[
            "To let a function change the caller's value, it must take `&mut i32` instead of `i32`.",
            "At the call site, lend it mutably: `sharpen(&mut sharpness);`",
            "Inside the function, reach through the reference with `*`: `*hook += 2;`",
        ],
    },
    Quest {
        id: 18,
        zone: 2,
        title: "The Net Log",
        npc: "Net-mender Sil",
        file_name: "18_the_net_log.rs",
        lesson: "slices",
        template: include_str!("templates/18_the_net_log.rs"),
        intro: &[
            "Juniper sent you my way? Kind of her. I keep a tide chart — but it only ever cares about the first three days of the week, the rest is noise for my purposes.",
            "My reading-rune borrows the WHOLE week just to look at three days of it, which feels like carrying the whole net home to check one knot.",
            "A slice, `&[T]`, borrows just a stretch of a list — `&week[..3]` for the first three, say — without owning any of it, and without dragging the rest along.",
        ],
        reminder: "Still borrowing the whole week. Narrow it: `&week[..3]` takes just the first three days.",
        success: &[
            "The rune reads exactly three days and not a splinter more. Sil checks it against the knots in the net and nods, satisfied.",
            "\"That's tidier. Morrow's just downstream, past the reeds — got a letter he's been puzzling over for days. Might be right up your alley.\"",
        ],
        hints: &[
            "A slice `&[T]` borrows part of a list without owning it or copying it.",
            "`&week[..3]` borrows just the first three entries; `&week[..]` borrows everything.",
        ],
    },
    Quest {
        id: 19,
        zone: 2,
        title: "A Message in a Bottle",
        npc: "Hermit Morrow",
        file_name: "19_a_message_in_a_bottle.rs",
        lesson: "String vs &str",
        template: include_str!("templates/19_a_message_in_a_bottle.rs"),
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
        id: 20,
        zone: 3,
        title: "The Lost Book",
        npc: "Archivist Elm",
        file_name: "20_the_lost_book.rs",
        lesson: "structs",
        template: include_str!("templates/20_the_lost_book.rs"),
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
        id: 21,
        zone: 3,
        title: "Waking the Golem",
        npc: "The Stone Golem",
        file_name: "21_the_stone_golem.rs",
        lesson: "impl & methods",
        template: include_str!("templates/21_the_stone_golem.rs"),
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
        id: 22,
        zone: 3,
        title: "The Sorting of Spellbooks",
        npc: "Sage Alderly",
        file_name: "22_the_sorting_of_spellbooks.rs",
        lesson: "enums & match",
        template: include_str!("templates/22_the_sorting_of_spellbooks.rs"),
        intro: &[
            "So. The Field Guide comes home at last — and carried by a rune-smith, no less. Fitting. Before I shelve it, one more matter: the returns cart. Fifty years of books, and my sorting-rune half-finished on my desk.",
            "Every spellbook belongs to a school: Ember, Tide, Gale, or Stone. Each school has its shelf — one through four — and its motto, which the books hum quietly when shelved correctly. It's charming. Slightly eerie. Mostly charming.",
            "A `match` handles it: every school, every case, nothing forgotten — the rune itself refuses to compile if you miss one. The Library finds that very reassuring. One more small matter waits after this, but you're nearly done.",
        ],
        reminder: "Two match-runes: shelf numbers (Ember 1, Tide 2, Gale 3, Stone 4) and mottos. The compiler will insist you cover every school — let it insist.",
        success: &[
            "The cart of spellbooks rises, sorts itself into four neat streams, and settles shelf by shelf. All through the Library, books begin to hum their mottos — warm hands, everything flows, lightly now, patience, patience.",
            "\"There,\" Alderly says, watching the last one settle. \"Every shelf true. One thing left, and it's not mine to fix — Faye's been muttering about a torn catalogue page. Down at the scribe's desk, if you'd indulge one more errand.\"",
        ],
        hints: &[
            "A match on an enum lists every variant: `match school { School::Ember => 1, School::Tide => 2, ... }`",
            "For the mottos, match again but return the `&'static str` for each school.",
            "No `_` catch-all needed — with all four variants listed, the compiler knows you've covered everything.",
        ],
    },
    Quest {
        id: 23,
        zone: 3,
        title: "The Missing Page",
        npc: "Scribe Faye",
        file_name: "23_the_missing_page.rs",
        lesson: "Result<T, E> and the ? operator",
        template: include_str!("templates/23_the_missing_page.rs"),
        intro: &[
            "Alderly sent you? Then you're nearly done, rune-smith — one last small thing. Some books in this library are missing pages here and there. The Field Guide isn't one of them, thankfully, but my cataloguing-rune doesn't know that yet — it just guesses, and guessing is no way to run a library.",
            "I need a rune that reads two pages and, if EITHER one doesn't exist, says so plainly instead of pretending. `find_page` already knows how to fail honestly — it hands back a `Result`, `Ok` or `Err`.",
            "The `?` mark does the asking for you: unwraps an `Ok`, or hands the `Err` straight back up the chain if something's missing. Use it on both lookups, and you're finished — truly finished, this time.",
        ],
        reminder: "The rune still can't add two Results together — because they're not numbers, not yet. `?` on each `find_page` call unwraps the page or ends the function early with the error.",
        success: &[
            "Both pages read true, page 58 and page 100, added up honest as anything. Faye closes the catalogue with a satisfied little snap.",
            "\"There. Every book accounted for, every page that exists, exists on paper AND in the record. You've done more for this library in an afternoon than most manage in a year, rune-smith. Go rest. You've more than earned it.\"",
        ],
        hints: &[
            "`Result<T, E>` is `Ok(value)` or `Err(reason)` — a function that can fail says so in its return type.",
            "`?` after a `Result`-returning call unwraps `Ok`, or returns the `Err` immediately from the whole function: `find_page(total_pages, first)?`.",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_ids_line_up_and_zones_only_advance() {
        let mut last_zone = 0;
        for (i, q) in QUESTS.iter().enumerate() {
            assert_eq!(q.id as usize, i + 1);
            assert!(
                q.zone >= last_zone,
                "quest {} is in zone {} but an earlier quest was already in zone {}",
                q.id,
                q.zone,
                last_zone
            );
            last_zone = q.zone;
            assert!(!q.hints.is_empty());
            assert!(q.file_name.ends_with(".rs"));
        }
    }
}
