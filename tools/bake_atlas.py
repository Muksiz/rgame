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
from pathlib import Path

from PIL import Image

TILE = 16
COLS = 16  # atlas cells per row

NA_DIR = Path(__file__).resolve().parent.parent / "assets" / "ninja_adventure"
ZL_DIR = Path(__file__).resolve().parent.parent / "assets" / "zelda_like"

# The cast, in atlas order. The order is historical (player, quest NPCs 1..12,
# named side folk, then three spares); who actually *wears* each sprite is
# decided in code — the char-select roster in src/gfx/atlas.rs and npc_sprite
# in src/gfx/scene.rs — so trust the per-line notes, not the position. Each
# Ninja Adventure strip holds one 16x16 frame per facing, in the order down,
# up, left, right — which is also the order the cells are baked in.
CAST = [
    ("BOY", "Boy"),  # char-select: "the young traveller"
    ("MASTER", "Master"),  # 1 Elder Rowan
    ("OLDMAN2", "OldMan2"),  # 2 Baker Poppy (the beret! the stripes!)
    ("VILLAGER_M", "Villager"),  # 3 Well-keeper Bram
    ("CHILD", "Child"),  # char-select: "the curious sprout"
    ("VILLAGE6", "Village6"),  # 5 Forager Maren (red headscarf, warm smile)
    ("MANGREEN", "ManGreen"),  # char-select: "the greenwood ranger"
    ("VILLAGER2", "Villager2"),  # 7 Ferryman Wick
    ("EGGBOY", "EggBoy"),  # 8 Fisher Juniper (bucket hat)
    ("MONK", "Monk"),  # 9 Hermit Morrow
    ("INSPECTOR", "Inspector"),  # 10 Archivist Elm (spectacles)
    ("STATUE", "Statue"),  # 11 The Stone Golem
    ("OLDMAN3", "OldMan3"),  # 12 Sage Alderly
    ("OLDWOMAN", "OldWoman"),  # Granny Sorrel
    ("GREENMAN", "Greenman"),  # Old Nettle
    ("HUNTER", "Hunter"),  # Carpenter Alder
    ("WOMAN", "Woman"),  # char-select: "the roaming herbalist"
    ("NOBLE", "Noble"),  # Under-librarian Twill
    # The three NPCs whose sprites the char-select roster took over now wear
    # these spare villager looks (see npc_sprite in src/gfx/scene.rs).
    ("VILLAGER3", "Villager3"),  # Shepherd Ambrose
    ("VILLAGER4", "Villager4"),  # Wren
    ("PRINCESS", "Princess"),  # Hen-keeper Tilly
]

# The beginner-quest expansion's cast, baked as its own block at the tail of
# the atlas (see the append-only rule below) rather than folded into CAST, so
# none of the original members' ids move. Same four-facing Ninja Adventure
# strip format; these eleven only ship a SpriteSheet.png (no separate
# Idle.png), which na_idles() already falls back to.
NEW_CAST = [
    ("TANSY", "EggGirl"),  # 2 Tansy
    ("FITCH", "Villager5"),  # 4 Fitch
    ("HOBB", "OldMan"),  # 5 Hobb
    ("REED", "Tengu"),  # 4 Reed (Jani didn't like the Shaman look)
    ("PIP", "Villager6"),  # 8 Pip (the Cavegirl2 look got a playtest thumbs-down)
    ("SALLOW", "ShamanLion"),  # 10 Sallow (redesigned from Briar by playtest note)
    ("YEW", "Monk2"),  # 12 Yew
    ("SABLE", "Caveman"),  # 14 Sable
    ("FENN", "Eskimo"),  # 15 Fenn
    ("SIL", "Caveman2"),  # 18 Sil
    ("FAYE", "Sultan2"),  # 23 Faye
]

# Fernshade's residents (the hamlet in the Whispering Woods) — baked as
# their own tail block at the very end of the atlas, so nothing shifts.
FERN_CAST = [
    ("IVY", "Vampire"),  # Grandmother Ivy, Pip's gran
    ("MOSS", "SorcererOrange"),  # Lamplighter Moss
    ("BRACKEN", "FighterWhite"),  # Innkeep Bracken of the Glowworm
]

# The wild runes' visible forms (Ninja Adventure monsters, CC0), one per rune
# in id order 1-16 — the little creature that bobs on the encounter screen.
# Each is chosen to fit its rune's stir line in src/content/wilds.rs.
WILD_FORMS = [
    "Flam",  # 1 the Bang Rune — excitable, pops out of the grass
    "Mole",  # 2 the Mut Rune — plants itself, refuses to budge
    "ButterflyBlue",  # 3 the Arrow Rune — slender, flies past and circles
    "Spirit",  # 4 the Semicolon Sprite — a tiny hanging wisp
    "Racoon",  # 5 the Move Rune — scurries off with the acorn
    "Slime",  # 6 the Clone Rune — splits into two, delighted
    "KappaGreen",  # 7 the Borrow Rune — polite, asks to hold your satchel
    "Snake",  # 8 the Slice Rune — thin, only ever a part of the whole
    "Mollusc",  # 9 the Struct Rune — named parts under one neat roof
    "Mouse",  # 10 the Dot Rune — taps from crate to crate, precisely
    "Fish",  # 11 the Update Rune — last year's scales, one part new
    "Owl",  # 12 the Derive Rune — reads everything aloud, unprompted
    "KappaRed",  # 13 the Method Rune — bows; it belongs to something
    "Larva",  # 14 the Winding Rune — winds itself tighter and tighter
    "Eye",  # 15 the Summoning Rune — summoned, apparently, by no one
    "Axolot",  # 16 the Mirror Rune — admiring its own reflection
]


def na_tile(sheet_name, c, r):
    """One 16x16 cell from a Ninja Adventure tileset (margin-free grid)."""
    sheet = Image.open(NA_DIR / "tilesets" / f"{sheet_name}.png").convert("RGBA")
    return sheet.crop((c * TILE, r * TILE, c * TILE + TILE, r * TILE + TILE))


def zl_prefab(name, px, py, tw, th, keep=None, paste=None):
    """A multi-tile building prefab from the Zelda-like Overworld sheet
    (ArMM1998, CC0), sliced into tw x th atlas cells row-major; only the
    first cell gets the named constant, the rest follow consecutively.

    The sheet's prefabs aren't grid-aligned and sometimes touch their
    neighbors, so the crop starts at an arbitrary pixel origin (px, py);
    `keep` (one rel-pixel box (x0, y0, x1, y1), or a list of them) erases
    every pixel not inside at least one box, and `paste` composites another
    region of the sheet (sx, sy, w, h, rx, ry) onto the crop — how the
    doorless barn gets its arched door.
    """
    sheet = Image.open(ZL_DIR / "Overworld.png").convert("RGBA")
    img = sheet.crop((px, py, px + tw * TILE, py + th * TILE)).copy()
    if keep:
        boxes = keep if isinstance(keep, list) else [keep]
        for y in range(img.height):
            for x in range(img.width):
                if not any(
                    x0 <= x < x1 and y0 <= y < y1 for (x0, y0, x1, y1) in boxes
                ):
                    img.putpixel((x, y), (0, 0, 0, 0))
    if paste:
        sx, sy, w, h, rx, ry = paste
        img.alpha_composite(sheet.crop((sx, sy, sx + w, sy + h)), (rx, ry))
    return [
        (
            name if r == 0 and c == 0 else None,
            img.crop((c * TILE, r * TILE, c * TILE + TILE, r * TILE + TILE)),
        )
        for r in range(th)
        for c in range(tw)
    ]


