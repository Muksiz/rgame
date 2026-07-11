# Rune & Road — Roadmap III

## Jani's notes

Playtest findings, jotted here between sessions — anything from a one-pixel
nit to "this feels wrong". These take priority over everything below: fix
first, then return to the roadmap. Remove a note once it's done.

---

Two roadmaps in. The first built the world outward (interiors, keepsakes,
wild runes, secrets, day/night, audio); the second made it deep — every
quest teaches exactly one chapter of the Brown Book (ch. 3 → 5.3 across the
four zones), the folk go home at night to beds that exist, Ferris walks at
your heels and talks, the journey folds out onto honest parchment at `m`,
and every footstep, door and milestone has its sound. All five items
shipped; their full write-ups live in git history.

This roadmap lets the world **grow again — but only past the end of the
road**. The epilogue stays the ending of the main journey; what's new
starts after it, or beside it. One new region resumes the curriculum where
chapter 5 left off, and the three quieter arcs pay off collections the
first two roadmaps built: the grimoire learns to cast, the villages learn
to trade and cook, the year learns to turn, and the things you've gathered
get a shelf to stand on.

Ordering: **the far shore → castable runes → coins & kitchens → the turning
year → keepsakes & save slots.** Curriculum first, same reason as always —
it's why the game exists. Castable runes next because they're small and
instantly multiply a collection that already works. Coins before festivals
because festivals want things to trade and cook. Keepsake polish last, so
the shelf can display everything the cycle added.

---

## 1. Mistholm — the far shore (ch. 6, *Enums and Pattern Matching*)

The Silverford ferry — moored scenery since the day it was baked — finally
sails. Once all 23 quests are done and the epilogue has rolled, boarding it
carries you to **Mistholm**: a small grey-water archipelago, pier-linked
isles under sea mist, gulls, waves (the vendored waves/storm ambience beds
have waited for exactly this). The road ends; the sea doesn't.

The region teaches **chapter 6** — the first of the banished tokens comes
home. Six quests (ids 24–29), one lesson each, in book order:

- **24** — define an `enum`, variants as the kinds a thing can be (a tide
  is `Ebb` or `Flood`; a lamp is `Lit` or `Dark`) (6.1)
- **25** — variants that carry data, constructing them (6.1)
- **26** — `match`: every arm, exhaustiveness as a kindness (6.2)
- **27** — patterns that bind values out of variants (6.2)
- **28** — `Option<T>`: lights in the mist are `Some` or `None`; why the
  compiler makes you say which (6.1 + 6.2)
- **29** — `if let` / `let…else` for the one-armed cases (6.3)

Everything past ch. 6 stays banished from templates and solutions (no
`Vec`, no `Result`/`?`, no traits, no closures); everything earlier is
fair game, as always.

In lockstep, like last time:

- **Four new wild runes** in the isle grass, quizzing ch. 6 — the grimoire
  grows 16 → 20 (new pages, new Ninja Adventure forms).
- **A small new cast** (a ferrykeeper, isle folk) quarried from the
  vendored pack; a new zone music pick from the 42 tracks and a night bed
  from the shelf; mist as the zone weather.
- **Ferris has opinions** about the sea (`content/ferris.rs` grows a
  region), and the parchment map learns a fifth panel.

**Engineering — the one structural rule**: interiors currently start at
zone index 4, and "overworld" is assumed as `zone 0–3` in the music
tables, weather, the map, `visited.<zone>` flags, `content/wilds.rs`'s
zone assertion and Ferris's regions. Mistholm therefore **appends at the
end of `zones()`** (no existing index shifts, old saves keep loading), and
overworld-ness becomes a first-class property — a regions list or a
`Zone` field — consulted everywhere `0..4` is assumed today. Quest `zone`
fields stay monotonic (17 > 3, the test keeps passing); keepsakes stay
pinned at 6 and 17.

Invariants (to test): template fails untouched / twin solution passes /
identical `#[test]` blocks for 24–29; the journey test extends past the
epilogue, boards the ferry and finishes quest 29; BFS reachability,
standability and warp round-trips across every isle; the map renders every
charted combination including the fifth panel; ids/zones monotonic; an old
23-quest `save.json` still loads and the ferry is already waiting.

## 2. Castable grimoire runes

Caught runes stop being trophies. In the world, a new key opens a small
casting ring of everything in the grimoire; choosing a rune casts it —
gentle, cosmetic-first overworld magic in the existing spark/puff FX
style. Charm, not keys: **no cast may substitute for a keepsake or open
anything gated** — the storm-lantern and the rod keep their jobs.

- Each rune's effect keys to its name and nature: flowers bloom in a ring,
  water ripples, startled birds, a soft chime, a moment of light that
  doesn't count as the lantern, a rune that makes Ferris weigh in, and one
  small honest utility — a rune that makes the nearest unfound runestone
  glimmer for a breath.
- All effects derive from `hash2` and tick — nothing persists, nothing
  saved. Casting is free and can't fail; fizzles stay quest-things.
- One new `Screen` (or overlay) reachable through `App::on_key`, a
  snapshot scene, a render-matrix row.

Invariants (to test): every caught rune is castable and every cast
returns cleanly to the world; casting changes no flag, no save, no gate;
the ring renders at 0, some, and all runes caught.

## 3. Coins & kitchens — economy & home life

The villages learn to trade, quietly. Nothing on the main road ever costs
a coin — money is a side-layer, like fishing.

- **Coins**: the first stored counter — `SaveData::coins` behind
  `#[serde(default)]`, earned gently and spent locally.
