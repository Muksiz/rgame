//! The baked sprite atlas (see `tools/bake_atlas.py` and `assets/CREDITS.md`).
//!
//! Cells are 16×16, laid out 16 per row, no margins. The constants below must
//! match the bake script's output order.

pub const TILE: usize = 16;
const ATLAS_COLS: usize = 16;

pub const GRASS: u16 = 0;
pub const GRASS_ALT: u16 = 1;
pub const PATH: u16 = 2;
pub const PATH_ALT: u16 = 3;
pub const SAND: u16 = 4;
pub const WATER_A: u16 = 5;
pub const WATER_B: u16 = 6;
pub const FLOOR: u16 = 7;
pub const STONE: u16 = 8;
pub const FLOWER_ORANGE: u16 = 9;
pub const FLOWER_WHITE: u16 = 10;
pub const FLOWER_BLUE: u16 = 11;
pub const TREE_GREEN: u16 = 12;
pub const TREE_ORANGE: u16 = 13;
pub const PINE: u16 = 14;
pub const BUSH: u16 = 15;
pub const BERRY_BUSH: u16 = 16;
pub const SPROUT: u16 = 17;
pub const SPROUT_ALT: u16 = 18;
pub const LILY: u16 = 19;
pub const ROCK_GREY: u16 = 20;
pub const ROCK_BROWN: u16 = 21;
pub const FENCE: u16 = 22;
pub const GATE: u16 = 23;
pub const SIGN: u16 = 24;
pub const DOOR: u16 = 25;
pub const BRIDGE: u16 = 26;
pub const WALL: u16 = 27;
pub const ROOF: u16 = 28;
pub const CAMPFIRE_A: u16 = 29;
pub const CAMPFIRE_B: u16 = 30;
pub const TORCH_UNLIT: u16 = 31;
pub const TORCH_LIT_A: u16 = 32;
pub const TORCH_LIT_B: u16 = 33;
pub const PLAYER: u16 = 34;
pub const NPC_1: u16 = 35;
pub const CHICKEN: u16 = 47;
pub const SHEEP: u16 = 48;
pub const FROG: u16 = 49;
pub const MOTH: u16 = 50;
pub const FERRIS: u16 = 51;
pub const MUSHROOM: u16 = 52;
pub const MUSHROOM_TALL: u16 = 53;
pub const STUMP: u16 = 54;
pub const PEBBLE: u16 = 55;
pub const FLOWER_SMALL_A: u16 = 56;
pub const FLOWER_SMALL_B: u16 = 57;
pub const BUTTERFLY_A: u16 = 58;
pub const BUTTERFLY_B: u16 = 59;
pub const BIRD_A: u16 = 60;
pub const BIRD_B: u16 = 61;
pub const VOID: u16 = 62;
pub const RUG: u16 = 63;
pub const TABLE: u16 = 64;
pub const STOOL: u16 = 65;
pub const BED_HEAD: u16 = 66;
pub const BED_FOOT: u16 = 67;
pub const HEARTH_A: u16 = 68;
pub const HEARTH_B: u16 = 69;
pub const BARREL: u16 = 70;
pub const CRATE: u16 = 71;
pub const SHELF: u16 = 72;
pub const BOOKSHELF: u16 = 73;
pub const CAT: u16 = 74;
pub const VILLAGER_1: u16 = 75;
pub const CHEST: u16 = 78;
pub const RUNESTONE: u16 = 79;
pub const HERB: u16 = 80;
pub const GRASS_MOTTLED: u16 = 81;
pub const AUTUMN_GRASS: u16 = 82;
pub const BLOSSOM_GRASS: u16 = 83;
pub const FLOWERBED_RED: u16 = 84;
pub const FLOWERBED_WHITE: u16 = 85;
pub const FLOWERBED_BLUE: u16 = 86;
pub const COBBLE: u16 = 87;
pub const COBBLE_ALT: u16 = 88;
pub const TREE_TEAL: u16 = 89;
pub const PINE_ORANGE: u16 = 90;
pub const PINE_TEAL: u16 = 91;
pub const DEAD_TREE: u16 = 92;
pub const BUSH_FLOWER: u16 = 93;
pub const BUSH_FRUIT: u16 = 94;
pub const HEDGE: u16 = 95;
pub const ROCK_BROWN_B: u16 = 96;
pub const ROCK_BROWN_C: u16 = 97;
pub const ROCK_GREY_B: u16 = 98;
pub const ROCK_GREY_C: u16 = 99;
pub const ROCK_MOSSY_BROWN: u16 = 100;
pub const ROCK_MOSSY_GREY: u16 = 101;
pub const CRACK_A: u16 = 102;
pub const CRACK_B: u16 = 103;
pub const LILY_PAD: u16 = 104;
pub const SHRUB_SMALL: u16 = 105;
pub const STUMP_OLD: u16 = 106;
pub const STALL_A: u16 = 107;
pub const STALL_B: u16 = 108;
pub const AWNING_ORANGE: u16 = 109;
pub const AWNING_GREEN: u16 = 110;
pub const IVY: u16 = 111;
// The animated cast (Ninja Adventure pack): four idle cells per member,
// facing down, up, left, right. Atlas order is historical, but who *wears*
// each sprite is decided in code: the char-select roster (see PLAYABLE) wears
// members 0, 4, 6, 16 (Boy, Child, ManGreen, Woman) and no NPC touches those,
// so a chosen traveller never has a twin in the world. Every other member is
// an NPC — members 18..=20 (Villager3, Villager4, Princess) stand in for Wren,
// Shepherd Ambrose and Hen-keeper Tilly, whose sprites the roster took over
// (see CAST in tools/bake_atlas.py, and npc_sprite in gfx/scene.rs).
pub const CAST: u16 = 112;
pub const CAST_FACINGS: u16 = 4;
pub const CAST_BOY: u16 = 112;
pub const CAST_MASTER: u16 = 116;
pub const CAST_OLDMAN2: u16 = 120;
pub const CAST_VILLAGER_M: u16 = 124;
pub const CAST_CHILD: u16 = 128;
pub const CAST_VILLAGE6: u16 = 132;
pub const CAST_MANGREEN: u16 = 136;
pub const CAST_VILLAGER2: u16 = 140;
pub const CAST_EGGBOY: u16 = 144;
pub const CAST_MONK: u16 = 148;
pub const CAST_INSPECTOR: u16 = 152;
pub const CAST_STATUE: u16 = 156;
pub const CAST_OLDMAN3: u16 = 160;
pub const CAST_OLDWOMAN: u16 = 164;
pub const CAST_GREENMAN: u16 = 168;
pub const CAST_HUNTER: u16 = 172;
pub const CAST_WOMAN: u16 = 176;
pub const CAST_NOBLE: u16 = 180;
pub const CAST_VILLAGER3: u16 = 184;
pub const CAST_VILLAGER4: u16 = 188;
pub const CAST_PRINCESS: u16 = 192;
/// The player's stride: two frames per facing, same facing order as CAST.
/// Only the default look (the Boy) has a baked walk-cycle; the others walk on
/// their idle facings, turning to look where they go.
pub const PLAYER_WALK: u16 = 196;

