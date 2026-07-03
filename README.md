# ✦ Rune & Road ✦

A cosy little adventure that teaches the Rust programming language, rendered
in 16×16 pixels.

You are an apprentice rune-smith returning an overdue spellbook to the Great
Library at Hearthspire. Magic in this world is written as runes — which happen
to be Rust code. Twelve travelers along the road need one well-written rune
each; you write them in your own editor, and cast them in the game.

No timers. No death. No fail states. The borrow checker is a grumpy but kind
house spirit, and a failed compile is just "the rune fizzles — no harm done."

## Playing

```sh
cargo run
```

Keep your editor (Zed, or anything) open next to the game window. When you
accept a quest, its exercise file appears in `quests/` — open it there, fill
in the code, then come back to the game and press `c` to cast.

### Controls

| Key | Action |
|---|---|
| arrows / `H` `J` `K` `L` (vim) | walk (hold to keep walking) |
| `e` / Enter / Space | talk, advance dialogue, choose in menus, read signs & books, fish, rest at a campfire |
| `c` | cast — compile & test the current quest file |
| `q` | journal (current quest, file path, hints, satchel) |
| `g` | grimoire — wild runes caught in the tall grass |
| `f` | ask Ferris for a hint |
| Esc | rest menu (text speed, save & quit) |

Letter keys are read from the OS text stream, so the game follows your
keyboard layout — Dvorak, AZERTY, and friends all just work. The window
fills the whole screen with no black bars, ultrawide and superultrawide
included.

### The road

1. **Emberwick Village** — `println!`, variables & `mut`, numeric types
2. **Whispering Woods** — functions, `if`/`else`, loops & Strings
3. **Silverford Riverlands** — ownership, `&mut` borrows, `String` vs `&str`
4. **Hearthspire Approach** — structs, `impl` methods, enums & `match`

Before you set out you choose your traveller — a look and a name of your own.
Then the days turn: morning, midday, evening, and a starlit night roll by in
real time, and the whole world dims and brightens with them. Scattered
campfires let you rest — press `e` to doze off with a scrap of Rust lore and
wake at the next turn of the day; at night the folk of the world are fast
asleep.

And off the road: every house door opens, the tall grass hides wild runes
with quick questions for your grimoire (each asked only until you answer it
true), quests leave keepsakes that unlock corners of the world (a lantern for
the dark cave, a rod for the riverbanks), and the Great Library — three
sunlit halls of stacks, a showcase gallery, and shelves of real books about
Rust — waits at the end of the road. Each place keeps its own weather:
petals over Emberwick, fireflies in the Woods, rain over the river, mist on
the mountain road.

Progress autosaves to `save.json`. Your quest files are never overwritten by
the game — your work is sacred.

## Development

```sh
cargo test                    # includes a full simulated playthrough
cargo test --test journey     # just the start-to-finish playthrough
cargo run --example snapshot -- world 0 --out shot.png   # any screen → PNG, no window
```

`tests/solutions/` holds reference solutions (spoilers!) used by the
solve-through test, which proves every template fails untouched and every
quest is completable.

Built with [Macroquad](https://macroquad.rs) — the game renders CPU-side into
a framebuffer (480×270 by default, widened to match the window so ultrawide
screens fill edge-to-edge) that the window integer-scales, so every screen can
also be rendered headless. Sprites are baked from
[Kenney](https://kenney.nl)'s CC0 packs (see `assets/CREDITS.md`). Exercise
checking is rustlings-style: each quest file is compiled standalone with
`rustc --edition 2024 --test`.
