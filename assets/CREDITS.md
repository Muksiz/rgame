# Asset credits

`atlas.png` is baked by `tools/bake_atlas.py` from two spritesheets by
**Kenney** (www.kenney.nl), both licensed **Creative Commons Zero (CC0)**:

- *Roguelike/RPG pack* — `kenney/roguelikeSheet_transparent.png`
  https://kenney.nl/assets/roguelike-rpg-pack
- *Roguelike Characters* — `kenney/roguelikeChar_transparent.png`
  https://kenney.nl/assets/roguelike-characters

The critter and Ferris sprites in the atlas are original pixel art for this
project (same CC0 spirit — do what you like with them).

To rebake after editing the script:

```sh
python3 tools/bake_atlas.py assets/kenney/roguelikeSheet_transparent.png assets/kenney/roguelikeChar_transparent.png
```
