# ✦ Rune & Road ✦

A cosy little terminal adventure that teaches the Rust programming language.

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

Keep your editor (Zed, or anything) open next to the terminal. When you accept
a quest, its exercise file appears in `quests/` — open it there, fill in the
code, then come back to the game and press `c` to cast.

A terminal with truecolor support and a decent size is recommended. The maps
are big (240×70): ultrawide terminals get more world, not black bars.

### Controls

| Key | Action |
|---|---|
| arrows / WASD / HJKL | walk |
| `e` / Enter | talk, read signs |
| `c` | cast — compile & test the current quest file |
| `q` | journal (current quest, file path, hints so far) |
| `f` | ask Ferris for a hint |
| Esc | rest (save & quit menu) |

### The road

1. **Emberwick Village** — `println!`, variables & `mut`, numeric types
2. **Whispering Woods** — functions, `if`/`else`, loops & Strings
3. **Silverford Riverlands** — ownership, `&mut` borrows, `String` vs `&str`
4. **Hearthspire Approach** — structs, `impl` methods, enums & `match`

Progress autosaves to `save.json`. Your quest files are never overwritten by
the game — your work is sacred.

## Development

```sh
cargo test                    # includes a full simulated playthrough
cargo test --test journey     # just the start-to-finish playthrough
cargo run --example snapshot -- 0 86 33 200 55   # render a map area as text
```

`tests/solutions/` holds reference solutions (spoilers!) used by the
solve-through test, which proves every template fails untouched and every
quest is completable.

Built with [Ratatui](https://ratatui.rs). Exercise checking is rustlings-style:
each quest file is compiled standalone with `rustc --edition 2024 --test`.
