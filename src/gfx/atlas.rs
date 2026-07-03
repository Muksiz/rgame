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
pub const CAST_CAVEGIRL: u16 = 132;
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

/// The characters a new traveller may choose to play as: a display look, a
/// suggested name, and the cast member whose four idle facings they wear.
/// The first (the Boy) is the one with a full stride animation.
pub struct Playable {
    pub look: &'static str,
    pub default_name: &'static str,
    pub cast: u16,
}

pub const PLAYABLE: [Playable; 4] = [
    Playable {
        look: "the young traveller",
        default_name: "Rue",
        cast: CAST_BOY,
    },
    Playable {
        look: "the curious sprout",
        default_name: "Fern",
        cast: CAST_CHILD,
    },
    Playable {
        look: "the greenwood ranger",
        default_name: "Linden",
        cast: CAST_MANGREEN,
    },
    Playable {
        look: "the roaming herbalist",
        default_name: "Hazel",
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