def na_prefab(sheet_name, name, px, py, tw, th, paste=None, shift=(0, 0)):
    """A multi-tile prefab from a Ninja Adventure tileset (margin-free grid),
    sliced into tw x th atlas cells row-major, same contract as zl_prefab.

    `paste` composites another region of the same sheet (sx, sy, w, h, rx, ry)
    onto the crop — how the flavor houses get their doors shut. `shift`
    offsets the art inside the cell box (for centering a crop that isn't a
    whole multiple of 16 wide, e.g. the big nature trees).
    """
    sheet = Image.open(NA_DIR / "tilesets" / f"{sheet_name}.png").convert("RGBA")
    img = Image.new("RGBA", (tw * TILE, th * TILE), (0, 0, 0, 0))
    img.alpha_composite(sheet.crop((px, py, px + tw * TILE, py + th * TILE)), shift)
    if paste:
        sx, sy, w, h, rx, ry = paste
        img.alpha_composite(sheet.crop((sx, sy, sx + w, sy + h)), (rx, ry))
    return [
        (
            name if r == 0 and c == 0 else None,
            img.crop((c * TILE, r * TILE, c * TILE + TILE, r * TILE + TILE)),
        )
        for r in range(th)
        for c in range(tw)
    ]


def img_prefab(path, name, px, py, tw, th):
    """A multi-tile prefab cut straight from any RGBA image file (the raw
    `pack/` material), sliced into tw x th atlas cells row-major, same
    contract as zl_prefab/na_prefab."""
    sheet = Image.open(path).convert("RGBA")
    img = sheet.crop((px, py, px + tw * TILE, py + th * TILE))
    return [
        (
            name if r == 0 and c == 0 else None,
            img.crop((c * TILE, r * TILE, c * TILE + TILE, r * TILE + TILE)),
        )
        for r in range(th)
        for c in range(tw)
    ]


def na_idles(folder):
    """The four 16x16 idle frames (down, up, left, right) of a cast member.

    Most characters ship an `Idle.png` strip; a couple only have the compact
    `SpriteSheet.png`, whose top row is the same four facings.
    """
    idle = NA_DIR / folder / "Idle.png"
    sheet = Image.open(idle if idle.exists() else NA_DIR / folder / "SpriteSheet.png")
    sheet = sheet.convert("RGBA")
    return [sheet.crop((d * TILE, 0, d * TILE + TILE, TILE)) for d in range(4)]


def na_monster(folder):
    """Two down-facing frames of a Ninja Adventure monster (columns are
    facings, rows the walk cycle; rows 0 and 2 make a gentle bob). The
    spritesheet's filename varies per monster, but it's always the one
    file that isn't the Faceset."""
    d = NA_DIR / "pack" / "Actor" / "Monster" / folder
    (sheet_path,) = [p for p in sorted(d.iterdir()) if p.name != "Faceset.png"]
    sheet = Image.open(sheet_path).convert("RGBA")
    return [sheet.crop((0, r * TILE, TILE, r * TILE + TILE)) for r in (0, 2)]


def na_walks(folder):
    """Two stride frames per facing (down, up, left, right) from Walk.png,
    where columns are facings and rows are the four walk-cycle frames."""
    sheet = Image.open(NA_DIR / folder / "Walk.png").convert("RGBA")
    frames = []
    for d in range(4):
        for row in (1, 3):  # the two full-stride frames of the 4-frame cycle
            frames.append(
                sheet.crop((d * TILE, row * TILE, d * TILE + TILE, row * TILE + TILE))
            )
    return frames


def na_steps(folder):
    """A two-frame stride per facing (same layout as na_walks) for cast
    members that ship only the compact SpriteSheet.png: its second row is a
    step pose per facing, paired here with the idle row above it."""
    sheet = Image.open(NA_DIR / folder / "SpriteSheet.png").convert("RGBA")
    frames = []
    for d in range(4):
        for row in (1, 0):
            frames.append(
                sheet.crop((d * TILE, row * TILE, d * TILE + TILE, row * TILE + TILE))
            )
    return frames


def cell(sheet, c, r):
    """Cut one 16x16 tile from a Kenney sheet (1px margins)."""
    x, y = c * 17, r * 17
    return sheet.crop((x, y, x + TILE, y + TILE))


def fence_corner(sheet, east, south):
    """A fence corner joining a horizontal run (toward east/west) with a
    vertical one (toward south/north): half of each rail sprite so both
    reach their tile edge, and the stout post pasted over the joint."""
    h = cell(sheet, 52, 23)  # the horizontal rails (FENCE)
    v = h.transpose(Image.ROTATE_90)  # the vertical rails (FENCE_V)
    out = Image.new("RGBA", (TILE, TILE), (0, 0, 0, 0))
    half = TILE // 2
    hx = 0 if not east else half
    out.alpha_composite(h.crop((hx, 0, hx + half, TILE)), (hx, 0))
    vy = 0 if not south else half
    out.alpha_composite(v.crop((0, vy, TILE, vy + half)), (0, vy))
    out.alpha_composite(cell(sheet, 45, 23))  # FENCE_POST covers the joint
    return out


def kenney_sheet(sheet_path, name):
    """A sibling Kenney sheet living next to the main one (same 1px margins)."""
    return Image.open(Path(sheet_path).parent / name).convert("RGBA")


def over(base, top):
    out = base.copy()
    out.alpha_composite(top)
    return out


def tint(sprite, rgb):
    """Recolor a sprite by luminance to give each NPC their own tunic color."""
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


def dim(sprite, mul):
    """Darken a sprite channel-by-channel (mul = (r, g, b) factors) — how the
    deep-woods floor gets its dusk without a second source tile."""
    out = sprite.copy()
    px = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = px[x, y]
            px[x, y] = (int(r * mul[0]), int(g * mul[1]), int(b * mul[2]), a)
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


