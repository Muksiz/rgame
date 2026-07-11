# Asset credits

`atlas.png` is baked by `tools/bake_atlas.py` from four spritesheets by
**Kenney** (www.kenney.nl), all licensed **Creative Commons Zero (CC0)**:

- *Roguelike/RPG pack* — `kenney/roguelikeSheet_transparent.png`
  https://kenney.nl/assets/roguelike-rpg-pack
- *Roguelike Characters* — `kenney/roguelikeChar_transparent.png`
  https://kenney.nl/assets/roguelike-characters
- *Roguelike Indoors* — `kenney/roguelikeIndoor_transparent.png`
  https://kenney.nl/assets/roguelike-indoors
  (the furniture that tells each interior apart: counters, stoves, beds,
  frames, the Library's piano and case clock)
- *Roguelike Caves & Dungeons* — `kenney/roguelikeDungeon_transparent.png`
  https://kenney.nl/assets/roguelike-caves-dungeons
  (the Echo Cave and storehouse cellar: cave floors, stone walls,
  stalagmites, crystals, mushrooms, cobwebs)

The player and every villager are from the **Ninja Adventure asset pack** by
**Pixel-boy and AAA** (https://pixel-boy.itch.io/ninja-adventure-asset-pack),
also **CC0**: `ninja_adventure/<Character>/` holds each cast member's idle
strip (plus the player's walk strip), unmodified, and
`ninja_adventure/tilesets/` the pack's floor/nature/detail tilesets that give
each overworld zone its own biome ground and props. The older Kenney-composed
character cells remain in the atlas so existing sprite ids never shift.

The same pack now also furnishes the village expansion:
`ninja_adventure/tilesets/TilesetHouse.png` holds the premade homes (the
thatched, plain and flat-roofed cottages, the two-storey townhouse, the
shopfront and the tavern) — flavor houses whose open doorways
`tools/bake_atlas.py` pastes shut with the sheet's own plank door — and the
big old-growth trees (blossoming, summer, snow-dusted and autumn crowns,
towering pines, triple canopies) plus the garden bushes and tall flowers are
cropped from `TilesetNature.png`, already vendored above.

`ninja_adventure/pack/` vendors the rest of that same download unmodified
(`Actor/Character`, `Actor/CharacterAnimated`, `Actor/Animal`, `Actor/Boss`,
`Actor/Monster`, `Items`, `FX`, `Ui`, the full `Backgrounds` tree and its
`LICENSE.txt`, `README.md` and `Palette.png`) — raw source material for
future features (journal/keepsake icons, casting FX, the `Ui/Dialog` boxes
and `Ui/Emote` balloons) so pulling a new sprite into the atlas is a crop,
not a re-download. First harvests: the sixteen wild runes' visible forms on
the encounter screen are `Actor/Monster` creatures (two down-facing frames
each, see `WILD_FORMS` in `tools/bake_atlas.py`), the casting screen's
blooming spark circle is `FX/Magic/Circle` and the fizzle's smoke puff
`FX/Smoke` (six 32×32 frames each, baked as 2×2 cell blocks). The pack's whole soundtrack and
SFX library is vendored too — see "Ninja Adventure audio" under Audio below.
`ninja_adventure/tilesets/` likewise now holds *every* tileset the pack
ships — the interior set (`Interior/`, walls, floors and furnishings), the
desert, dungeon, field, towers, relief and logic sheets and the rest — so
future rooms and biomes are also a crop away.
`tools/bake_atlas.py` reads the flat `ninja_adventure/<Character>/Idle.png`
layout above; eleven characters (Tansy, Fitch, Hobb, Reed, Pip, Briar, Yew,
Sable, Fenn, Sil, Faye — the beginner-quest expansion's cast, added as a tail
block in the atlas so no earlier id shifts) instead carry a plain
`SpriteSheet.png` copied straight out of `pack/Actor/Character/<Name>/`,
which the bake script's `na_idles()` reads just as well (its top row is the
same four idle facings).

The village buildings — the perspective-drawn cottages, barn, shed, market
stall and plaza fountain — are from **"Zelda-like tilesets and sprites"** by
**ArMM1998** (https://opengameart.org/content/zelda-like-tilesets-and-sprites,
**CC0**): `zelda_like/Overworld.png` is the pack's overworld sheet, unmodified;
`tools/bake_atlas.py` crops each building prefab out of it (and pastes the
sheet's arched-door piece onto the doorless ones) at bake time.

The rest of that same Zelda-like download is now vendored too, unmodified:
`zelda_like/cave.png` and `zelda_like/Inner.png` (its cave and interior
sheets), `zelda_like/objects.png` (chests, pots, hearts and small props),
`zelda_like/character.png` and `zelda_like/NPC_test.png` (its player and NPC
sprite sheets — the last unvendored pieces), `zelda_like/log.png` (its
dialogue-box art) and `zelda_like/font.png` (its bitmap font) — the whole
pack is now in the repo; raw source material, nothing baked from these yet.

Two more sheets from the **Ninja Adventure asset pack** (CC0, credited above)
sit in `ninja_adventure/tilesets/` and are baked into the atlas:
`TilesetWater.png` gives Silverford's harbor its pier planks, piling ends and
moored skiffs (its water autotile sets remain unbaked), and
`TilesetVillageAbandoned.png` the two moss-eaten abandoned houses standing in
the Whispering Woods. The same pack's dead and gnarled trees (from
`TilesetNature.png`) stand through the deep woods, and its forest-floor tiles
are additionally baked in a dimmed mossy variant (the `WOODS_FLOOR` family)
for the same zone. Alongside them, `ninja_adventure/pack/Backgrounds/`
vendors `Vehicles/Boat.png` (the ferry tied up at the long pier) and
`Animated/WaterRipples/SpriteSheet16x16.png` (the four-frame ripple lapping
around hulls and pilings).

`kenney/input_prompts/` holds the tilemap sheets of **Kenney's** *Input
Prompts Pixel 16×* (https://kenney.nl/assets/input-prompts-pixel, **CC0**):
16×16 keyboard-key glyphs for drawn key hints (`e`, arrows, `g`) in the HUD
and dialogue; `Tilesheet.txt` is the pack's own layout notes (`tilemap.png`
has 1px gutters, `tilemap_packed.png` has none).

`kenney/pixel_ui/` holds **Kenney's** *Pixel UI Pack*
(https://kenney.nl/assets/pixel-ui-pack, **CC0**): the two spritesheets plus
the pack's 9-slice panel pieces — 750 bits of panel/border chrome for
whenever the journal, grimoire or options menu outgrow the hand-drawn boxes.
Nothing baked from it yet.

`kenney/tiny_dungeon/` holds the tilemap sheets of **Kenney's** *Tiny
Dungeon* (https://kenney.nl/assets/tiny-dungeon, **CC0**, 16×16):
dungeon walls, doors, crates, potions, weapons and a small bestiary in a
soft palette — quarry for a dungeon-style interior. Nothing baked from it
yet; `Tilesheet.txt` is the pack's own layout notes.

`fonts/managore/` holds **m5x7** and **m3x6** by **Daniel Linssen**
(https://managore.itch.io/m5x7, https://managore.itch.io/m3x6, free to use
with attribution appreciated — this credit is that attribution): compact
pixel display faces for banners and titles. Not wired in yet.

`fonts/monogram/` holds **monogram** by **Vinícius Menézio (datagoblin)**
(https://datagoblin.itch.io/monogram, **CC0**): `monogram-bitmap.png` +
`monogram-bitmap.json` are the embeddable bitmap form (same shape as the
`font8x8` tables in `gfx/font.rs`), `monogram-extended.ttf` the full
extended-charset face, `credits.txt` the author's own credits file.

The critter, Ferris, bookshelf, chest, runestone and moon-mint sprites in
the atlas are original pixel art for this project (same CC0 spirit — do
what you like with them). So are the six frames of the little crab
companion, drawn after Rust's mascot Ferris (rustacean.net, by Karen Rustad
Tölva, CC0) in its canonical colors.

To rebake after editing the script:

```sh
python3 tools/bake_atlas.py assets/kenney/roguelikeSheet_transparent.png assets/kenney/roguelikeChar_transparent.png
```

## Audio

`assets/audio/` holds every sound the game plays. The chiptune loops and retro
SFX below are all by **Juhani Junkala** (https://juhanijunkala.com), **CC0**;
the night ambiences and the owl come from other artists and are credited under
their own headings further down.

- `audio/music/` — one loop per overworld zone. Three are from *Chiptune
  Adventures* (https://opengameart.org/content/4-chiptunes-adventure):
  `emberwick.ogg` ("Stage 1"), `silverford.ogg` ("Stage Select"),
  `hearthspire.ogg` ("Boss Fight" — the Hearthspire Approach is guarded by the
  Stone Golem, so the fit was too good to pass up). The Whispering Woods,
  though, wanted something calmer and stranger than a chiptune:
  `whispering-woods.ogg` is **"Lanterns in the Hollowed Forest"** by **Tsorthan
  Grove** (https://opengameart.org/content/lanterns-in-the-hollowed-forest,
  **CC0**) — a soft-horror loop of whispers drifting from the branches, re-encoded
  from FLAC to Ogg Vorbis. Interiors (zone 4+) stay quiet. `title.ogg` is the
  "Title Screen" track from *5 Chiptunes (Action)*
  (https://opengameart.org/content/5-chiptunes-action, Juhani Junkala), looping
  through the title and char-select screens.
- `audio/sfx/` — `cast.ogg`, `pass.ogg` and `fizzle.ogg`, trimmed from *The
  Essential Retro Video Game Sound Effects Collection*
  (https://opengameart.org/content/512-sound-effects-8-bit-style): a menu
  blip, a fanfare and a soft error tone.

- `audio/shelf/` — the rest of both Junkala chiptune packs. Two now play:
  `chiptune-adventures-stage-2.ogg` ("Stage 2", unmodified) is the campfire
  rest theme, and `chiptunes-action-level-1.ogg` is the wild-rune encounter
  sting. Still on the shelf: `chiptunes-action-level-2/3.ogg` and
  `chiptunes-action-ending.ogg` (re-encoded from the pack's WAVs to Ogg
  Vorbis q5, otherwise unmodified).

### Kenney audio packs

`kenney/audio/` vendors three of **Kenney's** audio packs in full (all
**CC0**, each folder carries the pack's own `License.txt`), and the foley
layer now plays from them:

- `kenney/audio/rpg-audio/` — *RPG Audio*
  (https://kenney.nl/assets/rpg-audio): `footstep00–09.ogg` are the
  terrain footsteps (grass, earth, sand, wood, stone — two takes each),
  `doorOpen_2.ogg` creaks on every door warp, `creak2.ogg` is the cellar
  chest's groan, `handleCoins.ogg` chimes when a keepsake changes hands,
  and `bookFlip2.ogg` turns the dialogue pages.
- `kenney/audio/ui-audio/` — *UI Audio*
  (https://kenney.nl/assets/ui-audio): `click1.ogg` is the menu blip.
- `kenney/audio/music-jingles/` — *Music Jingles*
  (https://kenney.nl/assets/music-jingles): `jingles_STEEL07.ogg` sparkles
  when a wild rune joins the grimoire, `jingles_NES00.ogg` gleams for a
  found runestone. The other 83 fanfares stay on the shelf.

### Ninja Adventure audio

`ninja_adventure/pack/Audio/` vendors the **Ninja Adventure asset pack**'s
entire soundtrack and SFX library (**CC0**, credited above): `Musics/` holds
all 42 tracks (one of them, "22 - Dream", already plays as the night theme —
see below), and `Sounds/` the 140+ effects in their category folders (Menu,
Bonus, Magic & Skill, Ambient, …). Everything is unmodified except
`Sounds/Ambient/`, whose long WAV loops (rain, river, storm, waves, wind)
were re-encoded to Ogg Vorbis q4 to keep the repo light. Three of those now
play as the daytime weather beds (`DAY_BEDS` in `src/main.rs`): `Wind2.ogg`
through the Whispering Woods' canopy, `Rain.ogg` under Silverford's constant
rain, `Wind.ogg` off the misty Hearthspire road — each looped softly under
its zone's chiptune until dark, when the night beds take over.

### Night ambiences

`audio/music/night/` — one calm nature bed per overworld zone, swapped in for
the daytime loop after dark (`App::is_night()`), same filename per zone. All
**CC0**, from OpenGameArt:

- `theme.ogg` — the calm melody laid *over* every zone's nature bed after
  dark: "22 - Dream" from the **Ninja Adventure asset pack** by **Pixel-boy
  and AAA** (https://pixel-boy.itch.io/ninja-adventure-asset-pack, **CC0**),
  unmodified — so night has real music rather than crickets alone.
- `emberwick.ogg` — "Crickets Ambient Noise - loopable" by **Wolfgang_**
  (https://opengameart.org/content/crickets-ambient-noise-loopable).
- `whispering-woods.ogg` — "Swamp Environment Audio" by **LokiF**
  (https://opengameart.org/content/swamp-environment-audio): frogs and water,
  for the Woods at night.
- `silverford.ogg` — "AMB Rain Loop 1" by **Kresiek the Furry**
  (https://opengameart.org/content/amb-rain-loop-1), under Silverford's rain.
- `hearthspire.ogg` — "Winter Wind" by **wipics**
  (https://opengameart.org/content/winter-wind), off the mountain road.

### Hearth

`audio/music/fireplace.ogg` — the soft crackle looped inside the lived-in
rooms (the bakery, the cottages, the workshop, the Great Library — see
`zones::has_hearth`), so stepping indoors trades the zone music for a fire
instead of dead silence. "Fireplace Sound loop" by **PagDev**
(https://opengameart.org/content/fireplace-sound-loop), **CC0**; converted
to mono OGG and trimmed in level, otherwise unmodified.

### Owl

`audio/sfx/owl.ogg` — the lone night owl, hooted at random intervals under the
night ambience. A brown hawk-owl (*Ninox scutulata*) call by **Shyamal**, from
Wikimedia Commons
(https://commons.wikimedia.org/wiki/File:Brown_hawk_owl.ogg), licensed
**CC BY 4.0** (https://creativecommons.org/licenses/by/4.0). This is the one
non-CC0 asset in the repo: attribution-only, so redistributable with this
credit; no genuinely CC0 owl hoot turned up.

Loaded and played from `src/main.rs` only, via macroquad's `audio` feature —
the lib and tests stay sound-free.
