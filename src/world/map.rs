use crate::world::entity::{Critter, Npc, Sign};

pub const MAP_W: i32 = 240;
pub const MAP_H: i32 = 70;

/// Deterministic 2D hash used for scattering decoration, animation phases,
/// out-of-bounds scenery and anything else that must look random but stay put.
pub fn hash2(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32)
        .wrapping_mul(374_761_393)
        .wrapping_add((y as u32).wrapping_mul(668_265_263))
        ^ seed.wrapping_mul(2_246_822_519);
    h ^= h >> 13;
    h = h.wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Grass,
    TallGrass,
    Flower,
    Tree,
    Bush,
    Water,
    /// A rock breaking the river's surface, ripples lapping around it.
    WaterRock,
    Reed,
    Bridge,
    Path,
    /// Cobbled paving — the festival square and other village-proud ground.
    Plaza,
    Sand,
    Wall,
    Roof,
    Door,
    Floor,
    Fence,
    Cliff,
    Rock,
    Campfire,
    Lantern,
    Gate,
    Sign,
    /// The darkness outside an interior's walls — nothing there, on purpose.
    Void,
    Rug,
    Bookshelf,
    /// A shop shelf stocked with wares (loaves, jars, good intentions).
    Shelf,
    Table,
    Stool,
    BedHead,
    BedFoot,
    /// An indoor fireplace/oven, always cosily lit.
    Hearth,
    Barrel,
    Crate,
    /// A market stall counter, stacked with produce and good cheer.
    Stall,
    /// The striped canopy over a stall's counter.
    Awning,
    /// A patch of something pickable (moon-mint, so far). Press `e` beside it.
    Herb,
    /// A sturdy old chest. Locked, until it isn't.
    Chest,
    /// A hidden standing stone with a rune worth collecting. Press `e`.
    Runestone,
    /// A tall window set into a wall — sunlight (or moonlight) spills through
    /// it onto the floor below.
    Window,
    /// A framed painting hung on a wall (a Library showcase piece).
    Painting,
    /// A potted plant on display in the showcase gallery.
    Plant,
    /// A plinth bearing a curio — a rock, a relic, a small wonder.
    Pedestal,
    /// An upright piano against a wall (the Library's music corner).
    Piano,
    /// A tall case clock, ticking away the reading hours.
    Clock,
}

impl Tile {
    pub fn walkable(self) -> bool {
        matches!(
            self,
            Tile::Grass
                | Tile::TallGrass
                | Tile::Flower
                | Tile::Path
                | Tile::Plaza
                | Tile::Bridge
                | Tile::Sand
                | Tile::Floor
                | Tile::Door
                | Tile::Rug
        )
    }
}

/// What the world looks like past the edges of the authored map. The renderer
/// samples this for out-of-bounds cells so an oversized view never shows bars.
#[derive(Clone, Copy)]
pub enum Border {
    Forest,
    Meadow,
    Cliffs,
    /// Interiors: past the walls there is only warm, harmless darkness.
    Void,
}