def from_art_prefab(name, art, palette, tw, th):
    """A hand-pixeled multi-tile prefab: tw*16 columns by th*16 rows of
    palette characters, sliced into atlas cells row-major (same contract as
    zl_prefab, so MapBuilder::prefab can place it)."""
    rows = [line for line in art.strip("\n").split("\n")]
    img = Image.new("RGBA", (tw * TILE, th * TILE), (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch != ".":
                img.putpixel((x, y), palette[ch])
    return [
        (
            name if r == 0 and c == 0 else None,
            img.crop((c * TILE, r * TILE, c * TILE + TILE, r * TILE + TILE)),
        )
        for r in range(th)
        for c in range(tw)
    ]


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

STUMP = """
................
................
................
................
................
................
....LLLLLL......
...LLbbbbLL.....
...LbLLLLbL.....
...bbbbbbbb.....
...bbbbbbbb.....
...bbbbbbbb.....
..bbbbbbbbbb....
................
................
................
"""

BUTTERFLY_A = """
................
................
................
................
................
....pp....pp....
...pppp..pppp...
...ppppkkpppp...
....pppkkppp....
.....pp..pp.....
................
................
................
................
................
................
"""

BUTTERFLY_B = """
................
................
................
................
................
......p..p......
.....pp..pp.....
.....ppkkpp.....
......pkkp......
................
................
................
................
................
................
................
"""

BIRD_A = """
................
................
................
................
................
................
...k......k.....
..kkk....kkk....
....kkkkkk......
......kk........
................
................
................
................
................
................
"""

BIRD_B = """
................
................
................
................
................
................
................
................
..kkkkkkkkkk....
....kk..kk......
................
................
................
................
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

CAT = """
................
................
................
....t...t.......
....tt.tt.......
....ttttt.......
....tktkt.......
....ttttt.......
....twwtt.tt....
...ttwwttttt....
...tttttttt.....
...tttttttt.....
....tt..tt......
................
................
................
"""

# The small-lives expansion: a village dog for Emberwick, a wild boar
# rooting through the Whispering Woods, a duck for Silverford's banks and
# a pack donkey on the Hearthspire road — hand-pixeled like their fellows.
DOG = """
................
................
................
...b......b.....
...bt....tb.....
...bttttttb.....
...ttkttktt.....
...tttttttt.....
....ttkktt......
.....tttt.......
....tttttt..b...
...tttttttt.b...
...ttttttttbb...
...tttttttt.....
....tt..tt......
................
"""

BOAR = """
................
................
................
................
................
....bbbbbbb.....
...bbbbbbbbb....
..bbbbbbbbbbb...
.bkbbbbbbbbbb...
.bbbbbbbbbbbb...
.tbbbbbbbbbbb...
.wtbbbbbbbbb....
....bb...bb.....
....kk...kk.....
................
................
"""

DUCK = """
................
................
................
................
.....ww.........
....wwww........
....wkww........
..yywwww........
....wwwww.......
....wwwwww......
...wwwwwwww.....
...wwwwwwww.....
....wwwwww......
.....y..y.......
................
................
"""

DONKEY = """
................
................
...S..S.........
...s..s.........
...ssss.........
...sksks........
...ssss.........
...Lsss.........
...ssssssssss...
...ssssssssss...
...sssssssssss..
....ss....ss....
....ss....ss....
....SS....SS....
................
................
"""

BOOKSHELF = """
LLLLLLLLLLLLLLLL
LbbbbbbbbbbbbbbL
Lb1e21g3e12g3ebL
Lb1e21g3e12g3ebL
Lb1e21g3e12g3ebL
LbbbbbbbbbbbbbbL
Lbg31e2g13ge21bL
Lbg31e2g13ge21bL
Lbg31e2g13ge21bL
LbbbbbbbbbbbbbbL
Lb2ge13g21e3g1bL
Lb2ge13g21e3g1bL
Lb2ge13g21e3g1bL
LbbbbbbbbbbbbbbL
LLLLLLLLLLLLLLLL
LLLLLLLLLLLLLLLL
"""

CHEST = """
................
................
................
....LLLLLLLL....
...LbbbbbbbbL...
...LbbbbbbbbL...
...LLLLLLLLLL...
...LbbbyybbbL...
...Lbbbyy1bbL...
...LbbbbbbbbL...
...LbbbbbbbbL...
...LLLLLLLLLL...
....k......k....
................
................
................
"""

RUNESTONE = """
................
................
................
......SS........
.....sssS.......
....sssssS......
....sscssS......
...ssscsssS.....
...sscscssS.....
...ssscsssS.....
...sssscssS.....
..ssssscsssS....
..sssssssssS....
..sSSSSSSSSS....
................
................
"""

HERB = """
................
................
................
................
................
......G..w......
....G.g.wGw..G..
...GgGg..w..gG..
....ggGg...Gg...
..G..gGg..gg....
...gg.gg.gG.....
.....g.ggg......
......ggg.......
.......g........
................
................
"""

WINDOW = """
bbbbbbbbbbbbbbbb
biiiiiiibiiiiiib
biiiiiiibiiiiiib
biiiiiiibiiiiiib
biiiiiiibiiiiiib
bbbbbbbbbbbbbbbb
bjjjjjjjbjjjjjjb
bjjjjjjjbjjjjjjb
bjjjjjjjbjjjjjjb
bjjjjjjjbjjjjjjb
biiiiiiibiiiiiib
bbbbbbbbbbbbbbbb
................
................
................
................
"""

PAINTING = """
................
.LLLLLLLLLLLLLL.
.LyLLLLLLLLLLyL.
.L2222222222y2L.
.L22222y222222L.
.L222222222222L.
.Lg2222222g2ggL.
.Lggg3ggg3gggL..
.Lg3gg3gg3gg3L..
.LgggggggggggL..
.LyLLLLLLLLLLyL.
.LLLLLLLLLLLLLL.
................
................
................
................
"""

PLANT = """
................
......G.G.......
.....GgGgG......
....Gg3g3gG.....
...Gg3gGg3gG....
....Gg3g3gG.....
.....GgGgG......
.......g........
.......g........
......ooo.......
.....ooooo......
.....obbbo......
.....ooooo......
......ooo.......
................
................
"""

PEDESTAL = """
................
.......c........
......ccc.......
......ccc.......
.....ccccc......
......ccc.......
.......c........
.....ssssss.....
....ssSSSSss....
....sSSSSSSs....
....sSSSSSSs....
...ssssssssss...
...sSSSSSSSSs...
..ssssssssssss..
..sSSSSSSSSSSs..
................
"""

VOID = """
vvvvvvvvvvvvvvvv
vvvvvvvvvvvvvvvv
vvvvvVvvvvvvvvvv
vvvvvvvvvvvvvvvv
vvvvvvvvvvvvVvvv
vvvvvvvvvvvvvvvv
vvVvvvvvvvvvvvvv
vvvvvvvvvvvvvvvv
vvvvvvvvvVvvvvvv
vvvvvvvvvvvvvvvv
vvvvVvvvvvvvvvvv
vvvvvvvvvvvvvvvv
vvvvvvvvvvvvvVvv
vvvvvvvvvvvvvvvv
vvvvvvvVvvvvvvvv
vvvvvvvvvvvvvvvv
"""

ROD_CAST = """
.....b..........
.....bb.........
......bb........
.......bb.......
........bb......
.........b......
.........k......
.........k......
.........k......
.........k......
........1w......
........11......
................
................
................
................
"""

# ── the companion at your heels: a very small crab, pixeled after Ferris ──
# (rustacean.net's flat art: body #F74C00, accents #A52B00 — the "f"/"F"
# palette keys below. Round-domed shell, big glinting eyes, little claws.)

CRAB_IDLE = """
................
................
................
................
......ffff......
....ffffffff....
..ffffffffffff..
..ffwkffffwkff..
..ffkkffffkkff..
..fffFffffFfff..
..ffffFFFFffff..
.ff.ffffffff.ff.
.ff.F.F..F.F.ff.
...F........F...
................
................
"""

CRAB_WAVE = """
................
................
...ff......ff...
..f.ff....ff.f..
......ffff......
....ffffffff....
..ffffffffffff..
..ffwkffffwkff..
..ffkkffffkkff..
..fffFffffFfff..
..ffffFFFFffff..
...ffffffffff...
..F.F.F..F.F.F..
................
................
................
"""

CRAB_WALK_A = """
................
................
................
................
......ffff......
....ffffffff....
..ffffffffffff..
..ffwkffffwkff..
..ffkkffffkkff..
..fffFffffFfff..
..ffffFFFFffff..
.ff.ffffffff.ff.
.ff.F..F..F..ff.
...F........F...
................
................
"""

CRAB_WALK_B = """
................
................
................
................
......ffff......
....ffffffff....
..ffffffffffff..
..ffwkffffwkff..
..ffkkffffkkff..
..fffFffffFfff..
..ffffFFFFffff..
.ff.ffffffff.ff.
.ff..F..F..F.ff.
..F..........F..
................
................
"""

CRAB_CURL = """
................
................
................
................
................
................
................
................
.....ffffff.....
....ffffffff....
...ffffffffff...
...ffFffffFff...
...ffffffffff...
....F......F....
................
................
"""

CRAB_PEEK = """
................
................
................
................
................
................
................
................
....wk....wk....
....kk....kk....
.....f....f.....
.....f....f.....
....ffffffff....
................
................
................
"""

# The village well (playtest note: the old one was a ring of rock around a
# water tile — "a block of water"). A proper roofed draw-well: gable, posts,
# rope and bucket over a round stone rim, hand-pixeled in the house style.
WELL_ART = """
..............bbbb..............
............bbttttbb............
..........bbttttttttbb..........
........bbttttttttttttbb........
......bbttttttttttttttttbb......
....bbttttttttttttttttttttbb....
...bttttttttttttttttttttttttb...
...bbbbbbbbbbbbbbbbbbbbbbbbbb...
....bt.........ee.........tb....
....bt.........ee.........tb....
....bt.........ee.........tb....
....bt.........ee.........tb....
....bt........bttb........tb....
....bt........bttb........tb....
....bt........bbbb........tb....
....bt....................tb....
....bt....ssssssssss......tb....
....bt..ssSSSSSSSSSSss....tb....
....btssSVVVVVVVVVVVVSss..tb....
....btsSVVVVVVVVVVVVVVSs..tb....
...ssssSVVVV22222VVVVVSssss.....
..sSSssSVVV2jj22j22VVVSssSSs....
..ssssssSVVV22222VVVVSssssss....
..sSSSssSSVVVVVVVVVSSssSSSSs....
..ssSSssssSSSSSSSSSssssSSSss....
..sSSSsssssssssssssssssSSSSs....
..ssssSSssssSSSssssSSsssssss....
..sSSsssssSSsssSSsssssSSSSss....
...ssssssssssssssssssssssss.....
....ssssssssssssssssssssss......
................................
................................
"""

# A cave mouth: a dark arched opening set into a rocky face, so the Echo Cave
# entrance reads as a cave instead of a gap in a pile of cliffs. Walkable — the
# warp sits on it — so the arch darkens to near-black at its heart.
CAVE_MOUTH_ART = """
..SSSSSSSSSSSS..
.SSssssssssssSS.
SSskkkkkkkkkksSS
SskkkkkkkkkkkksS
SskkkvvvvvvkkksS
SskkvvvvvvvvkksS
SskvvvvvvvvvvksS
SskvvvvvvvvvvksS
SskvvvvvvvvvvksS
SskkvvvvvvvvkksS
SskkkvvvvvvkkksS
SSskkkkkkkkkksSS
.SSssssssssssSS.
..SSSSSSSSSSSS..
SS..........SS..
................
"""

# The Great Library's grand entrance: one arched double door of old dark
# timber in a stone surround, spanning two tiles. Drawn as a transparent
# overlay and composited over the brick WALL cell at bake time, so the
# FacadeDoor tiles that wear it stay fully opaque in the wall.
GRAND_DOOR_ART = """
..........ssssssssssss..........
.......ssSSSSSSSSSSSSSSss.......
.....sSbbbbbbbbkkbbbbbbbbSs.....
...sSbbbbbbbbbbkkbbbbbbbbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBybkkbyBbbbBbbbSs...
...sSbbbBbbbBybkkbyBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSbbbBbbbBbbkkbbBbbbBbbbSs...
...sSBBBBBBBBBBkkBBBBBBBBBBSs...
..sSSSSSSSSSSSSSSSSSSSSSSSSSSs..
"""

PALETTE = {
    "w": (240, 238, 230, 255),
    "o": (222, 120, 80, 255),
    "y": (240, 200, 60, 255),
    "k": (40, 36, 40, 255),
    # dark old timber, for the Great Library's grand doors
    "B": (88, 64, 44, 255),
    "g": (110, 185, 90, 255),
    "G": (200, 230, 170, 255),
    "m": (214, 208, 232, 255),
    "a": (150, 140, 170, 255),
    "p": (232, 168, 200, 255),
    "L": (214, 188, 148, 255),
    "b": (124, 94, 62, 255),
    "t": (214, 164, 110, 255),
    "1": (188, 92, 84, 255),
    "2": (108, 140, 178, 255),
    "3": (128, 166, 106, 255),
    "e": (222, 198, 152, 255),
    "v": (18, 16, 22, 255),
    "V": (26, 23, 32, 255),
    "s": (152, 152, 164, 255),
    "S": (104, 104, 118, 255),
    "c": (122, 208, 222, 255),
    "i": (200, 224, 244, 255),
    "j": (150, 188, 222, 255),
    # Ferris's own colors (rustacean.net): body orange and its dark accent.
    "f": (247, 76, 0, 255),
    "F": (165, 43, 0, 255),
}

# NPC tunic colors, in quest order 1..12 (canonical here; tints the sprites).
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
# Flavor villagers who live inside the enterable houses: same recipe as the
# quest NPCs, their own colors and hair so nobody looks like anyone's twin.
VILLAGER_COLORS = [(188, 168, 138), (170, 190, 160), (208, 178, 188)]
VILLAGER_LOOKS = [(1, 3, 22, 0), (0, 2, 29, 0), (1, 1, 30, 0)]

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
    # The two later Kenney packs (both CC0, see assets/CREDITS.md): furniture
    # to tell each interior apart, and proper cave/cellar stone.
    indoor = kenney_sheet(sheet_path, "roguelikeIndoor_transparent.png")
    dungeon = kenney_sheet(sheet_path, "roguelikeDungeon_transparent.png")

    def icell(c, r):
        return cell(indoor, c, r)

    def dcell(c, r):
        return cell(dungeon, c, r)

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

    def villager(i):
        bc, br, hc, hr = VILLAGER_LOOKS[i]
        spr = over(cell(chars, bc, br), tint(shirt, VILLAGER_COLORS[i]))
        return over(spr, cell(chars, hc, hr))

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
        ("WALL", cell(sheet, 9, 2)),
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
        # grass decor & ambient life
        ("MUSHROOM", cell(sheet, 48, 4)),
        ("MUSHROOM_TALL", cell(sheet, 48, 3)),
        ("STUMP", from_art(STUMP, PALETTE)),
        ("PEBBLE", cell(sheet, 56, 20)),
        ("FLOWER_SMALL_A", cell(sheet, 28, 9)),
        ("FLOWER_SMALL_B", cell(sheet, 31, 9)),
        ("BUTTERFLY_A", from_art(BUTTERFLY_A, PALETTE)),
        ("BUTTERFLY_B", from_art(BUTTERFLY_B, PALETTE)),
        ("BIRD_A", from_art(BIRD_A, PALETTE)),
        ("BIRD_B", from_art(BIRD_B, PALETTE)),
        # indoors: furniture overlays for the enterable houses
        ("VOID", from_art(VOID, PALETTE)),
        ("RUG", cell(sheet, 27, 25)),
        ("TABLE", cell(sheet, 26, 4)),
        ("STOOL", cell(sheet, 19, 5)),
        ("BED_HEAD", cell(sheet, 14, 1)),
        ("BED_FOOT", cell(sheet, 14, 3)),
        ("HEARTH_A", cell(sheet, 13, 0)),
        ("HEARTH_B", cell(sheet, 14, 0)),
        ("BARREL", cell(sheet, 22, 0)),
        ("CRATE", cell(sheet, 26, 0)),
        ("SHELF", cell(sheet, 31, 0)),
        ("BOOKSHELF", from_art(BOOKSHELF, PALETTE)),
        ("CAT", from_art(CAT, PALETTE)),
        *[(f"VILLAGER_{i + 1}", villager(i)) for i in range(3)],
        # side content: the cellar chest, hidden runestones, moon-mint
        ("CHEST", from_art(CHEST, PALETTE)),
        ("RUNESTONE", from_art(RUNESTONE, PALETTE)),
        ("HERB", from_art(HERB, PALETTE)),
        # ── the variety pass: more of the sheet, so the world isn't monotone ──
        # richer ground bases (opaque)
        ("GRASS_MOTTLED", cell(sheet, 3, 16)),
        ("AUTUMN_GRASS", cell(sheet, 3, 19)),
        ("BLOSSOM_GRASS", cell(sheet, 3, 22)),
        ("FLOWERBED_RED", cell(sheet, 3, 7)),
        ("FLOWERBED_WHITE", cell(sheet, 3, 10)),
        ("FLOWERBED_BLUE", cell(sheet, 3, 13)),
        ("COBBLE", cell(sheet, 6, 2)),
        ("COBBLE_ALT", cell(sheet, 6, 3)),
        # more trees & bushes (overlays)
        ("TREE_TEAL", cell(sheet, 15, 9)),
        ("PINE_ORANGE", cell(sheet, 17, 9)),
        ("PINE_TEAL", cell(sheet, 18, 9)),
        ("DEAD_TREE", cell(sheet, 27, 11)),
        ("BUSH_FLOWER", cell(sheet, 25, 10)),
        ("BUSH_FRUIT", cell(sheet, 26, 10)),
        ("HEDGE", cell(sheet, 19, 9)),
        # the rock family (overlays)
        ("ROCK_BROWN_B", cell(sheet, 55, 19)),
        ("ROCK_BROWN_C", cell(sheet, 56, 19)),
        ("ROCK_GREY_B", cell(sheet, 55, 21)),
        ("ROCK_GREY_C", cell(sheet, 56, 21)),
        ("ROCK_MOSSY_BROWN", cell(sheet, 55, 20)),
        ("ROCK_MOSSY_GREY", cell(sheet, 55, 22)),
        ("CRACK_A", cell(sheet, 26, 19)),
        ("CRACK_B", cell(sheet, 26, 20)),
        # riverbank & village life (overlays)
        ("LILY_PAD", cell(sheet, 26, 11)),
        ("SHRUB_SMALL", cell(sheet, 28, 11)),
        ("STUMP_OLD", cell(sheet, 53, 19)),
        ("STALL_A", cell(sheet, 15, 6)),
        ("STALL_B", cell(sheet, 16, 6)),
        ("AWNING_ORANGE", cell(sheet, 10, 0)),
        ("AWNING_GREEN", cell(sheet, 11, 0)),
        ("IVY", cell(sheet, 44, 23)),
        # ── the animated cast (Ninja Adventure pack, see assets/CREDITS.md) ──
        # Four idle cells per member (facing down, up, left, right); only the
        # first of each quartet gets a named constant.
        *[
            (f"CAST_{name}" if d == 0 else None, frame)
            for name, folder in CAST
            for d, frame in enumerate(na_idles(folder))
        ],
        # The player's stride: two frames per facing, same facing order.
        *[
            ("PLAYER_WALK" if i == 0 else None, frame)
            for i, frame in enumerate(na_walks("Boy"))
        ],
        # ── biome grounds & props (Ninja Adventure tilesets) ─────────────────
        # Each overworld zone gets its own ground family: plain + two
        # tuft-decorated variants, all seamless fills.
        ("MARSH", na_tile("TilesetFloor", 0, 12)),
        ("MARSH_B", na_tile("TilesetFloor", 1, 12)),
        ("MARSH_C", na_tile("TilesetFloor", 3, 12)),
        ("DEEP", na_tile("TilesetFloor", 11, 12)),
        ("DEEP_B", na_tile("TilesetFloor", 12, 12)),
        ("DEEP_C", na_tile("TilesetFloor", 14, 12)),
        ("SNOW", na_tile("TilesetFloor", 0, 19)),
        ("SNOW_B", na_tile("TilesetFloor", 1, 19)),
        ("SNOW_C", na_tile("TilesetFloor", 3, 19)),
        # swaying tall-grass pairs, one per biome
        ("TUFT_DEEP_A", na_tile("TilesetFloorDetail", 0, 2)),
        ("TUFT_DEEP_B", na_tile("TilesetFloorDetail", 1, 2)),
        ("TUFT_MARSH_A", na_tile("TilesetFloorDetail", 6, 2)),
        ("TUFT_MARSH_B", na_tile("TilesetFloorDetail", 7, 2)),
        ("TUFT_SNOW_A", na_tile("TilesetFloorDetail", 0, 3)),
        ("TUFT_SNOW_B", na_tile("TilesetFloorDetail", 1, 3)),
        # ground-litter details
        ("LEAF_LITTER", na_tile("TilesetFloorDetail", 1, 0)),
        ("GOLD_SPECKS", na_tile("TilesetFloorDetail", 3, 0)),
        ("TWIG", na_tile("TilesetFloorDetail", 7, 0)),
        ("BONE", na_tile("TilesetFloorDetail", 14, 0)),
        # snowbound props
        ("SNOWROCK_A", na_tile("TilesetNature", 4, 12)),
        ("SNOWROCK_B", na_tile("TilesetNature", 6, 12)),
        ("ICE_A", na_tile("TilesetNature", 8, 13)),
        ("ICE_B", na_tile("TilesetNature", 9, 13)),
        # forest & marsh props
        ("NA_STONE", na_tile("TilesetNature", 17, 9)),
        ("LOG_NA", na_tile("TilesetNature", 12, 13)),
        ("FERN", na_tile("TilesetNature", 4, 11)),
        ("BOGBERRY", na_tile("TilesetNature", 4, 14)),
        # village garden flowers
        ("SUNFLOWER", na_tile("TilesetNature", 0, 11)),
        ("TULIP", na_tile("TilesetNature", 3, 11)),
        # fence pieces for the edge-aware pass: the vertical run is the
        # horizontal rail turned on its side, so the wood always matches
        ("FENCE_V", cell(sheet, 52, 23).transpose(Image.ROTATE_90)),
        ("FENCE_POST", cell(sheet, 45, 23)),
        # single flowers as transparent overlays, so they bloom on any ground
        ("FLOWER_O_OVER", cell(sheet, 29, 9)),
        ("FLOWER_W_OVER", cell(sheet, 31, 9)),
        ("FLOWER_B_OVER", cell(sheet, 30, 9)),
        # ── the Great Library showcase: windows, art, and exhibits ───────────
        ("WINDOW", from_art(WINDOW, PALETTE)),
        ("PAINTING", from_art(PAINTING, PALETTE)),
        ("PLANT", from_art(PLANT, PALETTE)),
        ("PEDESTAL", from_art(PEDESTAL, PALETTE)),
        # ── the interiors pass: every room gets its own furniture and floor ──
        # (Kenney "Roguelike Indoors" + "Roguelike Caves & Dungeons", CC0.)
        # Per-room floorboards from the main sheet.
        ("FLOOR_LIGHT", cell(sheet, 38, 16)),
        ("FLOOR_BOARDS", cell(sheet, 34, 16)),
        # The bakery kitchen: marble-top counters stocked with wares, a range.
        ("COUNTER_PLAIN", icell(3, 12)),
        ("COUNTER_PLATES", icell(4, 12)),
        ("COUNTER_JARS", icell(5, 12)),
        ("COUNTER_JUGS", icell(6, 12)),
        ("COUNTER_BOTTLES", icell(7, 12)),
        ("SINK", icell(8, 12)),
        ("STOVE_A", icell(14, 14)),
        ("STOVE_B", icell(15, 14)),
        ("TABLE_ROUND", icell(7, 0)),
        # The workshop bench family (wood-top counters, cluttered).
        ("BENCH_WOOD", icell(3, 16)),
        ("BENCH_PLATES", icell(4, 16)),
        ("BENCH_JARS", icell(5, 16)),
        ("BENCH_JUGS", icell(6, 16)),
        # Cottage comforts: a dresser and each cottage's own made bed
        # (striped covers at Granny Sorrel's, plain cream at Tilly's).
        ("DRESSER", icell(2, 12)),
        ("BED_STRIPE_HEAD", cell(sheet, 15, 1)),
        ("BED_STRIPE_FOOT", cell(sheet, 15, 3)),
        ("BED_CREAM_HEAD", cell(sheet, 17, 1)),
        ("BED_CREAM_FOOT", cell(sheet, 17, 3)),
        # Wall and showcase decor: little frames, gallery art, grand fixtures.
        ("FRAME_TEAL", icell(16, 12)),
        ("FRAME_AMBER", icell(17, 12)),
        ("FRAME_SMALL", icell(18, 12)),
        ("PAINTING_MEADOW", icell(19, 12)),
        ("PAINTING_MAP", icell(20, 12)),
        ("MIRROR", icell(22, 14)),
        ("PIANO", icell(23, 8)),
        ("CLOCK", icell(25, 8)),
        ("CANDELABRUM", icell(19, 0)),
        ("POT_PLANT_A", icell(16, 0)),
        ("POT_PLANT_B", icell(17, 0)),
        # ── cave & cellar stone (Caves & Dungeons) ──
        ("CAVE_FLOOR_A", dcell(16, 10)),
        ("CAVE_FLOOR_B", dcell(17, 10)),
        ("CAVE_FLOOR_C", dcell(16, 11)),
        ("EARTH_FLOOR_A", dcell(16, 12)),
        ("EARTH_FLOOR_B", dcell(17, 12)),
        ("EARTH_FLOOR_C", dcell(16, 13)),
        ("SANDSTONE_A", dcell(16, 14)),
        ("SANDSTONE_B", dcell(17, 14)),
        ("STONE_WALL", dcell(9, 2)),
        ("STONE_WALL_CRACK", dcell(9, 3)),
        ("STONE_WALL_VEIN", dcell(11, 3)),
        # Stalagmites, fallen rock, and what grows and glitters down there.
        ("STAL_TALL_A", dcell(5, 0)),
        ("STAL_TALL_B", dcell(6, 0)),
        ("STAL_SMALL", dcell(5, 1)),
        ("CAVE_ROCKS", dcell(0, 1)),
        ("CAVE_ROCKS_MOSS", dcell(1, 1)),
        ("CRYSTAL_VIOLET", dcell(14, 12)),
        ("CRYSTAL_AMBER", dcell(15, 12)),
        ("CRYSTAL_EMBER", dcell(14, 13)),
        ("CRYSTAL_MOSS", dcell(15, 13)),
        ("SHROOMS_PALE", dcell(0, 3)),
        ("SHROOMS_RED", dcell(1, 3)),
        ("SHROOMS_TALL", dcell(3, 3)),
        ("SKULL", dcell(0, 2)),
        ("OLD_BONES", dcell(2, 2)),
        ("URN", dcell(14, 15)),
        ("COBWEB_A", dcell(7, 0)),
        ("COBWEB_B", dcell(7, 1)),
        # The carpenter's anvil, standing among the workshop crates.
        ("ANVIL", cell(sheet, 15, 0)),
        # ── new quest-giver cast, appended (see NEW_CAST above; keeps every
        # id baked before this line unchanged) ──
        *[
            (f"CAST_{name}" if d == 0 else None, frame)
            for name, folder in NEW_CAST
            for d, frame in enumerate(na_idles(folder))
        ],
        # ── village building expansion: more roof/wall/door/window variety,
        # cropped from parts of the Kenney sheet the original bake never
        # touched. Appended so nothing above shifts.
        ("WALL_STONE", cell(sheet, 6, 2)),
        ("WALL_PLASTER", cell(sheet, 7, 2)),
        ("ROOF_SLATE", cell(sheet, 32, 22)),
        ("ROOF_CREAM", cell(sheet, 38, 22)),
        ("DOOR_ARCH", cell(sheet, 37, 0)),
        ("DOOR_DOUBLE_L", cell(sheet, 34, 0)),
        ("DOOR_DOUBLE_R", cell(sheet, 35, 0)),
        ("WINDOW_ROUND", cell(sheet, 44, 0)),
        ("WINDOW_SQUARE", cell(sheet, 48, 0)),
        # ── whole-building prefabs (Zelda-like sheet by ArMM1998, CC0) ──
        # Drawn in 3/4 perspective — sloped roofs, real front walls — and
        # placed in the world as Tile::Facade runs (see MapBuilder::prefab).
        # The cottage, its shut-door twin (a pasted arch over the open
        # doorway), the barn with and without a pasted door, a small shed,
        # the market stall, and the plaza fountain.
        *zl_prefab("HOUSE_A", 96, 0, 5, 5),
        *zl_prefab("HOUSE_A_SHUT", 96, 0, 5, 5, paste=(256, 64, 16, 32, 32, 44)),
        *zl_prefab("HOUSE_B", 173, 0, 5, 5),
        *zl_prefab("HOUSE_B_DOOR", 173, 0, 5, 5, paste=(256, 64, 16, 32, 32, 44)),
        *zl_prefab("SHED", 200, 86, 3, 3, keep=(8, 0, 40, 46)),
        *zl_prefab("STALL", 287, 360, 5, 5, keep=(0, 0, 76, 80)),
        # The three fountains in the sheet touch each other with no gap;
        # x=399 is the waist between the first and second, so the crop stops
        # a pixel short of it.
        *zl_prefab("FOUNTAIN", 351, 136, 3, 4, keep=(0, 2, 48, 55)),
        # ── the village expansion: premade homes from the Ninja Adventure
        # pack's TilesetHouse (see assets/CREDITS.md). Flavor homes, so every
        # open doorway gets the sheet's own plank door pasted shut — same
        # trick as HOUSE_A_SHUT above. ──
        *na_prefab("TilesetHouse", "NA_HOUSE_THATCH", 0, 0, 4, 3, paste=(32, 48, 16, 16, 24, 32)),
        *na_prefab("TilesetHouse", "NA_HOUSE_FLAT", 64, 0, 4, 3, paste=(32, 48, 16, 16, 24, 32)),
        *na_prefab("TilesetHouse", "NA_HOUSE_PLAIN", 128, 0, 4, 3, paste=(32, 48, 16, 16, 24, 32)),
        *na_prefab("TilesetHouse", "NA_SHOP", 256, 0, 3, 3, paste=(32, 48, 16, 16, 16, 32)),
        *na_prefab("TilesetHouse", "NA_TAVERN", 304, 0, 3, 3, paste=(32, 48, 16, 16, 16, 32)),
        *na_prefab("TilesetHouse", "NA_HOUSE_TALL", 464, 0, 4, 4, paste=(32, 48, 16, 16, 16, 46)),
        # ── the big trees (Ninja Adventure TilesetNature): stately old
        # growth planted as prefabs between the everyday single-tile trees ──
        *na_prefab("TilesetNature", "TREE_BIG_PINK", 0, 288, 3, 3),
        *na_prefab("TilesetNature", "TREE_BIG_GREEN", 48, 288, 3, 3),
        *na_prefab("TilesetNature", "TREE_BIG_WHITE", 96, 288, 3, 3),
        *na_prefab("TilesetNature", "TREE_BIG_ORANGE", 144, 288, 3, 3),
        *na_prefab("TilesetNature", "TREE_TALL_PINE", 0, 32, 4, 3),
        *na_prefab("TilesetNature", "TREE_TALL_CANOPY", 64, 32, 4, 3),
        *na_prefab("TilesetNature", "TREE_TALL_SNOW", 128, 32, 4, 3),
        *na_prefab("TilesetNature", "BUSH_BIG", 0, 0, 2, 2),
        # single-tile garden flowers to thicken the village beds
        ("SUNFLOWER_TALL", na_tile("TilesetNature", 1, 11)),
        ("ROSEBUSH", na_tile("TilesetNature", 2, 11)),
        # ── the zone-identity pass (all Ninja Adventure, CC0; see
        # assets/CREDITS.md): dead old growth for a darker Whispering Woods,
        # moss-eaten ruins to stumble on between the trees, and the makings
        # of Silverford's working waterfront. ──
        *na_prefab("TilesetNature", "TREE_DEAD_BIG", 64, 0, 2, 2),
        *na_prefab("TilesetNature", "TREE_GNARLED", 16, 80, 2, 3),
        *na_prefab("TilesetVillageAbandoned", "RUIN_STONE", 0, 0, 4, 3),
        *na_prefab("TilesetVillageAbandoned", "RUIN_LODGE", 192, 128, 4, 3),
        # Harbor timber from TilesetWater: pier planks (the END cell carries
        # the piling tops along its water edge) and a little moored skiff.
        ("PIER_PLANK", na_tile("TilesetWater", 5, 13)),
        ("PIER_END", na_tile("TilesetWater", 5, 15)),
        *na_prefab("TilesetWater", "SKIFF", 416, 0, 2, 1),
        # The ferry itself, from the raw pack (Backgrounds/Vehicles).
        *img_prefab(
            NA_DIR / "pack" / "Backgrounds" / "Vehicles" / "Boat.png",
            "BOAT", 0, 0, 5, 2,
        ),
        # A four-frame water ripple, lapping around hulls and pilings.
        *img_prefab(
            NA_DIR / "pack" / "Backgrounds" / "Animated" / "WaterRipples"
            / "SpriteSheet16x16.png",
            "RIPPLE", 0, 0, 4, 1,
        ),
        # A cast fishing line, drawn on the water south of whoever holds it.
        ("ROD_CAST", from_art(ROD_CAST, PALETTE)),
        # The deep-woods floor: the forest ground family dimmed toward moss
        # and shadow, plus matching dimmed encounter-grass tufts, so the
        # Whispering Woods keep their dusk even at noon.
        ("WOODS_FLOOR", dim(na_tile("TilesetFloor", 11, 12), (0.62, 0.74, 0.68))),
        ("WOODS_FLOOR_B", dim(na_tile("TilesetFloor", 12, 12), (0.62, 0.74, 0.68))),
        ("WOODS_FLOOR_C", dim(na_tile("TilesetFloor", 14, 12), (0.62, 0.74, 0.68))),
        ("WOODS_TUFT_A", dim(na_tile("TilesetFloorDetail", 0, 2), (0.62, 0.74, 0.68))),
        ("WOODS_TUFT_B", dim(na_tile("TilesetFloorDetail", 1, 2), (0.62, 0.74, 0.68))),
        # ── the companion at your heels: a very small crab in Ferris's exact
        # colors, hand-pixeled like the other critters. Idle, a claws-up
        # wave/startle, two scuttle frames, curled asleep, and just the
        # eyestalks peeking over tall grass. ──
        ("CRAB_IDLE", from_art(CRAB_IDLE, PALETTE)),
        ("CRAB_WAVE", from_art(CRAB_WAVE, PALETTE)),
        ("CRAB_WALK_A", from_art(CRAB_WALK_A, PALETTE)),
        ("CRAB_WALK_B", from_art(CRAB_WALK_B, PALETTE)),
        ("CRAB_CURL", from_art(CRAB_CURL, PALETTE)),
        ("CRAB_PEEK", from_art(CRAB_PEEK, PALETTE)),
        # ── walk cycles for the rest of the char-select roster (playtest
        # note: only the Boy strode). Same two-frames-per-facing layout as
        # PLAYER_WALK. The Child ships no Walk strip, so its stride pairs
        # the compact SpriteSheet's step row with its idle row (na_steps). ──
        *[
            ("WALK_CHILD" if i == 0 else None, frame)
            for i, frame in enumerate(na_steps("Child"))
        ],
        *[
            ("WALK_MANGREEN" if i == 0 else None, frame)
            for i, frame in enumerate(na_walks("ManGreen"))
        ],
        *[
            ("WALK_WOMAN" if i == 0 else None, frame)
            for i, frame in enumerate(na_walks("Woman"))
        ],
        # ── fence corners (playtest note: the pen's corners didn't join).
        # Named by where the corner sits on its enclosure: NW joins runs
        # heading east and south, and so around. ──
        ("FENCE_NW", fence_corner(sheet, east=True, south=True)),
        ("FENCE_NE", fence_corner(sheet, east=False, south=True)),
        ("FENCE_SW", fence_corner(sheet, east=True, south=False)),
        ("FENCE_SE", fence_corner(sheet, east=False, south=False)),
        # ── the village draw-well, hand-pixeled (see WELL_ART) ──
        *from_art_prefab("WELL", WELL_ART, PALETTE, 2, 2),
        # a dark cave mouth for the Echo Cave entrance
        ("CAVE_MOUTH", from_art(CAVE_MOUTH_ART, PALETTE)),
        # ── the Great Library's grand entrance (playtest note: the plain
        # door sprites read as lockers floating in the brick). One arched
        # double door across two FacadeDoor cells, drawn over the WALL
        # brick so the cells stay opaque wherever they're set. ──
        *[
            (["DOOR_GRAND_L", "DOOR_GRAND_R"][i], over(cell(sheet, 9, 2), img))
            for i, (_, img) in enumerate(
                from_art_prefab(None, GRAND_DOOR_ART, PALETTE, 2, 1)
            )
        ],
        # ── Fernshade, the hamlet under the Whispering Woods' canopy
        # (playtest note: the woods' folk deserved somewhere to live).
        # Dark-forest builds: the village cottage and the flavor homes are
        # dimmed toward moss and dusk the same way the WOODS_FLOOR family
        # is; the lamplit common house is the tavern build with its own
        # doorway left open (it already wears a lamp over the lintel). ──
        *[
            (n, dim(img, (0.66, 0.74, 0.72)))
            for n, img in zl_prefab("WOODS_COTTAGE", 96, 0, 5, 5)
        ],
        *[
            (n, dim(img, (0.72, 0.80, 0.76)))
            for n, img in na_prefab("TilesetHouse", "WOODS_COMMON", 304, 0, 3, 3)
        ],
        *[
            (n, dim(img, (0.50, 0.64, 0.60)))
            for n, img in na_prefab(
                "TilesetHouse", "WOODS_HOME_A", 0, 0, 4, 3,
                paste=(32, 48, 16, 16, 24, 32),
            )
        ],
        *[
            (n, dim(img, (0.55, 0.66, 0.62)))
            for n, img in na_prefab(
                "TilesetHouse", "WOODS_HOME_B", 128, 0, 4, 3,
                paste=(32, 48, 16, 16, 24, 32),
            )
        ],
        # Fernshade's residents, their own tail block like NEW_CAST above.
        *[
            (f"CAST_{name}" if d == 0 else None, frame)
            for name, folder in FERN_CAST
            for d, frame in enumerate(na_idles(folder))
        ],
        # ── the wild runes' visible forms (the shelf's "visible wild-rune
        # forms" note): two bobbing frames per rune in id order, so the
        # encounter screen can show who's rustling in the grass. ──
        *[
            ("WILD_FORM" if i == 0 else None, frame)
            for i, frame in enumerate(
                frame for folder in WILD_FORMS for frame in na_monster(folder)
            )
        ],
        # ── spell FX (Ninja Adventure FX strips, CC0): the casting screen's
        # blooming spark circle and the fizzle's gentle smoke puff. 32x32
        # frames baked as 2x2 cell blocks, six frames each, consecutive —
        # frame f of effect E lives at E + f*4. ──
        *[
            cell
            for i in range(6)
            for cell in img_prefab(
                NA_DIR / "pack" / "FX" / "Magic" / "Circle" / "SpriteSheetSpark.png",
                "FX_CAST" if i == 0 else None,
                i * 32, 0, 2, 2,
            )
        ],
        *[
            cell
            for i in range(6)
            for cell in img_prefab(
                NA_DIR / "pack" / "FX" / "Smoke" / "Smoke" / "SpriteSheet.png",
                "FX_PUFF" if i == 0 else None,
                i * 32, 0, 2, 2,
            )
        ],
        # ── the small-lives expansion: four more hand-pixeled critters ──
        ("DOG", from_art(DOG, PALETTE)),
        ("BOAR", from_art(BOAR, PALETTE)),
        ("DUCK", from_art(DUCK, PALETTE)),
        ("DONKEY", from_art(DONKEY, PALETTE)),
    ]

    rows = (len(cells) + COLS - 1) // COLS
    atlas = Image.new("RGBA", (COLS * TILE, rows * TILE), (0, 0, 0, 0))
    for i, (_, img) in enumerate(cells):
        atlas.paste(img, ((i % COLS) * TILE, (i // COLS) * TILE))
    atlas.save("assets/atlas.png")

    print(f"atlas: {atlas.width}x{atlas.height}, {len(cells)} cells")
    for i, (name, _) in enumerate(cells):
        if name is not None:
            print(f"pub const {name}: u16 = {i};")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
