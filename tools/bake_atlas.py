#!/usr/bin/env python3
"""Bake assets/atlas.png for the graphical (Macroquad) frontend.

Sources: Kenney's CC0 "Roguelike/RPG pack" and "Roguelike Characters"
(https://kenney.nl), both 16x16 tiles with a 1px margin. This script picks the
handful of sprites the game needs, composes/tints the characters, adds a few
hand-pixeled critters, and packs everything into one margin-free atlas whose
cell order matches the constants in src/gfx/atlas.rs.

Usage: python3 tools/bake_atlas.py <roguelikeSheet_transparent.png> <roguelikeChar_transparent.png>
"""

import sys

from PIL import Image

TILE = 16
COLS = 16  # atlas cells per row


def cell(sheet, c, r):
    """Cut one 16x16 tile from a Kenney sheet (1px margins)."""
    x, y = c * 17, r * 17
    return sheet.crop((x, y, x + TILE, y + TILE))


def over(base, top):
    out = base.copy()
    out.alpha_composite(top)
    return out


def tint(sprite, rgb):
    """Recolor a sprite by luminance so it matches the NPC's terminal color."""
    out = Image.new("RGBA", sprite.size, (0, 0, 0, 0))
    for xy in [(x, y) for y in range(sprite.height) for x in range(sprite.width)]:
        r, g, b, a = sprite.getpixel(xy)
        if a == 0:
            continue
        lum = (0.3 * r + 0.6 * g + 0.1 * b) / 200.0
        out.putpixel(
            xy,
            (
                min(255, int(rgb[0] * lum)),
                min(255, int(rgb[1] * lum)),
                min(255, int(rgb[2] * lum)),
                a,
            ),
        )
    return out


