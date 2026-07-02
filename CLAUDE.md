# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

**Rune & Road** — a cozy Ratatui TUI game that teaches Rust. The repo owner (Jani) is both the developer and the *player*: the game scaffolds exercise files into `quests/` which he solves in his own editor to learn Rust. Full solutions live in `tests/solutions/` — treat them as spoilers when he's asking about a quest he's playing, and give hints instead.

## Commands

```sh
cargo run                                        # play (needs a real TTY)
cargo test                                       # everything, incl. a simulated full playthrough (~4s)
cargo test --test journey                        # start-to-finish playthrough via real keystrokes
cargo test --test solve_through                  # compiles all 24 templates+solutions with real rustc
cargo test --test render                         # every screen at sizes 10x5 → 300x90
cargo test <name>                                # single unit test by name
cargo clippy --all-targets && cargo fmt          # keep both clean (they are)
cargo run --example snapshot -- <zone 0-3> <x> <y> <w> <h>   # render a map region as text, no TTY
cargo run --features gfx --bin rune-road-gfx    # play the graphical (Macroquad) edition in a window
cargo run --example gfx_snapshot -- <scene> [zone] [--pos x,y] [--tick n] [--out f.png]   # gfx screen → PNG, headless
```

The `snapshot` example is the way to "see" map edits without launching the game;
`gfx_snapshot` is the same idea for the graphical frontend (scenes: title, world,
dialogue, journal, casting, pass, fizzle, paused, epilogue, toast).

## Architecture

Lib + thin bin split (`src/lib.rs` + `src/main.rs`) exists so integration tests can drive the whole game black-box through `App::on_key` / `App::on_tick` — keep new behavior reachable through those two entry points.

**Quest pipeline** (the core loop, spans several files):
`content/quests.rs` holds the static `QUESTS: [Quest; 12]` array — all dialogue, hints, and templates (`include_str!` from `src/content/templates/*.rs`). Accepting a quest makes `checker::scaffold` copy the template to `./quests/` in the cwd (**never overwrites** — player work is sacred). Pressing `c` runs `checker::cast`: a background thread compiles the file standalone with `rustc --edition 2024 --test`, runs the test binary with a 10s kill-timeout (players write infinite loops), and reports an `Outcome` over an mpsc channel polled in `App::on_tick`.

**Template invariants**, enforced by `tests/solve_through.rs`:
- `src/content/templates/` files are intentionally broken (compile errors or `todo!()`); they are *not* cargo modules and are never compiled by the workspace build.
- Every template must FAIL untouched; its twin in `tests/solutions/` (same filename) must PASS.
- Template and solution must keep identical `#[test]` blocks — the tests *are* the quest's win condition.

**Quest ordering is strictly linear**: `App::active_quest()` = first id not in `completed`; quest `id` 1–12 maps to zone `(id-1)/3` (checked by a test). NPCs only offer their quest when it's the active one; zone gates open when the zone's three quests are done.

**World generation** (`world/map.rs` + `world/zones.rs`): maps are 240×70, built procedurally at startup by `MapBuilder` (scatter/road/river/stamp/clearing) — no map literals to edit; everything derives from the deterministic `hash2(x, y, seed)`, including animation phases and out-of-bounds scenery. Zone gates use `barrier()` (a full-height tree/cliff line with Gate tiles in the road) so they can't be walked around. If you move an NPC/sign/gate or reshape terrain, the BFS reachability tests in `world/zones.rs` and the spawn/standability tests in `app.rs` will catch dead ends — run `cargo test` after any map change.

**Rendering** (`ui/`): the camera (`world/camera.rs`) clamps to the map and, when the terminal exceeds the map, centers it while `Zone::tile` returns `Border` scenery for out-of-bounds coords — a hard requirement: ultrawide terminals must never see black bars. Tile appearance lives in one place, `tile_visual()` in `ui/overworld.rs` (glyph + fg + bg per tile, animated by tick). All colors pass through `ui::shade()` for the day/night cycle. Weather particles (`ui/effects.rs`) only replace glyph+fg, never bg, so they sit "in" the world.

**Screens** are one enum (`app::Screen`); input dispatch and all state transitions live in `app.rs`. Dialogue endings carry side effects via `DialogueKind` (Intro → scaffold+accept, Success → gate unlock / epilogue).

**Graphical frontend** (`src/gfx/` + `src/bin/rune_road_gfx.rs`, feature `gfx`): a second face on the same `App` — Zelda-style 16×16 sprites instead of glyphs. Everything renders CPU-side into a 480×270 RGBA framebuffer (`gfx/frame.rs`); the Macroquad shell only uploads that buffer and integer-scales it, so `examples/gfx_snapshot.rs` can dump pixel-identical PNGs headless (the way to "see" gfx changes). Tile appearance lives in `tile_sprites()` in `gfx/scene.rs`, the sprite twin of `tile_visual()` — keep the two in step. Sprites come from `assets/atlas.png`, baked by `tools/bake_atlas.py` from Kenney's CC0 packs (`assets/CREDITS.md`); rebake and re-check constants in `gfx/atlas.rs` if you touch it. Macroquad is an optional dep, feature-gated so `cargo test` and the TUI never compile it — don't leak it into the lib.

## Gotchas

- ratatui 0.30 re-exports crossterm as `ratatui::crossterm` — do **not** add a separate `crossterm` dependency (version mismatch).
- The game writes `quests/` and `save.json` into the **cwd** (both gitignored). Tests that touch them (`tests/journey.rs`) chdir into a temp dir; keep that test alone in its file since cwd is process-global.
- `Style::new().bold()/.italic()` are inherent in ratatui 0.30 — importing `Stylize` triggers unused-import warnings.
- Tone is part of the spec: cozy, gentle, no fail states. Player-facing text (fizzle messages, toasts, dialogue) should stay in voice — the compiler is "the politest grump", errors are fizzles, "no harm done."