/// The characters a new traveller may choose to play as: a display look and
/// the cast member whose four idle facings they wear. The name is the player's
/// to invent — no suggestions here. The first (the Boy) is the one with a full
/// stride animation.
pub struct Playable {
    pub look: &'static str,
    pub cast: u16,
}

pub const PLAYABLE: [Playable; 4] = [
    Playable {
        look: "the young traveller",
        cast: CAST_BOY,
    },
    Playable {
        look: "the curious sprout",
        cast: CAST_CHILD,
    },
    Playable {
        look: "the greenwood ranger",
        cast: CAST_MANGREEN,
    },
    Playable {
        look: "the roaming herbalist",
        cast: CAST_WOMAN,
    },
];
// Biome grounds (plain + two decorated variants each) and per-biome props,
// from the Ninja Adventure tilesets in assets/ninja_adventure/tilesets/.
pub const MARSH: u16 = 204;
pub const MARSH_B: u16 = 205;
pub const MARSH_C: u16 = 206;
pub const DEEP: u16 = 207;
pub const DEEP_B: u16 = 208;
pub const DEEP_C: u16 = 209;
pub const SNOW: u16 = 210;
pub const SNOW_B: u16 = 211;
pub const SNOW_C: u16 = 212;
pub const TUFT_DEEP_A: u16 = 213;
pub const TUFT_DEEP_B: u16 = 214;
pub const TUFT_MARSH_A: u16 = 215;
pub const TUFT_MARSH_B: u16 = 216;
pub const TUFT_SNOW_A: u16 = 217;
pub const TUFT_SNOW_B: u16 = 218;
pub const LEAF_LITTER: u16 = 219;
pub const GOLD_SPECKS: u16 = 220;
pub const TWIG: u16 = 221;
pub const BONE: u16 = 222;
pub const SNOWROCK_A: u16 = 223;
pub const SNOWROCK_B: u16 = 224;
pub const ICE_A: u16 = 225;
pub const ICE_B: u16 = 226;
pub const NA_STONE: u16 = 227;
pub const LOG_NA: u16 = 228;
pub const FERN: u16 = 229;
pub const BOGBERRY: u16 = 230;
pub const SUNFLOWER: u16 = 231;
pub const TULIP: u16 = 232;
pub const FENCE_V: u16 = 233;
pub const FENCE_POST: u16 = 234;
// Single flowers as transparent overlays, so they bloom on any ground.
pub const FLOWER_O_OVER: u16 = 235;
pub const FLOWER_W_OVER: u16 = 236;
pub const FLOWER_B_OVER: u16 = 237;
// The Great Library showcase: sunlit windows, framed art, and gallery exhibits.
pub const WINDOW: u16 = 238;
pub const PAINTING: u16 = 239;
pub const PLANT: u16 = 240;
pub const PEDESTAL: u16 = 241;
// The interiors pass (Kenney "Roguelike Indoors" + "Roguelike Caves &
// Dungeons", both CC0): per-room floors, furniture that tells each house
// apart, and proper cave/cellar stone.
pub const FLOOR_LIGHT: u16 = 242;
pub const FLOOR_BOARDS: u16 = 243;
pub const COUNTER_PLAIN: u16 = 244;
pub const COUNTER_PLATES: u16 = 245;
pub const COUNTER_JARS: u16 = 246;
pub const COUNTER_JUGS: u16 = 247;
pub const COUNTER_BOTTLES: u16 = 248;
pub const SINK: u16 = 249;
pub const STOVE_A: u16 = 250;
pub const STOVE_B: u16 = 251;
pub const TABLE_ROUND: u16 = 252;
pub const BENCH_WOOD: u16 = 253;
pub const BENCH_PLATES: u16 = 254;
pub const BENCH_JARS: u16 = 255;
pub const BENCH_JUGS: u16 = 256;
pub const DRESSER: u16 = 257;
pub const BED_STRIPE_HEAD: u16 = 258;
pub const BED_STRIPE_FOOT: u16 = 259;
pub const BED_CREAM_HEAD: u16 = 260;
pub const BED_CREAM_FOOT: u16 = 261;
pub const FRAME_TEAL: u16 = 262;
pub const FRAME_AMBER: u16 = 263;
pub const FRAME_SMALL: u16 = 264;
pub const PAINTING_MEADOW: u16 = 265;
pub const PAINTING_MAP: u16 = 266;
pub const MIRROR: u16 = 267;
pub const PIANO: u16 = 268;
pub const CLOCK: u16 = 269;
pub const CANDELABRUM: u16 = 270;
pub const POT_PLANT_A: u16 = 271;
pub const POT_PLANT_B: u16 = 272;
pub const CAVE_FLOOR_A: u16 = 273;
pub const CAVE_FLOOR_B: u16 = 274;
pub const CAVE_FLOOR_C: u16 = 275;
pub const EARTH_FLOOR_A: u16 = 276;
pub const EARTH_FLOOR_B: u16 = 277;
pub const EARTH_FLOOR_C: u16 = 278;
pub const SANDSTONE_A: u16 = 279;
pub const SANDSTONE_B: u16 = 280;
pub const STONE_WALL: u16 = 281;
pub const STONE_WALL_CRACK: u16 = 282;
pub const STONE_WALL_VEIN: u16 = 283;
pub const STAL_TALL_A: u16 = 284;
pub const STAL_TALL_B: u16 = 285;
pub const STAL_SMALL: u16 = 286;
pub const CAVE_ROCKS: u16 = 287;
pub const CAVE_ROCKS_MOSS: u16 = 288;
pub const CRYSTAL_VIOLET: u16 = 289;
pub const CRYSTAL_AMBER: u16 = 290;
pub const CRYSTAL_EMBER: u16 = 291;
pub const CRYSTAL_MOSS: u16 = 292;
pub const SHROOMS_PALE: u16 = 293;
pub const SHROOMS_RED: u16 = 294;
pub const SHROOMS_TALL: u16 = 295;
pub const SKULL: u16 = 296;
pub const OLD_BONES: u16 = 297;
pub const URN: u16 = 298;
pub const COBWEB_A: u16 = 299;
pub const COBWEB_B: u16 = 300;
pub const ANVIL: u16 = 301;
// The beginner-quest expansion's cast — baked as a tail block (see NEW_CAST
// in tools/bake_atlas.py) so none of the ids above ever moved.
pub const CAST_TANSY: u16 = 302;
pub const CAST_FITCH: u16 = 306;
pub const CAST_HOBB: u16 = 310;
pub const CAST_REED: u16 = 314;
pub const CAST_PIP: u16 = 318;
pub const CAST_BRIAR: u16 = 322;
pub const CAST_YEW: u16 = 326;
pub const CAST_SABLE: u16 = 330;
pub const CAST_FENN: u16 = 334;
pub const CAST_SIL: u16 = 338;
pub const CAST_FAYE: u16 = 342;
// Village building expansion: more roof/wall/door/window variety, cropped
// from parts of the Kenney sheet the original bake never touched.
pub const WALL_STONE: u16 = 346;
pub const WALL_PLASTER: u16 = 347;
pub const ROOF_SLATE: u16 = 348;
pub const ROOF_CREAM: u16 = 349;
pub const DOOR_ARCH: u16 = 350;
pub const DOOR_DOUBLE_L: u16 = 351;
pub const DOOR_DOUBLE_R: u16 = 352;
pub const WINDOW_ROUND: u16 = 353;
pub const WINDOW_SQUARE: u16 = 354;
// Whole-building prefabs (Zelda-like sheet by ArMM1998, CC0): 3/4-perspective
// buildings sliced into consecutive cells row-major; each constant is the
// top-left cell, placed in the world via `MapBuilder::prefab` with the
// matching width/height below.
pub const HOUSE_A: u16 = 355; // cottage, open doorway (5x5)
pub const HOUSE_A_SHUT: u16 = 380; // cottage with its door shut (5x5)
pub const HOUSE_B: u16 = 405; // barn, no entrance (5x5)
pub const HOUSE_B_DOOR: u16 = 430; // barn with an arched door (5x5)
pub const SHED: u16 = 455; // little steep-roofed shed (3x3)
pub const STALL: u16 = 464; // blue-striped market stall (5x5)
pub const FOUNTAIN: u16 = 489; // round plaza fountain (3x4)
// Premade village homes from the Ninja Adventure TilesetHouse (CC0, see
// assets/CREDITS.md), doors pasted shut at bake time — flavor homes that
// nobody needs to enter, for a village that finally looks lived-in.
pub const NA_HOUSE_THATCH: u16 = 501; // thatched roof, shoji windows (4x3)
pub const NA_HOUSE_FLAT: u16 = 513; // flat tan roof (4x3)
pub const NA_HOUSE_PLAIN: u16 = 525; // thatched roof, plain walls (4x3)
pub const NA_SHOP: u16 = 537; // little bakery-signed shopfront (3x3)
pub const NA_TAVERN: u16 = 546; // dark-beamed tavern, lanterns lit (3x3)
pub const NA_HOUSE_TALL: u16 = 555; // two-storey townhouse (4x4)
// Big premade trees from the Ninja Adventure TilesetNature — old growth
// planted between the everyday single-tile trees.
pub const TREE_BIG_PINK: u16 = 571; // blossoming crown (3x3)
pub const TREE_BIG_GREEN: u16 = 580; // broad summer crown (3x3)
pub const TREE_BIG_WHITE: u16 = 589; // snow-dusted crown (3x3)
pub const TREE_BIG_ORANGE: u16 = 598; // autumn crown (3x3)
pub const TREE_TALL_PINE: u16 = 607; // towering old pine (4x3)
pub const TREE_TALL_CANOPY: u16 = 619; // triple-crowned canopy (4x3)
pub const TREE_TALL_SNOW: u16 = 631; // snowbound canopy (4x3)
pub const BUSH_BIG: u16 = 643; // spreading garden bush (2x2)
pub const SUNFLOWER_TALL: u16 = 647;
pub const ROSEBUSH: u16 = 648;
// The zone-identity pass (all Ninja Adventure, CC0): dead old growth and
// moss-eaten ruins for a darker Whispering Woods, harbor timber and boats
// for Silverford's waterfront.
pub const TREE_DEAD_BIG: u16 = 649; // gnarled bare crown (2x2)
pub const TREE_GNARLED: u16 = 653; // tall dead bramble tree (2x3)
pub const RUIN_STONE: u16 = 659; // moss-eaten stone cottage, open doorway (4x3)
pub const RUIN_LODGE: u16 = 671; // overgrown timber lodge, open doorway (4x3)
pub const PIER_PLANK: u16 = 683; // boardwalk planking (seamless)
pub const PIER_END: u16 = 684; // planking with piling tops along its edge
pub const SKIFF: u16 = 685; // little moored rowboat (2x1)
pub const BOAT: u16 = 687; // the ferry, tied up along the long pier (5x2)
pub const RIPPLE: u16 = 697; // four-frame water ripple (RIPPLE..RIPPLE+3)
pub const ROD_CAST: u16 = 701; // a cast fishing line, bobber and all
// The deep-woods floor and tufts: the forest ground family dimmed toward
// moss and shadow, so the Whispering Woods keep their dusk even at noon.
pub const WOODS_FLOOR: u16 = 702;
pub const WOODS_FLOOR_B: u16 = 703;
pub const WOODS_FLOOR_C: u16 = 704;
pub const WOODS_TUFT_A: u16 = 705;
pub const WOODS_TUFT_B: u16 = 706;
// Ferris, the companion at your heels: a very small crab in his namesake's
// exact colors (rustacean.net's #F74C00 / #A52B00), hand-pixeled like the
// other critters. Idle, a claws-up wave/startle, two scuttle frames, curled
// asleep, and just the eyestalks peeking over tall grass.
pub const CRAB_IDLE: u16 = 707;
pub const CRAB_WAVE: u16 = 708;
pub const CRAB_WALK_A: u16 = 709;
pub const CRAB_WALK_B: u16 = 710;
pub const CRAB_CURL: u16 = 711;
pub const CRAB_PEEK: u16 = 712;
pub const HOUSE_SIZE: (i32, i32) = (5, 5);
pub const SHED_SIZE: (i32, i32) = (3, 3);
pub const STALL_SIZE: (i32, i32) = (5, 5);
pub const FOUNTAIN_SIZE: (i32, i32) = (3, 4);
pub const NA_HOUSE_SIZE: (i32, i32) = (4, 3);
pub const NA_SHOP_SIZE: (i32, i32) = (3, 3);
pub const NA_TALL_SIZE: (i32, i32) = (4, 4);
pub const TREE_BIG_SIZE: (i32, i32) = (3, 3);
pub const TREE_TALL_SIZE: (i32, i32) = (4, 3);
pub const BUSH_BIG_SIZE: (i32, i32) = (2, 2);
pub const TREE_DEAD_BIG_SIZE: (i32, i32) = (2, 2);
pub const TREE_GNARLED_SIZE: (i32, i32) = (2, 3);
pub const RUIN_SIZE: (i32, i32) = (4, 3);
/// Where the walkable doorway cell sits inside RUIN_STONE / RUIN_LODGE.
pub const RUIN_DOOR_AT: (i32, i32) = (1, 2);
pub const SKIFF_SIZE: (i32, i32) = (2, 1);
pub const BOAT_SIZE: (i32, i32) = (5, 2);
/// Where the walkable doorway cell sits inside HOUSE_A / HOUSE_B_DOOR.
pub const HOUSE_DOOR_AT: (i32, i32) = (2, 4);

static ATLAS_PNG: &[u8] = include_bytes!("../../assets/atlas.png");

pub struct Atlas {
    px: Vec<u8>, // RGBA8
    width: usize,
}

impl Atlas {
    pub fn load() -> Self {
        let decoder = png::Decoder::new(ATLAS_PNG);
        let mut reader = decoder
            .read_info()
            .expect("atlas.png is baked into the binary");
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("atlas.png decodes");
        assert_eq!(info.color_type, png::ColorType::Rgba, "atlas must be RGBA");
        buf.truncate(info.buffer_size());
        Self {
            px: buf,
            width: info.width as usize,
        }
    }

    /// RGBA of pixel (x, y) inside sprite `id`.
    #[inline]
    pub fn pixel(&self, id: u16, x: usize, y: usize) -> (u8, u8, u8, u8) {
        let sx = (id as usize % ATLAS_COLS) * TILE + x;
        let sy = (id as usize / ATLAS_COLS) * TILE + y;
        let i = (sy * self.width + sx) * 4;
        (self.px[i], self.px[i + 1], self.px[i + 2], self.px[i + 3])
    }
}