def from_art(art, palette):
    """A 16x16 sprite from 16 rows of 16 palette characters ('.' = clear)."""
    rows = [line for line in art.strip("\n").split("\n")]
    img = Image.new("RGBA", (TILE, TILE), (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch != ".":
                img.putpixel((x, y), palette[ch])
    return img


CHICKEN = """
................
................
................
......ww........
.....wwww.......
.....owyy.......
......ww........
....wwwww.......
...wwwwwww......
...wwwwwww......
...wwwwwww......
....wwwww.......
.....www........
......y.y.......
.....y..y.......
................
"""

SHEEP = """
................
................
................
...wwwwwwww.....
..wwwwwwwwww....
.wwwwwwwwwwww...
.wwwwwwwwwwkk...
.wwwwwwwwwkkkk..
.wwwwwwwwwkkkk..
.wwwwwwwwwwkk...
..wwwwwwwwww....
...wwwwwww......
....k...k.......
....k...k.......
................
................
"""

FROG = """
................
................
................
................
................
....gg...gg.....
....ggggggg.....
...ggkggkgg.....
...ggggggggg....
..ggggggggggg...
..gGGGGGGGGgg...
..ggggggggggg...
...gg.....gg....
..gg.......gg...
................
................
"""

MOTH = """
................
................
................
................
....m.....m.....
...mmm...mmm....
..mmmmm.mmmmm...
..mmmmmammmmm...
..mmmmaaammmm...
...mmmaaammm....
....mmaaamm.....
......aaa.......
......a.a.......
................
................
................
"""

FERRIS = """
................
................
...oo......oo...
..o.oo....oo.o..
..oo........oo..
....oooooooo....
...oooooooooo...
..oooooooooooo..
..ooko.oo.okoo..
..oooooooooooo..
..oooooooooooo..
...oooooooooo...
...o.o.o..o.o...
..o..o.o..o..o..
................
................
"""

PALETTE = {
    "w": (240, 238, 230, 255),
    "o": (222, 120, 80, 255),
    "y": (240, 200, 60, 255),
    "k": (40, 36, 40, 255),
    "g": (110, 185, 90, 255),
    "G": (200, 230, 170, 255),
    "m": (214, 208, 232, 255),
    "a": (150, 140, 170, 255),
}

# NPC terminal colors, in quest order 1..12 (from src/world/zones.rs).
NPC_COLORS = [
    (216, 186, 130),
    (234, 156, 146),
    (150, 184, 214),
    (152, 214, 122),
    (206, 130, 170),
    (196, 186, 156),
    (126, 168, 190),
    (142, 196, 196),
    (176, 156, 208),
    (188, 176, 146),
    (168, 168, 180),
    (226, 204, 152),
]
# Body/hair variety per NPC: (body col, body row, hair col, hair row) on the
# characters sheet. Bodies live at cols 0-1; hair/hats at cols 19-22 & 28-31.
NPC_LOOKS = [
    (0, 0, 20, 0),
    (1, 0, 21, 8),
    (0, 1, 28, 0),
    (1, 1, 20, 5),
    (0, 2, 21, 9),
    (1, 2, 28, 1),
    (0, 0, 21, 5),
    (1, 1, 20, 8),
    (0, 2, 28, 5),
    (1, 0, 20, 9),
    (0, 3, None, None),  # the golem gets the sturdy frame, no hat
    (0, 1, 21, 0),
]


def main(sheet_path, chars_path):
    sheet = Image.open(sheet_path).convert("RGBA")
    chars = Image.open(chars_path).convert("RGBA")

    shirt = cell(chars, 10, 0)  # plain tunic, used as the tint base

    def npc(i):
        bc, br, hc, hr = NPC_LOOKS[i]
        body = cell(chars, bc, br)
        spr = over(body, tint(shirt, NPC_COLORS[i]))
        if i == 10:  # golem: tint the whole body stony grey
            spr = tint(body, (168, 168, 180))
        elif hc is not None:
            spr = over(spr, cell(chars, hc, hr))
        return spr

    player = over(over(cell(chars, 0, 0), cell(chars, 10, 0)), cell(chars, 20, 0))

    cells = [
        # terrain bases (opaque)
        ("GRASS", cell(sheet, 5, 0)),
        ("GRASS_ALT", cell(sheet, 5, 1)),
        ("PATH", cell(sheet, 6, 0)),
        ("PATH_ALT", cell(sheet, 6, 1)),
        ("SAND", cell(sheet, 8, 0)),
        ("WATER_A", cell(sheet, 0, 0)),
        ("WATER_B", cell(sheet, 1, 0)),
        ("FLOOR", cell(sheet, 34, 15)),
        ("STONE", cell(sheet, 4, 26)),
        ("FLOWER_ORANGE", over(cell(sheet, 5, 0), cell(sheet, 29, 9))),
        ("FLOWER_WHITE", over(cell(sheet, 5, 0), cell(sheet, 31, 9))),
        ("FLOWER_BLUE", over(cell(sheet, 5, 0), cell(sheet, 30, 9))),
        # overlays (transparent, drawn on a base)
        ("TREE_GREEN", cell(sheet, 13, 9)),
        ("TREE_ORANGE", cell(sheet, 14, 9)),
        ("PINE", cell(sheet, 16, 9)),
        ("BUSH", cell(sheet, 25, 9)),
        ("BERRY_BUSH", cell(sheet, 24, 9)),
        ("SPROUT", cell(sheet, 22, 10)),
        ("SPROUT_ALT", cell(sheet, 22, 11)),
        ("LILY", cell(sheet, 25, 11)),
        ("ROCK_GREY", cell(sheet, 54, 21)),
        ("ROCK_BROWN", cell(sheet, 54, 19)),
        ("FENCE", cell(sheet, 52, 23)),
        ("GATE", cell(sheet, 46, 23)),
        ("SIGN", cell(sheet, 19, 0)),
        ("DOOR", cell(sheet, 28, 5)),
        ("BRIDGE", cell(sheet, 34, 17)),
        ("WALL", cell(sheet, 14, 15)),
        ("ROOF", cell(sheet, 25, 22)),
        ("CAMPFIRE_A", cell(sheet, 14, 8)),
        ("CAMPFIRE_B", cell(sheet, 15, 8)),
        ("TORCH_UNLIT", cell(sheet, 16, 7)),
        ("TORCH_LIT_A", cell(sheet, 17, 7)),
        ("TORCH_LIT_B", cell(sheet, 17, 8)),
        # people & friends
        ("PLAYER", player),
        *[(f"NPC_{i + 1}", npc(i)) for i in range(12)],
        ("CHICKEN", from_art(CHICKEN, PALETTE)),
        ("SHEEP", from_art(SHEEP, PALETTE)),
        ("FROG", from_art(FROG, PALETTE)),
        ("MOTH", from_art(MOTH, PALETTE)),
        ("FERRIS", from_art(FERRIS, PALETTE)),
    ]

    rows = (len(cells) + COLS - 1) // COLS
    atlas = Image.new("RGBA", (COLS * TILE, rows * TILE), (0, 0, 0, 0))
    for i, (_, img) in enumerate(cells):
        atlas.paste(img, ((i % COLS) * TILE, (i // COLS) * TILE))
    atlas.save("assets/atlas.png")

    print(f"atlas: {atlas.width}x{atlas.height}, {len(cells)} cells")
    for i, (name, _) in enumerate(cells):
        print(f"pub const {name}: u16 = {i};")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