- **The trading post**: the Emberwick market stall (the prefab is already
  standing) gets a keeper who buys what the world already yields — caught
  fish, moon-mint-style herbs, mushrooms — and sells seeds and small
  goods.
- **Gardening on the clock**: a few soil plots; plant a bought seed, and
  growth advances one stage per campfire rest — the two-state clock
  already measures nights, so crops literally grow while you sleep.
  Derived where possible; the planted-state that can't be derived goes in
  `SaveData` behind `#[serde(default)]`.
- **Cooking at Poppy's ovens**: combine harvest into a handful of little
  dishes at the bakery kitchen. Dishes are gifts — each named villager has
  a favorite and a new line for receiving it (flags, like moon-mint).
  No stats, no buffs; the reward is the line.
- Autosave stays milestone-only: buying, planting and gifting don't write;
  the existing milestones carry the state to disk soon enough.

Invariants (to test): an old `save.json` (no coins, no garden) loads with
empty pockets and bare plots; selling/buying/planting/cooking round-trip
through `on_key` black-box; no new writes to cwd from unit tests; the main
journey completes with zero coins touched.

## 4. The turning year

The sky and the calendar start moving — always derived, never stored
beyond one counter.

- **Rest count is the calendar**: `SaveData::rests` behind
  `#[serde(default)]` (old saves wake on day zero). Everything below
  derives from it.
- **Weather fronts**: each zone's weather stops being eternal — a
  deterministic function of (zone, rest count) drifts the fronts, so some
  mornings Silverford's rain has wandered to Emberwick and the Woods'
  fireflies rest. Static per zone per day, same as now; only the schedule
  of days changes.
- **Seasons**: every dozen-ish rests the season turns — a palette tint
  through `gfx::shade` and a particle swap (petals → leaves → snow →
  petals). Cosmetic everywhere; crops and quests don't care.
- **Festival days**: on certain rest counts the Emberwick square dresses
  up — lanterns, bunting, the cast gathered off-schedule, one-off lines,
  maybe Poppy's dishes on a long table. Gone the next morning.
- **A fellow traveler**: one recurring stranger who is always a zone ahead
  or behind, met at set story beats along the road, trading a few words
  each time — and standing on the Mistholm pier at the end.

Invariants (to test): weather/season/festival state is a pure function of
the rest counter (same count ⇒ same world); every festival arrangement is
standable and BFS-talkable; the fellow traveler is never in a wall and
never blocks a road; old saves load at rest zero with today's exact
weather.

## 5. Keepsakes & save slots

The collections get their home, and the household gets room for more than
one journey.

- **A home of your own**: one Emberwick door that was always yours — a
  small room with a bed, a table and **the shelf**: found runestones and
  keepsakes drawn standing on it, filling in as they're earned (derived
  from `completed` and flags, drawn like the grimoire counts, never
  stored).
- **Journal item icons**: the keepsakes in the journal satchel get their
  Ninja Adventure icon art (the 60+ item icons are vendored; this was the
  last piece of the visible-forms work).
- **Save slots**: three slots on the title screen. The existing
  `save.json` loads as slot one unchanged — compatibility is the promise —
  and new slots write beside it (`save2.json`, `save3.json`). Char-select
  runs per slot; delete asks twice, gently.

Invariants (to test): a pre-slots `save.json` appears in slot one exactly
as it was; each slot round-trips independently; the shelf renders at
every ownership combination in the render matrix; the home room passes
every interior invariant (warp back, reachable, standable).

---

## The shelf — considered, wanted, not this roadmap

- **A sixth region** for the advanced arc — traits & generics, lifetimes,
  iterators & closures (ch. 10 and 13 material). Flavors still banked:
  deep mines under Hearthspire, a harbor town on a grey sea (whichever
  Mistholm doesn't use).
- **Collections & error handling** (ch. 8–9) as the bridge between
  Mistholm and the advanced arc — `Vec`, `String` in anger, `HashMap`,
  `Result` and `?`.
- **Seasons that matter**: crops by season, festival menus by harvest —
  deferred until the cosmetic year has been lived with.
- **NPC-to-NPC life**: villagers talking to each other, visible errands
  between anchors.

---

## Engineering ground rules (hold these while doing any of the above)

- Every new screen is a `Screen` variant reachable through `App::on_key` /
  `App::on_tick` — the black-box tests must be able to play it.
- All randomness derives from `hash2(x, y, seed)` — playthroughs stay
  deterministic and testable.
- Anything derivable is derived, never stored: item ownership from
  `completed`, NPC positions from the hour, the shelf's contents from
  flags, the whole turning year from one rest counter. New persistent
  state goes in `SaveData` behind `#[serde(default)]` — an old `save.json`
  must always keep loading.
- Overworld-ness is a property, not an index range — nothing new may
  assume `zone < 4`, and new zones append so existing indices never shift.
- Tile appearance lives in one place: `tile_sprites()` in `gfx/scene.rs`.
- Extend the render matrix (`tests/render.rs`) and the world invariant
  tests (`world/zones.rs`) with every new screen, region, anchor, or warp.
- Autosave stays a milestone thing (quest pass, gate, quit) — frequent
  actions must not write to the cwd, or unit tests start littering the repo.
- Sound lives entirely in the `src/main.rs` shell; the lib emits events,
  the shell plays them. Tests assert events, never audio.
- Nothing on the main road ever needs a coin, a cast rune, or a season —
  the new layers stay optional, like fishing.
- Names stay in-world and original: the Grimoire, keepsakes, fizzles —
  nothing borrowed from the franchises that inspired the shape.
- Tone is spec: cozy, gentle, no fail states, the compiler is the politest
  grump.
