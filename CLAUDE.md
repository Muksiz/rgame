# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

**Rune & Road** — a cozy game that teaches Rust, growing step by step into an old-Pokémon/Zelda-shaped world (see `ROADMAP.md` for what's done and what's next). It renders as 16×16 sprites on a framebuffer (480×270 by default, sized up to the window's aspect so ultrawide screens fill edge-to-edge with no black bars), integer-scaled into a Macroquad window. The original Ratatui TUI frontend has been retired; the graphical build *is* the game. The repo owner (Jani) is both the developer and the *player*: the game scaffolds exercise files into `quests/` which he solves in his own editor to learn Rust. Full solutions live in `tests/solutions/` — treat them as spoilers when he's asking about a quest he's playing, and give hints instead.

Beyond the 12 main quests, the world has optional layers: enterable interiors behind every door, quest keepsakes that gate the world (the storm-lantern, the fishing rod), wild rune encounters in the tall grass feeding the Grimoire collection, flag-driven side quests (Granny Sorrel's moon-mint, Old Nettle hidden in the deep woods, the locked chest in the storehouse cellar), eight hidden runestones to collect, and a Great Library whose shelves hold real books about Rust.

## Commands

```sh
cargo run                                        # play (opens the game window)
cargo test                                       # everything, incl. a simulated full playthrough
cargo test --test journey                        # start-to-finish playthrough via real key input
cargo test --test solve_through                  # compiles all 24 templates+solutions with real rustc
cargo test --test render                         # every screen through the real renderer, headless
cargo test <name>                                # single unit test by name
cargo clippy --all-targets && cargo fmt          # keep both clean (they are)
cargo run --example snapshot -- <scene> [zone] [--pos x,y] [--tick n] [--day t] [--size WxH] [--out f.png]   # any screen → PNG, no window
```

The `snapshot` example is the way to "see" anything — map edits, new screens —
without launching the game: it renders the same framebuffer the window shows,
byte for byte, to a PNG (scenes: title, charselect, world, dialogue, journal,
casting, pass, fizzle, paused, resting, banner, epilogue, toast, encounter,
caught, grimoire, book; `world` takes a zone index: 0-3 overworld, 4+
interiors). `--day t` sets the day/night clock position (0 = dawn; outdoor
scenes only); `--size WxH` renders at a non-native framebuffer size to preview
ultrawide.

## Architecture

Lib + thin bin split: `src/main.rs` is only the Macroquad shell (input → `app::Key`, a 50ms tick, a window-sized `Frame`, framebuffer upload). Everything else lives in the lib so integration tests can drive the whole game black-box through `App::on_key` / `App::on_tick` — keep new behavior reachable through those two entry points. Input is the game's own `app::Key` enum; nothing outside `main.rs` knows about macroquad key codes. **Letter keys come from the OS text stream** (`get_char_pressed`), so the game follows the player's keyboard layout (Dvorak, AZERTY, …); only non-character keys (arrows, Enter, Esc, Space, Backspace, PageUp/Down) go through the physical `KEYMAP`. Movement is arrows + vim `H J K L` (no WASD); `e`/Enter/Space are one unified confirm. The `Frame` (`gfx/frame.rs`) carries its own `w`/`h`; `main.rs` resizes it each frame and `gfx/scene.rs` lays everything out from `fb.width()`/`fb.height()`, never fixed constants. A new journey runs through `Screen::CharSelect` (pick a look from `atlas::PLAYABLE` + type a name; both persist in `SaveData`).

**Quest pipeline** (the core loop, spans several files):
`content/quests.rs` holds the static `QUESTS: [Quest; 12]` array — all dialogue, hints, and templates (`include_str!` from `src/content/templates/*.rs`). Accepting a quest makes `checker::scaffold` copy the template to `./quests/` in the cwd (**never overwrites** — player work is sacred). Pressing `c` runs `checker::cast`: a background thread compiles the file standalone with `rustc --edition 2024 --test`, runs the test binary with a 10s kill-timeout (players write infinite loops), and reports an `Outcome` over an mpsc channel polled in `App::on_tick`.

**Template invariants**, enforced by `tests/solve_through.rs`:
- `src/content/templates/` files are intentionally broken (compile errors or `todo!()`); they are *not* cargo modules and are never compiled by the workspace build.
- Every template must FAIL untouched; its twin in `tests/solutions/` (same filename) must PASS.
- Template and solution must keep identical `#[test]` blocks — the tests *are* the quest's win condition.

**Quest ordering is strictly linear**: `App::active_quest()` = first id not in `completed`; quest `id` 1–12 maps to zone `(id-1)/3` (checked by a test). NPCs only offer their quest when it's the active one; zone gates open when the zone's three quests are done.

**Items & wild runes** (the optional layer): `content/items.rs` — keepsakes granted by quest success; *owning one is derived from `completed`*, never stored, so old saves stay valid. They gate things: dark places (`zones::needs_light` — the Echo Cave and the storehouse cellar) need the storm-lantern (quest 3), reedy banks become fishing spots with the rod (quest 8). `content/wilds.rs` — 16 wild runes living in `Tile::TallGrass`; each step rolls `hash2(pos, seed ^ grass_steps)` for an encounter (`Screen::Encounter`, a 3-option Rust question; fleeing is always free, wrong answers just fizzle). Caught runes fill the grimoire (`Screen::Grimoire`, key `g`, persisted in `save.json` behind `#[serde(default)]`). The Great Library's shelves are readable: `content/books.rs` holds real books about Rust (guides, features, history); pressing `e` at a `Tile::Bookshelf` opens the book assigned to that shelf in row-major shelf order (`DialogueKind::Book`) — keep facts in them accurate.

**Side quests, flags & runestones** (the other optional layer): world state that isn't quest completion lives in `App::flags` / `SaveData::flags` (`BTreeSet<String>`, `#[serde(default)]`), set at dialogue close via `DialogueKind::Side(Option<flag>)` or directly on interaction — nothing derivable is ever stored, and side actions deliberately don't autosave. `content/sides.rs` owns the flag names and all side dialogue (`sides::talk(npc, flags)`, checked first in `npc_dialogue`): Granny Sorrel wants moon-mint (a `Tile::Herb` off the cave path), Old Nettle sits at the end of an unsignposted trail of clearings in the deep woods and gives the key to the cellar chest (`Tile::Chest`). `content/stones.rs` holds the eight runestones: seven `Tile::Runestone` spots catalogued in `zones::runestone_spots()` (they glimmer until their `runestone.<id>` flag is set), the eighth inside the chest. Signs are readable on *any* tile, so interiors can have notes on tables/crates (speaker "A note"). Invariant tests pin all of it: spots ↔ tiles agree, every secret is BFS-reachable, stone count = spots + chest.

**World generation** (`world/map.rs` + `world/zones.rs`): maps are 240×70, built procedurally at startup by `MapBuilder` (scatter/road/river/stamp/clearing) — no map literals to edit; everything derives from the deterministic `hash2(x, y, seed)`, including animation phases and out-of-bounds scenery. Zone gates use `barrier()` (a full-height tree/cliff line with Gate tiles in the road) so they can't be walked around. If you move an NPC/sign/gate or reshape terrain, the BFS reachability tests in `world/zones.rs` and the spawn/standability tests in `app.rs` will catch dead ends — run `cargo test` after any map change.

**Interiors & warps**: zones 0–3 are the overworld; zones 4+ are rooms behind doors (houses, the Echo Cave, the Great Library) — small stamped rooms floating in `Tile::Void`, built by `room()` in `world/zones.rs`. Every `Tile::Door` (and the cave mouth) carries a `Warp { at, to_zone, to_pos }`; `App::try_move` warps on step-on. Invariant tests: every Door tile has a warp, every warp lands on ground reachable from the destination zone's spawn, and every way in has a way back. **Weather is static per zone** (`Zone::weather: Option<Weather>` — interiors have `None`): petals over Emberwick, fireflies in the Woods, rain over Silverford, mist on the Hearthspire road. **Time of day now flows**: `App::day_ticks` runs a real-time clock (`DayPhase` morning/midday/evening/night, lengths in `app.rs`), and `App::daylight()` returns the shared sky brightness (`app::sky_daylight`) for outdoor zones, lightly tinted by each zone's `Zone::daylight` canopy factor. Interiors ignore the clock and keep their own fixed `Zone::daylight`. Campfires (`Tile::Campfire`, `e`) open `Screen::Resting`, tell a `content::lore` snippet, and jump the clock (day→night, night→morning); at night outdoor NPCs are drawn asleep.

**Rendering** (`src/gfx/`): everything draws CPU-side into a 480×270 RGBA framebuffer (`gfx/frame.rs`); the shell only uploads and integer-scales it, which is what lets `examples/snapshot.rs` dump pixel-identical PNGs headless and lets `tests/render.rs` exercise every screen without a window. The camera (`world/camera.rs`) follows the player and clamps to the map. Tile appearance lives in one place: `tile_sprites()` in `gfx/scene.rs` (base sprite + transparent overlay per tile, animated by tick), with edge-aware detail passes (shoreline, path/floor rims, rugs, roof shingles) beside it. All colors pass through `gfx::shade()` with the zone's fixed `App::daylight()`. Weather is pixel particles drawn over the world but under the HUD. Text is the `font8x8`-based bitmap font in `gfx/font.rs`. Sprites come from `assets/atlas.png`, baked by `tools/bake_atlas.py` from Kenney's CC0 packs (terrain/props, sheets in `assets/kenney/`) and the CC0 Ninja Adventure pack (the whole cast, strips in `assets/ninja_adventure/<Character>/` — see `CAST` in the bake script and `assets/CREDITS.md`); to add sprites, append cells at the *end* of the bake list (existing ids must not shift), run `python3 tools/bake_atlas.py assets/kenney/roguelikeSheet_transparent.png assets/kenney/roguelikeChar_transparent.png` (needs Pillow), and sync the printed constants into `gfx/atlas.rs`. Characters are animated: each cast member has four idle facings (`atlas::CAST`, NPCs turn to face the adjacent player and sway), the player has stride frames (`atlas::PLAYER_WALK`, driven by `App::facing`/`App::walked_at` — cosmetic, never saved). Hand-pixeled sprites (critters, Ferris, the bookshelf) are `from_art` text grids in the bake script — match that style for new ones.

**Screens** are one enum (`app::Screen`); input dispatch and all state transitions live in `app.rs`, drawing in `gfx/scene.rs`. Dialogue endings carry side effects via `DialogueKind` (Intro → scaffold+accept, Success → gate unlock / epilogue / keepsake handover; Book → a Library shelf reading itself, with a bookshelf portrait).

## Gotchas

- The game writes `quests/` and `save.json` into the **cwd** (both gitignored). Tests that touch them (`tests/journey.rs`) chdir into a temp dir; keep that test alone in its file since cwd is process-global.
- **Save compatibility is a promise**: new `SaveData` fields go behind `#[serde(default)]`, and state that can be derived (item ownership comes from `completed`) is derived, never stored — an old `save.json` must always load. There's a test for it in `save.rs`.
- **Autosave only at milestones** (quest pass, gate crossing, save-and-quit). Frequent actions — warping through doors, catching runes, fishing — deliberately don't write to disk, both for feel and so unit tests that exercise them don't litter the repo cwd with `save.json`.
- Macroquad is a plain dependency compiled by every build including `cargo test` (only the first build pays for it). Keep it confined to `src/main.rs` — the lib and tests must stay window-free so everything runs headless.
- Tone is part of the spec: cozy, gentle, no fail states. Player-facing text (fizzle messages, toasts, dialogue) should stay in voice — the compiler is "the politest grump", errors are fizzles, "no harm done."
