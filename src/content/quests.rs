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
            "Ah — a traveler, and carrying a rune-satchel no less. Welcome to Emberwick. You find us on festival day, which is fortunate or unfortunate, depending on how you feel about chores.",
            "Tonight is the Lantern Festival, but the great lantern has hung dark for years. The lighting-spell is written down in my kitchen — the cottage on the southwest corner of the square; do let yourself in — yet I always get the little mark at the end wrong. Macros, you see. They're particular.",
            "Would you take a look? Speak the old words exactly right, and the lantern will glow like a harvest moon.",
        ],
        reminder: "The spell is nearly right, I am sure of it. Something small and pointy is missing at the end of the word... a little mark of excitement, perhaps? It is written out in full on my kitchen table, should memory want refreshing.",
        success: &[
            "The lantern blooms with warm gold light, and the whole square goes 'oooh'. Rowan dabs an eye with his sleeve.",
            "\"There it is. Just as I remember it. You have a gift for the old words, traveler — the young keeper of the market stall could use a hand with her sign, if you are staying a while.\"",
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
        lesson: "shadowing (let, again)",
        template: include_str!("templates/02_the_market_sign.rs"),
        intro: &[
            "Oh! You're the one who lit Rowan's lantern. I heard the whole square cheer from here. I've got a much SMALLER problem, but it's driving me up the wall.",
            "Grandmother's counting-rune tallies the apples one chalk stroke at a time — one | per apple, bless it — but the sign out front wants a proper NUMBER, and the rune point-blank refuses to change kinds mid-thought.",
            "Grandmother swore by 'shadowing': bind the same name again with `let`, and it may carry a new value — even a whole new KIND of value. The strokes know their own count, if you ask with `.len()`. Can you fix it?",
        ],
        reminder: "Still chalk strokes where a number should be. You can't assign over `apples` — it isn't `mut`, and the kind is changing anyway. Bind it AGAIN with a fresh `let`.",
        success: &[
            "The chalk rearranges itself mid-air: 'tally, then a proper 21 apples and 7 pears on the stall today.' Tansy claps once, delighted.",
            "\"PERFECT. Oh — Poppy's ledger is being just as stubborn, if you've got another minute. She's right through that door.\"",
        ],
        hints: &[
            "Assigning `apples = ...` fails twice over: the binding isn't `mut`, and the new value isn't even text anymore. Shadowing sidesteps both — write `let` in front.",
            "`let apples = apples.len();` — the new `apples` is the count of the old one's chalk strokes. That one small `let` is the whole quest.",
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
        lesson: "booleans, comparisons & if/else",
        template: include_str!("templates/04_open_or_closed.rs"),
        intro: &[
            "Halt. ...Oh, it's you, the lantern-and-ledger one. Word travels fast in a village this size. I've got a gate-rune that's supposed to sort out who pays what — and right now it charges everybody full price. Even the children. ESPECIALLY the children, somehow.",
            "The rules are simple enough for a human: under twelve or a local, free through. Seventy or older, half toll — two coins. Everyone else, the full four. It's the rune that can't keep it straight.",
            "It wants a `bool` first — true or false, waived or not, built straight out of comparisons joined with `||` for 'or'. Then an `if`, an `else if`, an `else` — and in Rust the whole `if` is an *expression*, so it can sit right inside a `let` and BE the price.",
        ],
        reminder: "The gate-rune still charges everybody. `age < 12 || is_local` decides the waiver, and the price wants an `else if age >= 70` branch for the elders' half toll.",
        success: &[
            "The gate-rune hums once, waves a passing child through free, and quietly charges old Wilfred two coins instead of four. Wilfred is so pleased he pays three. Fitch actually smiles, which unsettles a nearby pigeon.",
            "\"Clean and honest, that. Every crosser, the right coins, no arguing. Old Hobb's toll-board could use the same treatment — he's just past the well, muttering at a sign that won't hold a number.\"",
        ],
        hints: &[
            "A `bool` is just `true` or `false` — a comparison like `age < 12` already produces one. Join two with `||` (or): `age < 12 || is_local`.",
            "`if` is an expression, so it can sit in a `let`: `let coins = if waived { 0 } else { 4 };`",
            "Chain a middle case with `else if`: `if waived { 0 } else if age >= 70 { 2 } else { 4 }`.",
        ],
    },
    Quest {
        id: 5,
        zone: 0,
        title: "The Toll Board",
        npc: "Toll-keeper Hobb",
        file_name: "05_the_toll_sign.rs",
        lesson: "const & integer arithmetic",
        template: include_str!("templates/05_the_toll_sign.rs"),
        intro: &[
            "Four coins a wagon, four slots on the board, every day — has been for thirty years and I intend for it to stay that way. But my board-rune keeps asking ME what the numbers are, like I might have changed my mind overnight.",
            "I haven't. I won't. I want them CARVED in, not chalked on — `const`, the old books call it, always in capitals so you can spot one at a glance. And once they're carved, the board should do the evening sums itself.",
            "Whole-number sums, mind: the day's takings split EVENLY across the four slots — `/` keeps the whole coins and drops the rest — and whatever's left over goes in the tea tin. There's a `%` mark that finds exactly that remainder. Oh, and read the little `//` notes on the scroll — they're for you, not the rune. The compiler walks right past them.",
        ],
        reminder: "The board still can't find TOLL_PRICE or BOARD_SLOTS. `const NAME: u32 = value;`, carved in at the top. Then `/` for the even split and `%` for the tea tin.",
        success: &[
            "The board resets itself with a satisfying click — four coins, four slots, the evening's takings split even and the tea tin honestly accounted. Hobb nods once, gruffly pleased.",
            "\"Now THAT'LL hold. Whole numbers dividing like whole numbers should — no halves, no pretending. Reed's over by the map table if you're looking for more work. And Bram's been on about his well again.\"",
        ],
        hints: &[
            "`const` declares a value that can never change, known before the program even runs: `const TOLL_PRICE: u32 = 4;` — the type is spelled out, the name in capitals.",
            "Integer division keeps only the whole part: `21 / 4` is `5`, not five-and-a-quarter.",
            "The `%` operator gives the remainder of that division: `21 % 4` is `1` — one coin for the tea tin.",
        ],
    },
    Quest {
        id: 6,
        zone: 0,
        title: "The Deep, Deep Well",
        npc: "Well-keeper Bram",
        file_name: "06_the_deep_well.rs",
        lesson: "writing functions",
        template: include_str!("templates/06_the_deep_well.rs"),
        intro: &[
            "Ah, the lantern-lighter! Settle a village argument, would you? How deep is this well? I drop a pebble and count heartbeats until the splash. Two heartbeats, usually. The old falling-stone rune knows the rest: five strides, times the heartbeats, times the heartbeats again.",
            "But my rune's gone wrong in a strange way — it does the sum, then keeps the answer to itself. A semicolon in the wrong spot, they tell me. In Rust the last line of a function, WITHOUT a semicolon, is what the function hands back; put a semicolon on it and the value is quietly set down instead.",
            "And while you've got the scroll open: the argument. Millbrook claims their well at eighteen strides. Write me a fresh judging-rune — a whole function, `fn`, name, what goes in, `->` for what comes out — that says true when a depth beats theirs.",
        ],
        reminder: "The rune still keeps the answer to itself — that trailing semicolon has to go. And the judging-rune wants writing whole: `fn deepest_in_the_valley(depth: u32) -> bool`, true past eighteen strides.",
        success: &[
            "\"TWENTY STRIDES!\" Bram bellows down the well, and the well, politely, echoes it back. \"Deepest in the valley! Millbrook can keep their eighteen!\" He looks enormously satisfied, and presses his old storm-lantern into your hands — 'for the dark places past the village', he says.",
            "\"So the last line of the function, no semicolon, is what comes out. A little machine with a door in and a door out. Sensible! Reed's up at the map table, if you haven't met her — pins on the village map, apparently very particular ones.\"",
        ],
        hints: &[
            "A function's body is a block, and the block's final *expression* — the last line with no semicolon — is its return value.",
            "`strides;` sets the value down and returns nothing. Remove the semicolon: `strides`",
            "A whole function, from nothing: `fn deepest_in_the_valley(depth: u32) -> bool { depth > 18 }`",
        ],
    },
    Quest {
        id: 7,
        zone: 0,
        title: "The Map Pins",
        npc: "Cartographer Reed",
        file_name: "07_the_map_pins.rs",
        lesson: "tuples, arrays & loops",
        template: include_str!("templates/07_the_map_pins.rs"),
        intro: &[
            "You're the one straightening out the village's little troubles, I hear. Good — my survey's a shambles, and it's the last thing between you and the east road. Two runes should do it.",
            "First: a landmark is two numbers that travel TOGETHER — a tuple, `(column, row)`. The old well wants pinning at column 12, row 4. Then the road east: four legs measured in a fixed row — an array, `[i32; 4]` — that a `for` loop should walk and sum. Get both right and the map's finally honest.",
        ],
        reminder: "The survey's still short: the well pin is the tuple `(12, 4)`, and the road wants a `for` loop summing its four legs.",
        success: &[
            "The map redraws itself — the well pin snapping to its mark, the road summing true. Reed studies it from three angles, finds nothing loose, and looks almost disappointed about that.",
            "\"There. A map I can trust. Yours, too, from here — the road east is clear, and the Whispering Woods are patient, but not THAT patient. Go on.\"",
        ],
        hints: &[
            "A tuple bundles values with parentheses: `(12, 4)`. Read them back by position — `pin.0`, `pin.1` — or by destructuring, `let (col, row) = pin;`.",
            "A `for` loop visits every element of an array: `for leg in legs { total += leg; }`. Start a `mut` total at 0 and add each leg.",
        ],
    },
    Quest {
        id: 8,
        zone: 1,
        title: "Counting Fireflies",
        npc: "Pip",
        file_name: "08_counting_fireflies.rs",
        lesson: "the String type",
        template: include_str!("templates/08_counting_fireflies.rs"),
        intro: &[
            "Whoa — you're the one who fixed Rowan's lantern AND sorted out half the market? Wren said a rune-smith might come through. I'm Pip. I catch fireflies, and every catch gets a tally-mark scratched onto the jar's label. That's the RULES. My rules, but still.",
            "Except my label-rune won't take the marks. The label it starts with is carved-in writing — a string literal, the kind that can never grow. Grandmother says there's a second kind of text in Rust: `String`, the *keeping* kind, that lives on the heap and grows as long as you need it to.",
            "`String::from` turns carved writing into the keeping kind, and `push_str` adds more on the end. Fix the label so the tallies stick? Oh — and Grandmother says when a String's scope ends, Rust sweeps it up all by itself. 'Drop', she calls it. No chores!",
        ],
        reminder: "The label still won't take a tally. It starts as a literal — the never-growing kind. `String::from(\"fireflies: \")` makes it the keeping kind, and then `push_str` works.",
        success: &[
            "The jar fills, firefly by firefly, and the label grows a neat little star for every one — the tallies finally STICK. Pip presses their nose to the glass, thrilled.",
            "\"IT WORKS! A label that grows with the catching! Wren's just up the path — they've got a spell of their OWN now, and a problem with it, and they will absolutely not stop talking about either.\"",
        ],
        hints: &[
            "A string literal like `\"fireflies: \"` is fixed forever; a `String` is owned, lives on the heap, and can grow.",
            "`String::from(\"fireflies: \")` makes the growable kind. Then `label.push_str(\"*\")` appends to it (the variable needs to be `mut`).",
            "When a `String`'s owner goes out of scope, Rust frees it automatically — that's `drop`, and it's why there's no tidying-up spell to cast.",
        ],
    },
    Quest {
        id: 9,
        zone: 1,
        title: "A Spell for Wren",
        npc: "Wren",
        file_name: "09_a_spell_for_wren.rs",
        lesson: "moves & .clone()",
        template: include_str!("templates/09_a_spell_for_wren.rs"),
        intro: &[
            "Hi!! Pip said you'd be by — are you a real rune-smith? I'm going to be one too! I already wrote my FIRST incantation: 'double step'. Twice the walking with the same feet! Well. It doesn't double anything yet. But it SPARKLES when you cast it, which is the important part.",
            "The problem is I want to cast it TWICE — once for each foot, obviously — and the casting-rune EATS it. I hand my incantation in the first time and it's just GONE. Grandmother says that's ownership: a String has exactly one owner, and handing it to a function MOVES it there for good.",
            "But she also says there's a spell for making an honest copy — `.clone()` — so the rune can eat the copy and I keep mine. Can you fix it? Please please please?",
        ],
        reminder: "Did you fix my spell?? The first cast still eats the incantation, so the second cast has nothing to say! Grandmother says: clone it into the FIRST cast, keep the real one for the second.",
        success: &[
            "TWO castings, TWO showers of sparkles, one incantation safe and sound in Wren's pocket. Wren hops in a circle. \"Double step! Double SPARKLE! It's even better than I planned!\"",
            "\"So moving is forever but cloning is copies. When I'm big I'll clone a WHOLE SPELLBOOK. Briar's just past the hollow, weaving baskets — she likes people who bring things back!\"",
        ],
        hints: &[
            "Passing a `String` into a function *moves* it — the caller can't use it again afterward. That's why the second cast fizzles.",
            "`.clone()` makes an independent copy, letters and all: `cast(spell.clone())` feeds the copy to the first cast, and the original survives for the second.",
        ],
    },
    Quest {
        id: 10,
        zone: 1,
        title: "The Standard Baskets",
        npc: "Basket-weaver Briar",
        file_name: "10_the_standard_baskets.rs",
        lesson: "ownership in & out of functions",
        template: include_str!("templates/10_the_standard_baskets.rs"),
        intro: &[
            "You'll be the one Wren's been chattering about. Good timing. Every basket that goes out to a stall comes past me first for a checking-stamp — and my stamping-rune has developed a HABIT. Baskets go in. Baskets do not come out.",
            "It's ownership, same as anything: hand the basket to the stamping-rune and the rune owns it. When the rune finishes and its scope closes, everything it still owns gets dropped. My basket! Dropped! Like mulch!",
            "The fix is manners: a function that takes a thing had better hand it BACK — returning a value moves ownership out to the caller, same as passing moved it in. Teach my rune to give as good as it gets, would you?",
        ],
        reminder: "Baskets still go in and don't come out. The stamping-rune must RETURN the stamped basket — `-> String`, stamped basket on the last line — and the caller must catch it.",
        success: &[
            "The rune stamps the basket and — miracle of manners — hands it back. Briar turns it over: stamped, whole, and hers again. She checks it twice, out of habit.",
            "\"In and back out. That's a rune I can lend things to. Maren's just through the ferns if you're after more forest work — mind the glowing mushrooms, she's VERY particular about those.\"",
        ],
        hints: &[
            "When a function takes a `String` parameter, ownership moves *in*; when the function ends, whatever it still owns is dropped.",
            "Returning moves ownership back *out*: give `stamp` a `-> String` and make the stamped basket its last expression (no semicolon).",
            "At the call site, catch what comes back: `let basket = stamp(basket);` — or just make the call itself the last expression.",
        ],
    },
    Quest {
        id: 11,
        zone: 1,
        title: "Mushrooms & Manners",
        npc: "Forager Maren",
        file_name: "11_mushrooms_and_manners.rs",
        lesson: "shared borrows (&T)",
        template: include_str!("templates/11_mushrooms_and_manners.rs"),
        intro: &[
            "Careful where you step — that patch is dinner. You have a deciding-rune in that satchel? Good. Mine has a manners problem. My rules are simple: if it GLOWS, never — no exceptions, I don't care how pretty. More than four spots, leave it. Otherwise, basket.",
            "The rune applies the rules fine. The trouble is it TAKES each mushroom to inspect it — ownership and all — and inspected mushrooms never come back. They're off in rune-land. My dinner! In rune-land!",
            "There's a politer way, and it's one small mark: `&`. A *borrow*. Asking to LOOK at a thing without keeping it. The rune reads the mushroom through the borrow, the mushroom stays mine. The forest respects people who ask properly.",
        ],
        reminder: "The rune still keeps every mushroom it inspects. It should take `&String` — a borrow — and the caller should lend with `&`. Looking, not keeping.",
        success: &[
            "Maren tests your rune against her whole basket, twice. Every verdict lands right, and every mushroom is still IN THE BASKET afterward. She nods slowly, which from Maren is a standing ovation.",
            "\"Borrowing. Asking first. If more runes had manners I'd lend them things more often. Ambrose is up in the meadow — fair warning, he'll be asleep.\"",
        ],
        hints: &[
            "`&String` is a shared borrow — the function may look at the mushroom but never owns it, so the caller keeps it.",
            "Two edits: the parameter becomes `mushroom: &String`, and the call site lends with `is_dinner(&find, 3)`.",
            "You can create as many shared borrows as you like at once — looking is free, that's the whole charm.",
        ],
    },
    Quest {
        id: 12,
        zone: 1,
        title: "The Echo Cave",
        npc: "Shepherd Ambrose",
        file_name: "12_the_echo_cave.rs",
        lesson: "mutable borrows (&mut T)",
        template: include_str!("templates/12_the_echo_cave.rs"),
        intro: &[
            "*yawn* ...oh. Hello. Don't mind me, I was resting my eyes in an alert and professional manner. The sheep are fine. Probably. The problem is the echo cave, south of the road. My sheep settle for the night on a good triple 'baa' — the call, plus two echoes on the end.",
            "So I wrote an echo-rune that borrows my call and adds an echo onto it. Borrows it POLITELY — one of those `&` marks, like Maren likes. And the cave refused! Apparently a shared borrow is only for LOOKING. Adding an echo *changes* the call, and changing needs a different kind of borrow.",
            "`&mut`, friend. The lending-of-the-pen. Only one pen out at a time, mind — the cave is strict about that, no lookers allowed while someone's writing. Fix the borrow at both ends and my flock sleeps. And therefore, so do I.",
        ],
        reminder: "The echo-rune still can't change the call through a `&` borrow. It wants `&mut String`, and the call site wants `&mut call`. One pen at a time. *yawn*",
        success: &[
            "You cast the mended rune into the cave. 'baa baa baa' rolls warmly back, and across the meadow every sheep folds itself down into the grass like a little cloud landing.",
            "Ambrose is already horizontal. \"Masterful. Lent the pen, wrote the echo, pen back in the pot... good manners all the way down. Yew keeps the old hollow, past the fallen log. Tell them I said the sheep are fine.\" *snore*",
        ],
        hints: &[
            "A shared borrow `&String` only permits looking. To *change* the borrowed value, the function must take `&mut String`.",
            "Both ends change: `fn add_echo(call: &mut String)` and, at the call site, `add_echo(&mut call);` (the variable itself must be `mut`).",
            "Only one mutable borrow can exist at a time — Rust's way of making sure nobody reads a sentence while it's still being written.",
        ],
    },
    Quest {
        id: 13,
        zone: 1,
        title: "The Winter Hollow",
        npc: "Hollow-keeper Yew",
        file_name: "13_the_winter_hollow.rs",
        lesson: "returning owned values",
        template: include_str!("templates/13_the_winter_hollow.rs"),
        intro: &[
            "Ambrose sent you? Good, good — sit, if you like, the hollow's patient. I keep the winter stores here, and every autumn the hoard gets a written label: how many acorns stand between us and spring. This year the labeling-rune has taken up... philosophy.",
            "It writes the label INSIDE itself, then tries to hand me a mere *borrow* of it — a pointed finger, not the label. But the moment the rune finishes, everything inside it is swept away. A finger pointing at swept-away writing! The compiler, quite rightly, refuses to allow such a thing to exist.",
            "The mending is honest work: don't hand out a borrow of what's about to vanish — hand out the THING. Return the owned `String` itself, and ownership walks out of the rune and into my hands, safe as acorns.",
        ],
        reminder: "The labeling-rune still tries to hand out `&String` — a borrow of writing that's swept away when the rune ends. Return the owned `String` instead: `-> String`, and hand out `label`, not `&label`.",
        success: &[
            "The rune writes the label and hands over the label itself — yours to keep, nothing swept away behind it. Yew hangs it on the hoard-post and reads it twice, satisfied down to their roots.",
            "\"Thirty-one acorns against the winter, and the words will still be there in spring. Good. That'll see us through. Sable's out past the ridgeline most evenings — looking for something they lost, if you've a moment to spare.\"",
        ],
        hints: &[
            "A function can never return a reference to something it created inside itself — the value is dropped when the function ends, and the reference would point at nothing.",
            "Return the owned value instead: change the return type from `&String` to `String`, and return `label` rather than `&label`.",
            "Handing back an owned value *moves* it out to the caller — nothing is copied, nothing dangles, everyone's happy.",
        ],
    },
    Quest {
        id: 14,
        zone: 1,
        title: "The Lost Bell",
        npc: "Woodward Sable",
        file_name: "14_the_lost_bell.rs",
        lesson: "slices & &str",
        template: include_str!("templates/14_the_lost_bell.rs"),
        intro: &[
            "Yew sent you. Fair enough — I could use the help. There's an old bell I lost years back, and I search for it evening by evening. Every ground I search gets one letter on my planning-strip: R for the ridge, H for the hollow, F for the ford, and so on down the week.",
            "Two problems, both in my planning-rune. One: an evening's walking covers exactly THREE grounds, but the rune plans the whole strip every night — it should borrow just a stretch of it. A *slice*, `&strip[..3]`, a window onto the first three letters. No copying, no taking, just a look at part.",
            "Two: the rune only accepts my strip when it's the keeping kind of text — `&String` — and turns its nose up at a plain written scrap. Take `&str` instead, the books say, and a rune accepts both kinds gracefully. Honest tools should be easy to hand things to.",
        ],
        reminder: "The planning-rune still bites off the whole strip — `&strip[..3]` takes just three grounds. And its parameter should be `&str`, not `&String`, so a plain scrap of writing works too.",
        success: &[
            "The rune reads exactly three grounds and not a letter more, and takes any scrap of writing you offer it. Sable folds the strip away, and for once the evening ahead looks like a walk instead of a campaign.",
            "\"Three grounds a night, honestly planned. No more pretending I can walk the whole wood by moonrise. The river's just past the ridge — Silverford, they call it. Whatever's past there, you've more than earned the crossing.\"",
        ],
        hints: &[
            "A string slice borrows part of a `String` without copying: `&strip[..3]` is a window onto the first three letters.",
            "A parameter of type `&str` accepts both a borrowed `String` *and* a string literal — it's the welcoming way to borrow text.",
            "`&strip[..]` would borrow the whole strip as a `&str`; `[..3]` narrows the window to the first three.",
        ],
    },
    Quest {
        id: 15,
        zone: 2,
        title: "The Dock Ledger",
        npc: "Dockhand Fenn",
        file_name: "15_the_dock_ledger.rs",
        lesson: "defining structs",
        template: include_str!("templates/15_the_dock_ledger.rs"),
        intro: &[
            "New face! You'll want the ferry, most likely — everyone does. First, though, if you don't mind: every crate that lands on this dock gets a record. What it is, what it weighs, whether the seal's unbroken. Three facts, ONE crate — and my ledger keeps them on three different pages.",
            "A label here, a weight there, a seal somewhere under a teacup. What I need is a bundle: one shape that holds all three facts and travels as one. The old books call it a `struct` — you name the shape, you name each field and its type, and from then on every record is cut to the same pattern.",
            "Define me a `Manifest` — label, weight, sealed — then fill one in for the crate that just landed, and the ledger-line reads each fact back out with a dot: `entry.label`, `entry.weight`. One crate, one bundle, no teacups.",
        ],
        reminder: "The ledger's still in pieces. Define the struct — `struct Manifest { label: String, weight: u32, sealed: bool }` — then build one for Crate 14: dry goods, 12 stone, seal unbroken.",
        success: &[
            "Label, weight and seal snap together into one record, and the ledger-line reads it back without a single page-turn. The crate finally has a proper paper trail. Fenn stamps it, satisfied.",
            "\"One crate, one bundle. That's bookkeeping. Wick down at the landing has paperwork of his own, if you're after the ferry — tokens, receipts, the works. Tell him Fenn's ledger balances.\"",
        ],
        hints: &[
            "A struct definition names the shape and every field's type: `struct Manifest { label: String, weight: u32, sealed: bool }`",
            "Build an instance by naming each field: `Manifest { label: String::from(\"Crate 14, dry goods\"), weight: 12, sealed: true }`",
            "Read fields back out with a dot: `entry.label`, `entry.weight`, `entry.sealed`.",
        ],
    },
    Quest {
        id: 16,
        zone: 2,
        title: "The Ferry Token",
        npc: "Ferryman Wick",
        file_name: "16_the_ferry_token.rs",
        lesson: "building struct instances",
        template: include_str!("templates/16_the_ferry_token.rs"),
        intro: &[
            "Crossing paperwork. Everyone's favorite. Rules of the Silverford: every crossing gets a token — a number, a holder, and a stamp once I've inspected it. The token's a struct, same trick Fenn uses for his crates. My trouble is in the ISSUING and the STAMPING.",
            "Issuing first: my issuing-rune takes the number and the holder in, and builds the token. The long way writes every field twice — `number: number`, `holder: holder` — and the long way makes my eyes cross. When the incoming names already MATCH the field names, you may write each name just once. Field init shorthand, the books call it.",
            "Then the stamp. A struct starts however you bind it — and I bound my token so nothing about it can change, stamp included. If any field of an instance is to change, the WHOLE instance must be `mut`. One little word at the `let`, and the stamp goes on.",
        ],
        reminder: "The token's still unstamped. The issuing-rune wants the shorthand — `Token { number, holder, stamped: false }` — and the crossing wants `let mut token`, or the stamp can't go on.",
        success: &[
            "Token No. 7, issued in one clean motion, stamped in another, and handed back across the rail. Wick files his copy of the receipt, which is to say, drops it in a tin. He actually smiles, which briefly startles a frog.",
            "\"Shorthand for the issuing, `mut` for the stamping. The river appreciates economy. Juniper's fishing upstream, by the way — she lends her spare rod to careful people, and you strike me as careful.\"",
        ],
        hints: &[
            "Field init shorthand: when a variable's name matches the field's name, write it once — `Token { number, holder, stamped: false }`.",
            "To assign to any field of a struct instance, the whole binding must be mutable: `let mut token = issue(7, ...);`",
            "Then the stamp is plain assignment: `token.stamped = true;`",
        ],
    },
    Quest {
        id: 17,
        zone: 2,
        title: "The Borrowed Rod",
        npc: "Fisher Juniper",
        file_name: "17_the_borrowed_rod.rs",
        lesson: "struct update syntax",
        template: include_str!("templates/17_the_borrowed_rod.rs"),
        intro: &[
            "Shhh — you'll scare the trout. There we go. Now: I lend my spare rod to half the riverbank, and I have one rule. Return it *better than you got it*. Sharpened hook, ideally. The rod's a struct these days — owner, reach, sharpness — Fenn's bundling habit is catching.",
            "So my returning-rune should build the RETURNED rod: same rod, one field better. And the draft I've got copies every field across by hand, line after line, and last week it copied the reach wrong and gave me a nine-pace rod back as a two-pace rod. TWO PACES.",
            "There's a tidier spell: struct update syntax. Name the fields that CHANGE, then say `..rod` — 'and the rest, from this one'. New sharpness, everything else carried over exactly. Two borrowers in a row means two sharpenings — my rule, applied twice, is a rod at seven.",
        ],
        reminder: "Still copying fields by hand, and still getting them wrong. The returned rod is `Rod { sharpness: rod.sharpness + 2, ..rod }` — the changed field named, the rest carried over.",
        success: &[
            "Two borrowers, two returns, and the rod comes home at sharpness seven with its reach intact — the hook could split a hair lengthwise. Juniper tests it with her thumb and beams.",
            "\"Returned better than you got it, twice over, and not a field miscopied. Keep the spare — you've the manners for it, and the reedy banks downriver are worth an afternoon. Sil mends nets just downstream, if you're heading that way.\"",
        ],
        hints: &[
            "Struct update syntax builds a new instance from an old one: name what changes, then `..old` for everything else — `Rod { sharpness: rod.sharpness + 2, ..rod }`.",
            "The `..rod` must come last, and it carries over every field you didn't name — reach and owner included.",
            "Two sharpenings chain naturally: `sharpened(sharpened(spare))` — 3, then 5, then 7.",
        ],
    },
    Quest {
        id: 18,
        zone: 2,
        title: "The Net Log",
        npc: "Net-mender Sil",
        file_name: "18_the_net_log.rs",
        lesson: "tuple structs",
        template: include_str!("templates/18_the_net_log.rs"),
        intro: &[
            "Juniper sent you my way? Kind of her. I keep the tide chart — and for my purposes it's three numbers, no more: the early week's counts, Monday, Tuesday, Wednesday. The rest of the week is noise, far as the nets care.",
            "Now, I COULD write a full struct with three solemn field names. But 'monday', 'tuesday', 'wednesday'? The POSITIONS already say it. For a bundle where order is the meaning, the books have a terser shape: a tuple struct. `struct EarlyWeek(u32, u32, u32);` — named as a whole, numbered inside.",
            "Define it, read this week's chart into one — two, four, six — and the tallying-rune reads the pieces by position: `.0`, `.1`, `.2`. Terse, tidy, and typed: an EarlyWeek is an EarlyWeek, not just any three numbers that happen to be standing together.",
        ],
        reminder: "The chart's still loose numbers. Define `struct EarlyWeek(u32, u32, u32);` and build this week's: `EarlyWeek(2, 4, 6)`. The rune reads them as `.0`, `.1`, `.2`.",
        success: &[
            "Three counts click into one named bundle, and the tally reads twelve, clean as a mended seam. Sil checks it against the knots in the net and nods, satisfied.",
            "\"An EarlyWeek that can't be mistaken for any other three numbers on the dock. That's typing I can trust my nets to. Morrow's just downstream, past the reeds — the river's been bringing him letters again, and he's been muttering about his records.\"",
        ],
        hints: &[
            "A tuple struct has a name but no field names: `struct EarlyWeek(u32, u32, u32);` — the semicolon on the end matters.",
            "Build one like a function call: `EarlyWeek(2, 4, 6)`.",
            "Its pieces read by position, like a tuple's: `days.0 + days.1 + days.2`.",
        ],
    },
    Quest {
        id: 19,
        zone: 2,
        title: "A Message in a Bottle",
        npc: "Hermit Morrow",
        file_name: "19_a_message_in_a_bottle.rs",
        lesson: "#[derive(Debug)] & {:?}",
        template: include_str!("templates/19_a_message_in_a_bottle.rs"),
        intro: &[
            "The river brought me a letter today. In two pieces, as usual — the river is enthusiastic about delivery, less so about condition. I mend them by hand, that part I've long since made peace with. It's the ARCHIVE that's defeated me.",
            "Every letter gets a record — who sent it, how many pieces it arrived in, what year — a struct, naturally. And I want my archive-rune to read a whole record aloud, every field at once, without me writing out the formatting for each one like a scribe with a quill cramp.",
            "The rune-marks `{:?}` do exactly that — print a struct whole — but only for types that OPT IN. One line above the struct: `#[derive(Debug)]`. An attribute, asking the compiler to write the printing-spell for you. It's very good at it. Add the line, and my archive finally speaks.",
        ],
        reminder: "The archive-rune still can't read a record aloud — the `Letter` struct hasn't opted in. One line above it: `#[derive(Debug)]`. Then `{:?}` (or `{:#?}`, the airy multi-line kind) does the rest.",
        success: &[
            "The archive-rune clears its throat and reads the whole record in one breath — sender, pieces, year, every field named. Morrow listens to it twice, then looks out at the water for a long moment.",
            "\"From M., as ever. Two pieces, mended, recorded. Now every letter the river brings gets remembered properly — thank you, rune-smith. The bridge east is Wick's business, but I suspect you've nearly earned it.\"",
        ],
        hints: &[
            "`{:?}` prints a value in debug form — but a struct must opt in by deriving the trait: put `#[derive(Debug)]` on the line above `struct Letter`.",
            "That attribute asks the compiler to write the whole printing implementation for you — no formatting code needed.",
            "`{:#?}` is the pretty variant: same information, spread over indented lines. Lovely for big records.",
        ],
    },
    Quest {
        id: 20,
        zone: 3,
        title: "The Lost Book",
        npc: "Archivist Elm",
        file_name: "20_the_lost_book.rs",
        lesson: "impl & &self methods",
        template: include_str!("templates/20_the_lost_book.rs"),
        intro: &[
            "Halt! ...at a leisurely pace. You carry a book. I can smell overdue ink from thirty yards. Before it comes home to the Library it must be processed, and the record itself — title, pages, years overdue — is the easy part. Any dockhand can bundle facts. No offense to dockhands.",
            "A LIBRARY record has *abilities*. It can answer for itself: is it overdue? What does its catalogue card say? In Rust, abilities that belong to a type are methods, and they live in an `impl` block — `impl Book { ... }` — each one taking `&self`: a polite borrow of the very record it's asked about.",
            "The Field Guide's record is filled in — 312 pages, fifty-eight years overdue, no judgement, some judgement — but its impl block stands EMPTY. Carve in the two methods, and call them with a dot: `book.is_overdue()`. The record answers. That's cataloguing.",
        ],
        reminder: "The impl block is still empty. Two methods, both borrowing with `&self`: `is_overdue` (true past zero years) and `catalogue_card` (title, then pages in brackets). Called with a dot: `book.is_overdue()`.",
        success: &[
            "The record-rune answers for itself — overdue, YES, card printed crisp — and seals with a satisfying THUMP of invisible ink. Elm inspects it from four angles and finds nothing to correct, which visibly disappoints him.",
            "\"Catalogued, and it catalogued ITSELF, which is the dignified way. Fifty-eight years... the late fee will be waived, this once. The Golem at the last bend handles admissions — it will need waking. It always needs waking.\"",
        ],
        hints: &[
            "Methods live in an `impl` block: `impl Book { fn is_overdue(&self) -> bool { self.years_overdue > 0 } }`",
            "`&self` is a shared borrow of the instance the method is called on; read its fields as `self.pages`, `self.title`.",
            "Call methods with a dot: `book.catalogue_card()` — e.g. `format!(\"{} ({} pages)\", self.title, self.pages)`.",
        ],
    },
    Quest {
        id: 21,
        zone: 3,
        title: "Waking the Golem",
        npc: "The Stone Golem",
        file_name: "21_the_stone_golem.rs",
        lesson: "methods with parameters",
        template: include_str!("templates/21_the_stone_golem.rs"),
        intro: &[
            "The golem is asleep. Standing up. Snoring gravel. A small brass plate on its chest reads: 'ADMISSIONS. All returns through the slot. THE SLOT DECIDES. — The Management.' The slot in question is a neat rectangular opening in the golem's chest, currently shut.",
            "Tucked behind the plate is the judging-rune, unfinished: a `Package` struct — width and height — and an impl block wanting one method. Not a lonely `&self` method, either: judging takes TWO packages, the parcel AND the slot, because a slot, the plate explains in smaller letters, IS A PACKAGE-SHAPED HOLE.",
            "So: `fits_through(&self, slot: &Package) -> bool` — a method with a parameter, borrowing a second instance to compare against. Strictly narrower and strictly shorter, or the slot stays shut. Finish the rune, and the golem can finally judge in its sleep. Which may be the only way it works at all.",
        ],
        reminder: "The judging-rune still can't compare parcel to slot. One method, one parameter: `fn fits_through(&self, slot: &Package) -> bool` — strictly narrower AND strictly shorter than the slot.",
        success: &[
            "The Field Guide slides through the slot with a whisper. Deep in the stone chest something goes *whirr*, then *bong*, and two pebble-eyes grind open. \"ADMISSIONS,\" the golem booms, delighted. \"THE SLOT HAS DECIDED. WELCOME. MIND THE STEP.\"",
            "It shakes your hand with alarming gentleness. \"A SLOT IS A PACKAGE-SHAPED HOLE. THE SAGE AWAITS AT THE DOOR. SHE HAS BEEN AWAITING FOR FIFTY-EIGHT YEARS. SHE WILL SAY IT DOES NOT MATTER. IT DOES, A LITTLE.\"",
        ],
        hints: &[
            "Methods can take parameters after `self`: `fn fits_through(&self, slot: &Package) -> bool` compares the instance it's called on against another one.",
            "Borrow the second instance (`&Package`) — judging shouldn't swallow the slot.",
            "Strictly means `<`, not `<=`: `self.width < slot.width && self.height < slot.height`.",
        ],
    },
    Quest {
        id: 22,
        zone: 3,
        title: "The Sorting of Spellbooks",
        npc: "Sage Alderly",
        file_name: "22_the_sorting_of_spellbooks.rs",
        lesson: "&mut self methods",
        template: include_str!("templates/22_the_sorting_of_spellbooks.rs"),
        intro: &[
            "So. The Field Guide comes home at last — and carried by a rune-smith, no less. Fitting. Before I shelve it, one more matter: the returns cart. Fifty years of homecomings pile up, and the cart keeps its own count — books waiting, books shelved. A struct with two tallies, and a half-finished rune on my desk.",
            "The cart already ANSWERS questions — `all_shelved` merely looks, a `&self` method, you've met the kind. But shelving a book must CHANGE the cart: one off the waiting pile, one onto the shelved count. A method that changes the very thing it's called on borrows it with the pen: `&mut self`.",
            "Same pen-rules as Ambrose's cave, mind — a `&mut self` method needs the instance itself bound `mut`, and the pen is only held for the length of the call. Carve in `shelve_one`, and the cart can finally count its own homecomings. The books hum when they're shelved right, you know. Charming. Slightly eerie. Mostly charming.",
        ],
        reminder: "The cart still can't shelve. One method wants carving: `fn shelve_one(&mut self)` — waiting down by one, shelved up by one. And the cart at the call site must be `let mut`.",
        success: &[
            "Three books, three turns of the rune, and the cart counts itself down to empty as the shelved pile rises. All through the Library, books begin to hum — warm hands, everything flows, lightly now, patience, patience.",
            "\"Every homecoming counted, by the cart itself. `&mut self` — the pen, held just long enough, put back just soon enough.\" Alderly watches the last book settle. \"One thing left, and it's not mine to fix — Faye's been muttering about a torn catalogue page. Down at the scribe's desk, if you'd indulge one more errand.\"",
        ],
        hints: &[
            "A method that changes its own struct takes `&mut self`: `fn shelve_one(&mut self) { self.waiting -= 1; self.shelved += 1; }`",
            "Compare with `all_shelved(&self)`, which only looks — read-methods borrow, change-methods borrow mutably.",
            "The caller's binding must allow it: `let mut cart = ...;` before any `cart.shelve_one()`.",
        ],
    },
    Quest {
        id: 23,
        zone: 3,
        title: "The Missing Page",
        npc: "Scribe Faye",
        file_name: "23_the_missing_page.rs",
        lesson: "associated functions",
        template: include_str!("templates/23_the_missing_page.rs"),
        intro: &[
            "Alderly sent you? Then you're nearly done, rune-smith — one last small thing. The great catalogue is missing a page. Page fifty-eight, torn out who-knows-when. I mean to rewrite it fresh tonight, and my page-rune is one ability short of ready.",
            "A fresh page isn't made by asking some EXISTING page to copy itself — there's no `self` to ask. It's made from nothing, by the TYPE: an *associated function*. You've been calling them all along — `String::from` is one. Two colons, straight on the type's name: `CataloguePage::new(58)`.",
            "Inside the impl block it's a function without `self`, returning `Self` — the type's own name for itself. Write `new`, and `record_entry` beside it — that one's a `&mut self` method, you know those now — and the page fills, entry by entry, until the catalogue is whole. Truly whole, this time.",
        ],
        reminder: "The catalogue still has its gap. `new` is an associated function — no `self`, returns `Self`, called as `CataloguePage::new(58)` — and `record_entry` is a plain `&mut self` method beside it.",
        success: &[
            "A fresh page fifty-eight, numbered from nothing, filled entry by entry until it sits flush in the great catalogue as if it had never been gone. Faye closes the book with a satisfied little snap.",
            "\"Made by the type, filled by its methods — structs and their abilities, start to finish. That's every page accounted for, and every trick in my book now in yours. You've done more for this library in an afternoon than most manage in a year, rune-smith. Go rest. You've more than earned it.\"",
        ],
        hints: &[
            "An associated function lives in the impl block but takes no `self`; it's called on the type with `::` — `CataloguePage::new(58)`, just like `String::from`.",
            "`Self` inside an impl block means the type itself: `fn new(number: u32) -> Self { Self { number, entries: 0 } }` (field init shorthand welcome).",
            "`record_entry` is an ordinary `&mut self` method: `self.entries += 1;` — the capstone is just the pieces you already know, standing together.",
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