impl Border {
    pub fn tile(self, x: i32, y: i32, seed: u32) -> Tile {
        let h = hash2(x, y, seed) % 100;
        match self {
            Border::Void => Tile::Void,
            Border::Forest => match h {
                0..=79 => Tile::Tree,
                80..=89 => Tile::Bush,
                _ => Tile::Grass,
            },
            Border::Meadow => match h {
                0..=9 => Tile::Tree,
                10..=24 => Tile::TallGrass,
                25..=29 => Tile::Reed,
                _ => Tile::Grass,
            },
            Border::Cliffs => match h {
                0..=74 => Tile::Cliff,
                75..=89 => Tile::Rock,
                _ => Tile::Grass,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Petals,
    Fireflies,
    Rain,
    Mist,
}

/// Stepping onto `at` whisks the player to `to_pos` in zone `to_zone` — how
/// doors lead into houses and back out again.
#[derive(Clone, Copy)]
pub struct Warp {
    pub at: (i32, i32),
    pub to_zone: usize,
    pub to_pos: (i32, i32),
}

pub struct Zone {
    pub id: usize,
    pub name: &'static str,
    pub tiles: Vec<Tile>,
    pub spawn: (i32, i32),
    /// Representative gate tile; walking onto any Gate tile tries to leave east.
    pub gate: Option<(i32, i32)>,
    pub locked_msg: &'static str,
    pub unlock_msg: &'static str,
    /// Every place keeps its own sky: weather never changes within a zone,
    /// and interiors (`None`) have no sky at all.
    pub weather: Option<Weather>,
    /// Fixed time of day for this place, 0.0 = deep night .. 1.0 = high noon.
    pub daylight: f32,
    /// True for rooms behind doors: no sky, no borders worth wandering to.
    pub interior: bool,
    pub border: Border,
    pub seed: u32,
    pub warps: Vec<Warp>,
    pub npcs: Vec<Npc>,
    pub critters: Vec<Critter>,
    pub signs: Vec<Sign>,
}

impl Zone {
    pub fn tile(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= MAP_W || y >= MAP_H {
            self.border.tile(x, y, self.seed)
        } else {
            self.tiles[(y * MAP_W + x) as usize]
        }
    }

    pub fn npc_at(&self, x: i32, y: i32) -> Option<&Npc> {
        self.npcs.iter().find(|n| n.pos == (x, y))
    }

    pub fn sign_at(&self, x: i32, y: i32) -> Option<&Sign> {
        self.signs.iter().find(|s| s.pos == (x, y))
    }

    pub fn warp_at(&self, x: i32, y: i32) -> Option<Warp> {
        self.warps.iter().copied().find(|w| w.at == (x, y))
    }
}

pub struct MapBuilder {
    pub tiles: Vec<Tile>,
    seed: u32,
}

impl MapBuilder {
    pub fn new(seed: u32) -> Self {
        Self {
            tiles: vec![Tile::Grass; (MAP_W * MAP_H) as usize],
            seed,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= MAP_W || y >= MAP_H {
            Tile::Tree
        } else {
            self.tiles[(y * MAP_W + x) as usize]
        }
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        if x >= 0 && y >= 0 && x < MAP_W && y < MAP_H {
            self.tiles[(y * MAP_W + x) as usize] = tile;
        }
    }

    /// Scatter `tile` over grass cells inside the region with `permille` density.
    pub fn scatter(&mut self, tile: Tile, permille: u32, region: (i32, i32, i32, i32)) {
        let (rx, ry, rw, rh) = region;
        for y in ry..ry + rh {
            for x in rx..rx + rw {
                if self.get(x, y) == Tile::Grass
                    && hash2(x, y, self.seed ^ tile_seed(tile)) % 1000 < permille
                {
                    self.set(x, y, tile);
                }
            }
        }
    }

    pub fn scatter_all(&mut self, tile: Tile, permille: u32) {
        self.scatter(tile, permille, (0, 0, MAP_W, MAP_H));
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, tile: Tile) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.set(xx, yy, tile);
            }
        }
    }

    /// A ragged band of border trees/cliffs around the map edge so the inside
    /// blends into the out-of-bounds filler.
    pub fn edge_band(&mut self, tile: Tile, depth: i32) {
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                let d = x.min(y).min(MAP_W - 1 - x).min(MAP_H - 1 - y);
                let ragged = depth + (hash2(x, y, self.seed ^ 0xE06E) % 3) as i32 - 1;
                if d < ragged && self.get(x, y) == Tile::Grass {
                    self.set(x, y, tile);
                }
            }
        }
    }

    /// Carve a winding road through the waypoints (L-shaped segments), clearing
    /// obstacles just off the roadside so it always reads as passable.
    pub fn road(&mut self, waypoints: &[(i32, i32)]) {
        let mut pts: Vec<(i32, i32)> = Vec::new();
        for pair in waypoints.windows(2) {
            let (x0, y0) = pair[0];
            let (x1, y1) = pair[1];
            let sx = if x1 >= x0 { 1 } else { -1 };
            for x in step_range(x0, x1, sx) {
                pts.push((x, y0));
            }
            let sy = if y1 >= y0 { 1 } else { -1 };
            for y in step_range(y0, y1, sy) {
                pts.push((x1, y));
            }
        }
        for &(x, y) in &pts {
            for dy in -2..=2 {
                for dx in -2..=2i32 {
                    let (nx, ny) = (x + dx, y + dy);
                    if dx.abs() <= 1 && dy.abs() <= 1 {
                        self.set(nx, ny, Tile::Path);
                    } else if !matches!(self.get(nx, ny), Tile::Path | Tile::Water | Tile::Bridge) {
                        self.set(nx, ny, Tile::Grass);
                    }
                }
            }
        }
    }

    /// A river running top-to-bottom, meandering around `center_x`.
    pub fn river(&mut self, center_x: i32, amplitude: f32, half_width: i32) {
        for y in 0..MAP_H {
            let cx = center_x + (amplitude * (y as f32 / 11.0).sin()) as i32;
            for x in cx - half_width..=cx + half_width {
                // Mostly open water, with the odd rock breaking the surface
                // (kept off the banks so the shoreline stays clean).
                let rock = x.abs_diff(cx) < half_width as u32
                    && hash2(x, y, self.seed ^ 0x0C0C) % 1000 < 30;
                self.set(x, y, if rock { Tile::WaterRock } else { Tile::Water });
            }
            for &x in &[cx - half_width - 1, cx + half_width + 1] {
                if self.get(x, y) == Tile::Grass && hash2(x, y, self.seed ^ 0x0EED) % 100 < 45 {
                    self.set(x, y, Tile::Reed);
                }
            }
        }
    }

    /// Stamp a prefab drawn as text. Space = leave alone.
    pub fn stamp(&mut self, x: i32, y: i32, art: &str) {
        for (dy, line) in art.lines().enumerate() {
            for (dx, c) in line.chars().enumerate() {
                if let Some(tile) = char_tile(c) {
                    self.set(x + dx as i32, y + dy as i32, tile);
                }
            }
        }
    }

    /// Make sure a spot (and a ring around it) is standable: anything that
    /// blocks walking becomes grass; paths, plazas and flowers are left alone.
    pub fn clearing(&mut self, x: i32, y: i32, r: i32) {
        for dy in -r..=r {
            for dx in -r..=r {
                let (nx, ny) = (x + dx, y + dy);
                if !self.get(nx, ny).walkable() {
                    self.set(nx, ny, Tile::Grass);
                }
            }
        }
    }
}

