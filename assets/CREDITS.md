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
