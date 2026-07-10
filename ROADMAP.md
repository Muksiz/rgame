# Rune & Road — Roadmap II

## Jani's notes

Playtest findings, jotted here between sessions — anything from a one-pixel
nit to "this feels wrong". These take priority over everything below: fix
first, then return to the roadmap. Remove a note once it's done.

---

The first roadmap (interiors → items & gates → grass encounters → side
content → NPC life → audio & polish) is **done**: every door opens, keepsakes
gate the world, the tall grass hides wild runes, the world keeps secrets off
the main road, a real day/night clock turns over four zones of weather and
music, and the whole thing fills any window edge-to-edge. That version of the
game is the foundation this file stands on.

This roadmap deliberately makes the world **deeper instead of wider**. No
fifth region, no new core mechanic — instead, the places and people that
already exist start behaving like they were here before you arrived and will
still be here after you leave. Each step multiplies the value of the day/night
clock, the interiors, and the cast that roadmap one built.

Ordering: **the book-shaped curriculum → NPC daily schedules → the world map →
foley & jingles.** The curriculum rewrite comes first because it's the reason
the game exists — a beginner should be able to play it with the book open;
schedules next because everything else reads off a world where people go
places; sound comes last because it polishes whatever exists by then, same as
last time.

---

## 1. The book-shaped curriculum — DONE

**Shipped**: all 20 rewritten quests (1–3 already fit), their templates and
twin solutions, the 16 wild-rune questions re-themed to their zone's chapter,
the NPC idle lines audited (Briar, Maren's neighbors kept their stories;
Fenn, Sil, Sable got new phrasing to match their new lessons), and the
epilogue's "wings we haven't touched" list corrected now that ownership *is*
touched. A grep sweep pins the banished tokens out of every template and
solution. The Turbofish, whose question was chapter-10 material, has returned
upstream — the Update Rune leaps in its pool now. Original notes follow.

Before this, a complete beginner couldn't play along with a book: the quests
leapt ahead of any reading order — `Vec<T>` and `Option<T>` in the Woods, `as`
casting in Emberwick, enums/`match`/`Result`/`?` on Hearthspire (chapter
6/8/9 material). The fix: each section of the world becomes solvable after
reading exactly one chapter of the Brown edition of *The Rust Programming
Language* (rust-book.cs.brown.edu):

- **Emberwick (zone 0, quests 1–7)** — ch. 3 *Common Programming Concepts*
- **Whispering Woods (zone 1, quests 8–14)** — ch. 4 *Understanding Ownership*
- **Silverford (zone 2, quests 15–19)** — ch. 5.1–5.2 *Defining & Using Structs*
- **Hearthspire (zone 3, quests 20–23)** — ch. 5.3 *Method Syntax*

(Three chapters, four zones: chapter 5 splits naturally at 5.2/5.3 — struct
data in the harbor, method syntax on the mountain road.)

**Content-only; the world doesn't move.** Quest ids, zones (7/7/5/4), NPCs,
titles, filenames and story beats all stay — the villagers' chores are
independent of which Rust concept fixes them, and the keepsakes stay pinned
to ids 6 (storm-lantern) and 17 (fishing rod). What changes per quest:
`lesson`, `intro`/`reminder`/`success`/`hints` in `content/quests.rs`, the
template, and its twin solution. Quests 1–3 (`println!`, shadowing, `mut`)
already fit ch. 3 and stay as they are.

The new lesson per quest:

- **Emberwick / ch. 3** — 4 *The Map Pins*: tuples & arrays, `for` over an
  array (3.2+3.5) · 5 *The Toll Board*: `const`, integer types & literals,
  arithmetic incl. integer division and remainder (3.1+3.2) · 6 *The Deep,
  Deep Well*: functions — typed params, `->`, statement vs expression, the
  semicolon-less tail (3.3) · 7 *Open or Closed*: `bool`, comparisons,
  `if`/`else if`/`else`, `if` in a `let` (3.2+3.5). Comments (3.4) are all
  over every template; quest 5 points at them.
  (Reordered so Watchman Fitch's gate quest — right by the road east — is
  the last thing in Emberwick, not the fourth of seven.)