fn step_range(a: i32, b: i32, step: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let mut x = a;
    loop {
        v.push(x);
        if x == b {
            break;
        }
        x += step;
    }
    v
}

fn tile_seed(tile: Tile) -> u32 {
    tile as u32 * 0x9E37
}

pub fn char_tile(c: char) -> Option<Tile> {
    Some(match c {
        'g' => Tile::Grass,
        ',' => Tile::TallGrass,
        '*' => Tile::Flower,
        'T' => Tile::Tree,
        'b' => Tile::Bush,
        '~' => Tile::Water,
        '"' => Tile::Reed,
        '=' => Tile::Bridge,
        ':' => Tile::Path,
        '_' => Tile::Sand,
        '#' => Tile::Wall,
        'r' => Tile::Roof,
        '+' => Tile::Door,
        '.' => Tile::Floor,
        'f' => Tile::Fence,
        '^' => Tile::Cliff,
        '%' => Tile::Rock,
        '&' => Tile::Campfire,
        'L' => Tile::Lantern,
        'G' => Tile::Gate,
        '!' => Tile::Sign,
        'v' => Tile::Void,
        'R' => Tile::Rug,
        'B' => Tile::Bookshelf,
        'S' => Tile::Shelf,
        't' => Tile::Table,
        's' => Tile::Stool,
        'n' => Tile::BedHead,
        'u' => Tile::BedFoot,
        'h' => Tile::Hearth,
        'o' => Tile::Barrel,
        'x' => Tile::Crate,
        'C' => Tile::Chest,
        '@' => Tile::Runestone,
        'W' => Tile::Window,
        'A' => Tile::Painting,
        'P' => Tile::Plant,
        'E' => Tile::Pedestal,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic_and_spread() {
        assert_eq!(hash2(3, 7, 42), hash2(3, 7, 42));
        assert_ne!(hash2(3, 7, 42), hash2(4, 7, 42));
    }

    #[test]
    fn road_is_walkable_between_waypoints() {
        let mut b = MapBuilder::new(1);
        b.rect(0, 0, MAP_W, MAP_H, Tile::Tree);
        b.road(&[(2, 10), (60, 10), (60, 30), (120, 30)]);
        assert_eq!(b.get(30, 10), Tile::Path);
        assert_eq!(b.get(60, 20), Tile::Path);
        assert_eq!(b.get(90, 30), Tile::Path);
    }

    #[test]
    fn out_of_bounds_uses_border_filler() {
        let zone = crate::world::zones::zones().remove(0);
        // Never panics, always returns some scenery tile.
        let t = zone.tile(-50, -50);
        assert!(!matches!(t, Tile::Gate));
        let _ = zone.tile(MAP_W + 100, MAP_H + 100);
    }
}
