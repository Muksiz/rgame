# Rune & Road — Roadmap

The long walk from "cozy quest corridor" to a full-fledged game in the spirit
of old Pokémon and Zelda. Each step multiplies the value of the world that
already exists; polish comes last because the tone is already carrying a lot.

Ordering: **interiors ✅ → items & gates ✅ → grass encounters ✅ →
side content ✅ → NPC life ✅ → audio & polish ✅.**

**Where things stand** (July 2026): every door opens, quests leave real
keepsakes that gate the world, the tall grass hides wild runes that fill the
Grimoire, and the world keeps secrets off the main road: side quests in world
flags, eight hidden runestones, a locked cellar, and a Library of real books
about Rust.

On top of that foundation the world now **lives**: a real day/night clock
turns morning → midday → evening → night in real time, dimming and brightening
the whole outdoors while each place keeps its own weather; scattered
**campfires** let you rest to a scrap of Rust lore and wake at the next turn of
the day, with the folk of the world asleep by night. You **choose your
traveller** — a look and a name — before setting out. The **Great Library**
grew into three sunlit halls (stacks, grand entrance, a showcase gallery of
plants, art and curios) behind tall windows that pour light onto the floor,
with a book for every shelf. Wild-rune encounters are gentler now (rarer, and
never re-asking a question you've answered true). NPCs sleep at night, glance
about by day, and thank you once their errand is done; arriving somewhere new
raises a **sliding place-name banner**, and the rest menu carries an
**options** toggle for text speed.

Under the hood: the framebuffer is sized to the window so the picture fills
the screen edge-to-edge with **no black bars** — ultrawide and superultrawide
included — and letter-key input is read from the OS text stream, so the game
follows the player's **keyboard layout** (Dvorak, AZERTY, …). Movement is
arrow keys and vim `H J K L`; `e`, Enter and Space are one and the same
"confirm".

Along the way, Fern became **Wren** (too close to Ferris), the rune collection
is deliberately the **Grimoire** — in-world names over franchise-adjacent ones
— and the original Ratatui TUI frontend was retired: the sprite-rendered
Macroquad build is the game now (`cargo run`).

---

## 1. Interiors and warps ✅

*Done.* Every door leads somewhere: the five Emberwick houses (Poppy's
Bakery, Granny Sorrel's and Tilly's cottages, Alder's workshop, the old
storehouse), the Echo Cave behind its rocky mouth, and the Great Library —
so the epilogue's destination is a real place. Interiors are ordinary
`Zone`s (indices 4+): a furnished room stamped into a map of `Tile::Void`,
entered via `Warp { at, to_zone, to_pos }` on each `Door` tile.

Invariants (tested): every Door tile has a warp; every warp lands on ground
reachable from the destination zone's spawn; every way in has a way back.

## 2. Items, inventory, and ability gates ✅

*Done.* Keepsakes are real (`content/items.rs`): Bram's storm-lantern
(quest 3) and Juniper's spare rod (quest 8), handed over in the success
dialogue and listed in the journal's satchel. Owning one is *derived* from
completed quests — no extra save state, and old saves get their items for
free. Two gates use them: the Echo Cave's mouth refuses entry without the
storm-lantern, and any reedy bank becomes a fishing spot with the rod. The
storehouse cellar is dark, and the lantern is the way in.

## 3. Tall-grass encounters — wild runes and the Grimoire ✅

*Done.* Stepping through `Tile::TallGrass` rolls (deterministically, via
`hash2` over a step counter) for a **wild rune encounter**: a three-option
Rust question themed on its zone's lessons (`content/wilds.rs`, 16 runes).
Answer true and the rune is inscribed in your **Grimoire** (`g`). Cozy,
no-fail: fleeing is always free, a wrong answer just fizzles. Encounters are
deliberately uncommon, and a rune already in the Grimoire never stirs again —
so the grass is a place to wander, not a gauntlet.

## 4. Side quests, secrets, and collectibles ✅

*Done.* Everything off the main road lives in a `flags: BTreeSet<String>` on
`App`/`SaveData` (behind `#[serde(default)]`): Granny Sorrel's moon-mint
favor, Old Nettle hidden in the deep woods with the cellar key, the locked
chest, eight hidden runestones (seven standing, one in the chest), interior
notes on tables and crates, and the Great Library's readable shelves
(`content/books.rs`, now twenty-four unique books).

## 5. NPC life and dialogue depth ✅

*Done.* NPCs turn to face a visitor within talking reach and glance about on
their own unhurried beat by day; at night they are drawn asleep (a little
drifting `z`) and give out no errands. Dialogue varies by state: once an
NPC's quest is complete they switch to a warmer, grateful line
("water's running clear again since you sorted the well…"). A tiny scripted
cutscene primitive remains a nice-to-have, but quest payoffs already stage
themselves through the success/keepsake dialogue.

## 6. Presentation polish ✅

*Done.*

- **Day/night clock** — morning/midday/evening/night turn in real time and
  drive the sky over every outdoor zone (interiors keep their own steady
  lamplight). The HUD shows the current phase; a sun/dusk/moon icon tracks it.
- **Campfires** — scattered rest points; `e` fades the screen to embers, tells
  a scrap of Rust lore (`content/lore.rs`), and rolls the clock to the next
  rest (daytime → night, night → a fresh morning).
- **Character choice** — pick a look and type a name before the first step;
  both persist in the save.
- **The Great Library** — three connected chambers, tall sunlit windows that
  cast light onto the floor, a showcase gallery (plants, framed art, curios),
  and enough books that no shelf repeats.
- **Zone-entry banners** — a place-name plate slides in when you arrive.
- **Options** — a text-speed toggle lives in the rest menu (Slow/Normal/Fast),
  saved with the rest of your progress.
- **Fills the window** — the framebuffer matches the window's shape, so
  ultrawide and superultrawide displays fill edge-to-edge with no black bars,
  and the map borders pad the view with scenery rather than cutting to black.
- **Layout-aware input** — letter keys come from the OS text stream, following
  the player's keyboard layout; movement is arrows + vim keys, and
  `e`/Enter/Space are one unified confirm.
- **Audio** — a CC0 chiptune loop per overworld zone (interiors stay quiet), a
  title theme looping through the title/char-select screens, and one-shot SFX
  for cast/pass/fizzle, all by Juhani Junkala (`assets/CREDITS.md`). Sound
  lives entirely in the `src/main.rs` shell, loaded via macroquad's `audio`
  feature and driven by diffing `App::screen` and `App::zone_idx` across
  frames — the lib and tests stay window- and sound-free.

Still open for a later pass:

- Save slots.

---

## Engineering ground rules (hold these while doing any of the above)

- Every new screen is a `Screen` variant reachable through `App::on_key` /
  `App::on_tick` — the black-box tests must be able to play it.
- All randomness derives from `hash2(x, y, seed)` — playthroughs stay
  deterministic and testable.
- Tile appearance lives in one place: `tile_sprites()` in `gfx/scene.rs`.
- Extend the render matrix (`tests/render.rs`) and the world invariant
  tests (`world/zones.rs`) with every new screen, zone, or warp.
- New persistent state goes in `SaveData` behind `#[serde(default)]`, and
  anything derivable (like item ownership) is derived, not stored — an old
  `save.json` must always keep loading.
- Autosave stays a milestone thing (quest pass, gate, quit) — frequent
  actions must not write to the cwd, or unit tests start littering the repo.
- Names stay in-world and original: the Grimoire, keepsakes, fizzles —
  nothing borrowed from the franchises that inspired the shape.
- Tone is spec: cozy, gentle, no fail states, the compiler is the politest
  grump.