- **Woods / ch. 4** — 8 *Counting Fireflies*: `String` lives on the heap,
  `String::from`, `push_str`, scope & drop (4.1) · 9 *A Spell for Wren*:
  moves and use-after-move, fixed with `.clone()` (4.1) · 10 *The Standard
  Baskets*: passing to a function moves, returning hands ownership back
  (4.1) · 11 *Mushrooms & Manners*: shared borrows `&T` — asking to look,
  not to keep (4.2) · 12 *The Echo Cave*: `&mut T`, one at a time, no
  alias+mutate — an echo is a reference (4.2) · 13 *The Winter Hollow*:
  fixing ownership errors — never return `&` to a local, return the owned
  value (4.3) · 14 *The Lost Bell*: slices, `&s[..n]`, `&str` params over
  `&String` (4.4–4.5).
- **Silverford / ch. 5.1–5.2** — 15 *The Dock Ledger*: define a struct,
  instantiate, dot-access · 16 *The Ferry Token*: `mut` instance, field init
  shorthand in a builder fn · 17 *The Borrowed Rod*: struct update syntax
  `..other` · 18 *The Net Log*: tuple structs · 19 *A Message in a Bottle*:
  `#[derive(Debug)]`, `{:?}`/`{:#?}`, functions borrowing `&Struct`.
- **Hearthspire / ch. 5.3** — 20 *The Lost Book*: `impl`, first `&self`
  method · 21 *Waking the Golem*: methods with parameters, two instances
  (`can_hold`-shaped) · 22 *The Sorting of Spellbooks*: `&mut self` methods ·
  23 *The Missing Page*: associated functions, `Self::new`, `::` calls — the
  capstone weaves struct + methods.

Everything past ch. 5 is banished from templates and solutions: no `Vec`, no
`Option`/`Result`/`match`/`?`, no `as`, no `enum`, no `.iter()`, no closures.
A quest may lean on any *earlier* chapter (Silverford templates can borrow;
Emberwick's can't even reference).

**In lockstep:** the 16 wild-rune questions in `content/wilds.rs` are
zone-tied and follow the same rule — a zone's grass never quizzes past its
chapter (zone 1's current function/loop questions move down to zone 0 topics'
replacements; zones 2–3 become struct/method questions). NPC idle lines in
`world/zones.rs` get audited for now-changed concept references; stories
stay, so most survive.

Invariants (already tested, must keep passing): template fails untouched,
twin solution passes, `#[test]` blocks identical between the pair
(`tests/solve_through.rs`); the full journey still plays start to epilogue
(`tests/journey.rs`); ids/zones monotonic (`quests.rs`); keepsakes at 6 and
17 (`items.rs`). Verify with the full suite plus a grep sweep for the
banished tokens.

## 2. NPC daily schedules — DONE

**Shipped** (in the amended two-state form): a pure
`content/schedule.rs::night_spot(npc)` names where each of the named folk
spends the night — Rowan and Poppy home in their own rooms, Fernshade's
kids called in to Grandmother Ivy's, Sallow and Maren taking their evening
at the Glowworm, the river folk making camp on the piers and the beach,
Hearthspire's scholars withdrawing into the Library. By day everyone stands
at their authored post. `App::apply_schedule` re-derives all positions from
the hour plus the active quest at the moments either can change (campfire
rest, quest pass, save load) — nothing is ever stored, so old saves wake up
already sorted. The active quest's giver ignores the hour: they keep watch
at their post, drawn awake with their marker while everyone else sleeps.
The amble was retired along with the flowing clock — the phase only ever
turns behind the rest screen's fade, so folk are simply *there* when you
wake, which is what sleeping past a change of shift feels like. Snapshot's
`--day` now applies the schedule, so night shots find everyone home.
Invariant tests pin: every night arrangement is standable, unshared and
BFS-talkable in every zone; the giver stays pinned; morning restores every
authored post. Original notes follow.

*(Amended in playtest: the clock no longer turns on its own — the sky waits
at day or night and campfire rests toggle it. Schedules should key off those
two player-driven states rather than four flowing phases.)*

