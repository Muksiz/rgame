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
