# Rune & Road — Roadmap

The long walk from "cozy quest corridor" to a full-fledged game in the spirit
of old Pokémon and Zelda. Each step multiplies the value of the world that
already exists; polish comes last because the tone is already carrying a lot.

Ordering: **interiors → items & gates → grass encounters → side content →
NPC life → audio & polish.**

---

## 1. Interiors and warps ✅

*Done.* Every door leads somewhere: the five Emberwick houses (Poppy's
Bakery, Granny Sorrel's and Tilly's cottages, Alder's workshop, the old
storehouse), the Echo Cave behind its rocky mouth, and the Great Library —
so the epilogue's destination is a real place. Interiors are ordinary
`Zone`s (indices 4+): a furnished room stamped into a map of `Tile::Void`,
entered via `Warp { at, to_zone, to_pos }` on each `Door` tile.

Also done alongside it: **time of day and weather are static per location**
(`Zone::daylight`, `Zone::weather: Option<Weather>`). Emberwick keeps its
petal-lit morning, the Woods their firefly dusk, Silverford its rain,
Hearthspire its misty night, interiors their steady lamplight. There is no
ticking day/night clock anymore.

Invariants (tested): every Door tile has a warp; every warp lands on ground
reachable from the destination zone's spawn; every way in has a way back.

Possible garnish later: a fade-to-black on transit (trivial in
`gfx/frame.rs`, `ui::shade`-to-black in the TUI).

## 2. Items, inventory, and ability gates

The quests already name objects that exist only in prose — the festival
lantern, the ferry token, the borrowed rod. Make them real:

- An `Item` enum and `inventory: BTreeSet<Item>` on `App` (and in
  `SaveData`), plus a journal tab or `Screen::Inventory`.
- **Zelda-style traversal gating**: the lantern lets you enter dark caves
  (render undiscovered tiles dimmed until then), the rod enables fishing
  spots, the ferry token crosses the big water. Each Rust concept learned =
  a traversal ability earned; backtracking becomes rewarding.
- NPCs hand the item over in their `DialogueKind::Success` ending — the
  side-effect channel already exists.

## 3. Tall-grass encounters — the Pokémon battle analog

`Tile::TallGrass` is everywhere and does nothing. Walking through it should
(deterministically, via `hash2`, so playthroughs stay testable) trigger a
**wild rune encounter**: a small battle screen whose "moves" are quick Rust
questions or a three-line fix — a featherweight cousin of `checker::cast`.

- Cozy, no-fail: fleeing is always free, a wrong answer just fizzles.
- Caught runes go in a **runedex** — a collection screen is half of
  Pokémon's pull, and it doubles as spaced repetition for the curriculum.
- Keep it a `Screen` variant reachable through `on_key`/`on_tick` so
  `tests/journey.rs` can play through encounters.

## 4. Side quests, secrets, and collectibles

`active_quest()` stays strictly linear for the main road, but old Pokémon
always had optional stuff off the critical path:

- A parallel set of side quests (a hidden NPC in the deep woods, a locked
  chest, a favor for Granny Sorrel) — needs a general
  `flags: BTreeSet<Flag>` on `App`/`SaveData` for world state that isn't
  quest completion.
- Collectibles: e.g. eight hidden runestones with a counter in the journal;
  a bush that hides a passage once you've learned the right rune.
- The interiors are ready-made stages for this — a note on a table, a
  cellar door in the storehouse.

## 5. NPC life and dialogue depth

- A one-tile wander for NPCs (the critters already know how), facing the
  player when spoken to.
- Dialogue that varies by flags ("heard you fixed the well!") — the `idle`
  arrays already hold multiple lines; today only the first is used.
- A tiny cutscene primitive (a scripted move/say/wait sequence driven from
  `on_tick`) so quest payoffs feel staged instead of instant.

## 6. Presentation polish

- **Audio** (gfx build only, feature-gated so `cargo test` and the TUI
  never touch it): a CC0 chiptune loop per zone, SFX for cast/pass/fizzle.
- Zone-entry banners ("~ Whispering Woods ~" sliding in), a walk cycle and
  grass-rustle particles in gfx.
- An options screen — text speed first, the most Pokémon setting there is.
- Save slots, or at least a "save & quit" confirmation.

---

## Engineering ground rules (hold these while doing any of the above)

- Every new screen is a `Screen` variant reachable through `App::on_key` /
  `App::on_tick` — the black-box tests must be able to play it.
- All randomness derives from `hash2(x, y, seed)` — playthroughs stay
  deterministic and testable.
- `tile_visual()` (TUI) and `tile_sprites()` (gfx) move in lockstep when
  tiles are added.
- Extend the render matrix (`tests/render.rs`) and the world invariant
  tests (`world/zones.rs`) with every new screen, zone, or warp.
- Tone is spec: cozy, gentle, no fail states, the compiler is the politest
  grump.
