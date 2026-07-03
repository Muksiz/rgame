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

The critter, Ferris, bookshelf, chest, runestone and moon-mint sprites in
the atlas are original pixel art for this project (same CC0 spirit — do
what you like with them).

To rebake after editing the script:

```sh
python3 tools/bake_atlas.py assets/kenney/roguelikeSheet_transparent.png assets/kenney/roguelikeChar_transparent.png
```

## Audio

`assets/audio/` holds every sound the game plays, all by **Juhani Junkala**
(https://juhanijunkala.com), **CC0**:

- `audio/music/` — one loop per overworld zone, from *Chiptune Adventures*
  (https://opengameart.org/content/4-chiptunes-adventure): `emberwick.ogg`
  ("Stage 1"), `whispering-woods.ogg` ("Stage 2"), `silverford.ogg` ("Stage
  Select"), `hearthspire.ogg` ("Boss Fight" — the Hearthspire Approach is
  guarded by the Stone Golem, so the fit was too good to pass up). Interiors
  (zone 4+) stay quiet. `title.ogg` is the "Title Screen" track from *5
  Chiptunes (Action)* (https://opengameart.org/content/5-chiptunes-action, same
  artist), looping through the title and char-select screens.
- `audio/sfx/` — `cast.ogg`, `pass.ogg` and `fizzle.ogg`, trimmed from *The
  Essential Retro Video Game Sound Effects Collection*
  (https://opengameart.org/content/512-sound-effects-8-bit-style): a menu
  blip, a fanfare and a soft error tone.

All three packs are vendored only in part — see the "Free asset shelf" section
of `CLAUDE.md` for the rest (SFX categories, ambience beds) that remain
unclaimed for a future pass. Loaded and played from `src/main.rs` only, via
macroquad's `audio` feature — the lib and tests stay sound-free.
