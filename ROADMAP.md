# Jani's notes (please remove this section from the roadmap when done)

## The library

The library should be more detailed and have multiple rooms. I'd like it to also
have large windows from where the sun can shine. Some bookshelves carry the same books,
add more books so that the game does not have to compensate by having the same book in
multiple places. There should also be a room in the library with a showcase of random
plants, rocks, artwork and such.

## Player's choice

When you begin a new game you should be able to choose your own character and name for the
character. Have at least a couple of choices for playable character.

## Time of day and Campfires

Bring in the time of day feature. I want morning to be 10 minutes in real life, day to be 20 min
in real life, evening 10 min of real life, and the night be 15 min in real life.

Scattered campfires as interaction points. When you press e by a campfire the screen fades to
black and you get a random Rust lore snippet. When you wake up it's night. At night the NPC's
are sleeping, and when you press e again by the campfire you get a random Rust lore snippet and
the time switches to morning.

## Other general stuff

The random rune encounters in the grass happen too often currently. They should not happen as
often as they do now. I also don't want the same grass interaction to happen twice, keep count
of those I've already answered correctly. A key rule for this project is that the more free sprites/textures
you can fetch from the internet that fit for this project the better. If you only use the same sprites/textures
across everything the game becomes pretty blend. Be radical in coming up with new scenes, animations, ideas,
gameplay.

# Rune & Road — Roadmap

The long walk from "cozy quest corridor" to a full-fledged game in the spirit
of old Pokémon and Zelda. Each step multiplies the value of the world that
already exists; polish comes last because the tone is already carrying a lot.

Ordering: **interiors ✅ → items & gates ✅ → grass encounters ✅ →
side content ✅ → NPC life (next) → audio & polish.**

**Where things stand** (July 2026): steps 1–4 are shipped — every door
opens, each place keeps its own fixed hour and weather, quests leave real
keepsakes that gate the world, the tall grass hides sixteen wild runes that
fill the Grimoire, and the world now keeps secrets off the main road: side
quests remembered in world flags, eight hidden runestones, a locked cellar,
and a Library of real books about Rust. Along the way, Fern became **Wren**
(too close to Ferris), the rune collection is deliberately the **Grimoire**
— in-world names over franchise-adjacent ones — and the original Ratatui
TUI frontend was retired: the sprite-rendered Macroquad build is the game
now (`cargo run`).

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
`gfx/frame.rs`).

## 2. Items, inventory, and ability gates ✅

*Done.* Keepsakes are real now (`content/items.rs`): Bram's storm-lantern
(quest 3) and Juniper's spare rod (quest 8), handed over in the success
dialogue and listed in the journal's satchel. Owning one is *derived* from
completed quests — no extra save state, and old saves get their items for
free. Two gates use them: the Echo Cave's mouth refuses entry without the
storm-lantern, and any reedy bank becomes a fishing spot (press `e`) once
the rod is in the satchel — strictly catch-and-release, with a running
count of fish met.

Still open for later passes: dimming undiscovered dark places until the
lantern is owned. (Step 4 delivered the promised gate on *optional*
content: the storehouse cellar is dark, and the lantern is the way in.)

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

Still open for later passes: more runes per zone, encounter art (a proper
grass-rustle transition), and rare runes that only stir at particular spots.

## 4. Side quests, secrets, and collectibles ✅

*Done.* `active_quest()` stays strictly linear for the main road; everything
here lives off it, remembered in a new `flags: BTreeSet<String>` on
`App`/`SaveData` (behind `#[serde(default)]`, so old scrolls keep loading):

- **Side quests** (`content/sides.rs`, dialogue driven by flags): Granny
  Sorrel's favor — a sprig of moon-mint from a patch off the Echo Cave
  path, for a kettle that finally commits to boiling — and Old Nettle, a
  whittler hidden deep in the Whispering Woods where no road goes, who
  hands over a rusted key stamped EMBERWICK STOREHOUSE.
- **The locked chest**: the storehouse grew a cellar door (a new interior
  zone) — pitch dark, so Bram's storm-lantern gates it, the promised
  "gate on optional content" — and the chest down there wants Nettle's key.
- **Collectibles** (`content/stones.rs`): eight hidden runestones, seven
  standing in quiet corners of the world (they glimmer until found), the
  eighth inside the chest. The journal counts them once the first is found.
- **Interior secrets**: signs generalized into notes that can sit on any
  tile — a recipe card in the bakery, Alder's measurements, a storehouse
  inventory whose margins point at both the cellar and the lost key.
- The Great Library's shelves are readable (`content/books.rs`) — sixteen
  real books on Rust's features, guides, and history, one per shelf tile,
  browsed in walking order with `e`.

Still open for later passes: more side arcs (the flags plumbing makes them
cheap now), and a bush that hides a passage once you've learned the right
rune.

## 5. NPC life and dialogue depth

- A one-tile wander for NPCs (the critters already know how), facing the
  player when spoken to.
- Dialogue that varies by flags ("heard you fixed the well!") — the `idle`
  arrays already hold multiple lines; today only the first is used.
- A tiny cutscene primitive (a scripted move/say/wait sequence driven from
  `on_tick`) so quest payoffs feel staged instead of instant.

## 6. Presentation polish

- **Audio**: a CC0 chiptune loop per zone, SFX for cast/pass/fizzle (keep
  playback in the `src/main.rs` shell so tests stay silent and headless).
- Zone-entry banners ("~ Whispering Woods ~" sliding in), a walk cycle and
  grass-rustle particles.
- An options screen — text speed first, the most Pokémon setting there is.
- Save slots, or at least a "save & quit" confirmation.

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