The clock already turns morning → midday → evening → night, and the cast
already sleeps after dark. Now they *live* by it: each named NPC gets an
anchor spot per `DayPhase` — Poppy at her ovens in the bakery kitchen at
morning, in the doorway at midday, on the square bench at evening; Alder at
the workshop bench by day and his porch at dusk; Bram walking the well road.
Interiors are ordinary zones, so a schedule can move someone *indoors* — which
finally gives the furnished rooms their owners.

- **Positions are derived, never stored**: a pure
  `schedule(npc, phase) -> (zone, pos)` in the content layer, same spirit as
  item ownership. Old saves stay valid for free.
- **The active quest pins its giver**: while an NPC's errand is the active
  quest, they wait at their canonical spot all day ("she's been watching the
  road for you") — the linear quest flow and the journey test stay intact.
- If the player is standing in the zone when the phase turns, the NPC ambles
  toward the new anchor for real (BFS path, cosmetic, abandoned at the zone
  edge); otherwise they're simply there next time you arrive.
- Dialogue can lean on place: a line variant for "at work" vs "at rest" where
  it's cheap, so catching Poppy on the bench feels different from the bakery.

Invariants (to test): every anchor is standable and BFS-reachable in its
zone; talk-reach works at every anchor; the active quest's giver is always at
their canonical spot; nothing about schedules touches `SaveData`.

## 3. A companion at your heels — DONE

**Shipped** (ahead of schedules, by request): the stray is a very small crab
in Ferris's exact colors (rustacean.net's #F74C00, hand-pixeled `from_art`
frames), curled in the gap between the Emberwick storehouse and the lean-to
shed. Feed it thrice — one morsel per hour of the day, `crab.fed.*` flags,
tamed derived from any three — and it walks one tile behind you forever:
scuttle frames on the move, a claws-up wave at rest, eyestalks in tall grass,
a startled hop when a wild rune stirs, a doze with its own z after dark, and
a curl in the ember-light on the resting screen. Ferris vouches for it
(family, apparently). Position is ephemeral (`App::companion`), snapped to
your side across warps/gates/loads; `--crab` previews it in the snapshot
tool. Remaining polish ideas: a choice of friend at char-select, a slow
blink at NPCs.

**Amended in playtest**: the taming quest is retired. The little crab is
Ferris himself, at your heels from the first morning — you two go way back —
and he talks: with nothing else in reach, `e` is a chat with your oldest
friend (`content/ferris.rs`, lines picked by `hash2` of spot and hour, with
a quieter night set).

A small friend who walks the road with you — earned, in keeping with the
world, through a gentle side quest (a stray curled behind the storehouse who
needs feeding thrice before it trusts you; flags, like moon-mint). Once won,
it follows a tile behind you, forever.

- **Follow mechanic**: a short breadcrumb queue of the player's recent tiles;
  the companion occupies no collision tile, draws in the world layer, and
  warps through doors with you. Standing still, it sits; at a campfire rest
  it curls up in the ember-light; in tall grass only its ears show.
- **Ownership is a flag**, position is ephemeral — never saved, recomputed at
  the player's side on load, like `walked_at`.
- Hand-pixeled in the existing `from_art` style (the atlas already has
  critters to match), two or three frames of idle/walk. Possibly a choice of
  friend, like the traveller's look at char-select.
- It reacts: a startled hop when a wild rune stirs, a slow blink at NPCs, a
  little `z` beside yours when the folk of the world sleep.

Invariants (to test): the companion is never left standing in a wall or lost
across a warp; render matrix covers world-with-companion; save round-trips
with and without the flag.

## 4. The world map

Press `m` (or find it in the rest menu): a parchment-styled map screen of the
whole journey. The zones are procedural but *deterministic*, so the map can be
honest — a downsampled rendering of the real tile maps (a pixel per couple of
tiles, terrain quantized to a parchment palette), the four zones laid along
the road west to east with the gates drawn as the seams they are.

- **Discovery, not spoilers**: zones you haven't entered yet are blank
  parchment ("uncharted"); visited zones need a tiny `visited.<zone>` flag,
  set on first entry (gate crossings already autosave, so it piggybacks on an
  existing milestone).
