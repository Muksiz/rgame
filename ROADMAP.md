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

## 2. Items, inventory, and ability gates ✅

*Done.* Keepsakes are real now (`content/items.rs`): Bram's storm-lantern
(quest 3) and Juniper's spare rod (quest 8), handed over in the success
dialogue and listed in the journal's satchel. Owning one is *derived* from
completed quests — no extra save state, and old saves get their items for
free. Two gates use them: the Echo Cave's mouth refuses entry without the
storm-lantern, and any reedy bank becomes a fishing spot (press `e`) once
the rod is in the satchel — strictly catch-and-release, with a running
count of fish met.

Still open for later passes: more gates on *optional* content (the main
road is linear, so meaningful gating wants side content from step 4), and
dimming undiscovered dark places until the lantern is owned.

## 3. Tall-grass encounters — wild runes and the Grimoire ✅

*Done.* Stepping through `Tile::TallGrass` rolls (deterministically, via
`hash2` over a step counter — same walk, same runes) for a **wild rune
encounter**: a little spirit poses a three-option Rust question themed on
its zone's lessons (`content/wilds.rs`, 16 runes — the Silverford grass
hides the legendary Turbofish). Answer true and the rune is inscribed in
your **Grimoire** (`g`), a collection screen that doubles as spaced
repetition for the curriculum. Cozy, no-fail: fleeing is always free, a
wrong answer just fizzles and the rune skitters off to ask again another
day. Both screens are `Screen` variants reachable through
`on_key`/`on_tick`, so the black-box tests can play them.

Still open for later passes: more runes per zone, encounter art in the gfx
build (a proper grass-rustle transition), and rare runes that only stir at
particular spots.

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