- Marks worth drawing: you (a blinking dot), place-name labels in the bitmap
  font, campfires you've rested at, doors you've opened. Found runestones may
  glint on it; unfound ones stay its secret. Gentle — a keepsake of where
  you've been, not a checklist.
- One more `Screen` variant through `App::on_key`, one more scene in
  `examples/snapshot.rs`, one more row in the render matrix — same drill as
  the grimoire.

Invariants (to test): the map screen renders for every combination of
visited-zone flags; `m` round-trips back to the world; snapshot scene added.

## 5. Foley & jingles

The world has music; now it gets *sounds*. The shell derives all audio by
diffing `App` state across frames, which was fine for screens and zones but
too coarse for footsteps — so the lib grows a small, testable event queue:
`App` pushes semantic events (`Stepped(Terrain)`, `DoorUsed`, `ChestOpened`,
`QuestPassed`, `RuneCaught`, `StoneFound`, `MenuMoved`, …) and `main.rs`
drains them into sounds each frame. The lib and tests stay silent and
window-free; the tests can finally *assert* that walking on wood sounds
different from grass, without hearing a thing.

- **Footsteps by terrain** — soft grass, firmer path, wooden floors, cave
  stone; quiet, every other step, mixed well under the music.
- **The world's small noises** — door creaks on warp, the cellar chest's
  groan, a coin-ish chime when a keepsake changes hands (Kenney *RPG Audio*).
- **Jingles at the milestones** — a short fanfare on quest pass, a soft
  sparkle when a rune joins the Grimoire, a gleam for a found runestone, and
  the gentlest possible "no harm done" for a fizzle (Kenney *Music Jingles*).
- **Menus blip** (Kenney *UI Audio*), the encounter gets its sting and the
  campfire its rest theme (the unused Junkala tracks — 3 left in each pack).
- **A sound option** joins text speed in the rest menu (off/quiet/full),
  saved behind `#[serde(default)]` like everything else.

All sources are already vetted CC0 on the asset shelf in `CLAUDE.md`; each
gets its line in `assets/CREDITS.md` when it lands.

---

## The shelf — considered, wanted, not this roadmap

Ideas weighed for this cycle and deliberately deferred, so they aren't
re-litigated from scratch next time:

- **A fifth region** with an advanced quest arc (traits & generics,
  lifetimes, iterators & closures) — flavors sketched: a harbor town on a
  grey sea, deep mines under Hearthspire, a misty archipelago.
- **Castable grimoire runes** — caught runes as gentle overworld magic.
- **Gardening on the clock** and **cooking at Poppy's ovens**.
- **Coins & a trading post.**
- **Dynamic weather fronts & seasons**; **festival days**; a **fellow
  traveler** you keep crossing on the road.
- **Visible wild-rune forms** in encounters (Ninja Adventure monster strips,
  already vendored) and item icons in the journal satchel.
- **Save slots** and a home shelf displaying runestones and keepsakes.

---

## Engineering ground rules (hold these while doing any of the above)

- Every new screen is a `Screen` variant reachable through `App::on_key` /
  `App::on_tick` — the black-box tests must be able to play it.
- All randomness derives from `hash2(x, y, seed)` — playthroughs stay
  deterministic and testable.
- Anything derivable is derived, never stored: item ownership from
  `completed`, NPC positions from the phase, the companion's spot from yours.
  New persistent state goes in `SaveData` behind `#[serde(default)]` — an old
  `save.json` must always keep loading.
- Tile appearance lives in one place: `tile_sprites()` in `gfx/scene.rs`.
- Extend the render matrix (`tests/render.rs`) and the world invariant tests
  (`world/zones.rs`) with every new screen, schedule anchor, or warp.
- Autosave stays a milestone thing (quest pass, gate, quit) — frequent
  actions must not write to the cwd, or unit tests start littering the repo.
- Sound lives entirely in the `src/main.rs` shell; the lib emits events, the
  shell plays them. Tests assert events, never audio.
- Names stay in-world and original: the Grimoire, keepsakes, fizzles —
  nothing borrowed from the franchises that inspired the shape.
- Tone is spec: cozy, gentle, no fail states, the compiler is the politest
  grump.
