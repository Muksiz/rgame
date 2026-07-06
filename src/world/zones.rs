use crate::world::entity::{Critter, CritterKind, Npc, Sign};
use crate::world::map::{Border, MAP_H, MAP_W, MapBuilder, Tile, Warp, Weather, Zone};

const LIBRARY: &str = concat!(
    "     rrrrrrrrrrrrrrrr     \n",
    "   rrrrrrrrrrrrrrrrrrrr   \n",
    " rrrrrrrrrrrrrrrrrrrrrrrr \n",
    "##########################\n",
    "#........................#\n",
    "#........................#\n",
    "#........................#\n",
    "############++############\n",
    "gggggggggggL::Lggggggggggg",
);

const CAVE: &str = concat!("  ^^^^^  \n", "^^^^^^^^^\n", "^^^% %^^^\n", "^^%   %^^",);

// ── the rooms behind the doors ──────────────────────────────────────────────
// Interiors are their own zones: a room stamped in the middle of a map of
// Void, with a warp on each door tile leading back out.

const BAKERY_ROOM: &str = concat!(
    "###############\n",
    "#.hh......SSS.#\n",
    "#.............#\n",
    "#..stts...oo..#\n",
    "#.........ox..#\n",
    "#....RR.......#\n",
    "#....RR.......#\n",
    "#.............#\n",
    "#######+#######",
);

const COTTAGE_ROOM: &str = concat!(
    "###########\n",
    "#.nh......#\n",
    "#.u....ts.#\n",
    "#.........#\n",
    "#...RR....#\n",
    "#...RR....#\n",
    "#.........#\n",
    "#####+#####",
);

const WORKSHOP_ROOM: &str = concat!(
    "#############\n",
    "#.nh.....xx.#\n",
    "#.u......xx.#\n",
    "#...........#\n",
    "#..tt....s..#\n",
    "#..tt.......#\n",
    "#....RR.....#\n",
    "#...........#\n",
    "######+######",
);

// The door in the back wall leads down to the cellar (dark — bring a light).
const STORE_ROOM: &str = concat!(
    "######+######\n",
    "#.oo.....oo.#\n",
    "#.oox....xo.#\n",
    "#...........#\n",
    "#..xx.......#\n",
    "#........t..#\n",
    "#...........#\n",
    "#...........#\n",
    "######+######",
);

const CELLAR_ROOM: &str = concat!(
    "#############\n",
    "#:::::::::::#\n",
    "#:oo:::::xx:#\n",
    "#::::::::::C#\n",
    "#:x:::::::::#\n",
    "#:::::::::::#\n",
    "######+######",
);

// The rooms the woods took back: bare boards gone to trodden earth in
// patches, whatever furniture wasn't worth carrying, and an old note each.
const RUIN_ROOM: &str = concat!(
    "#########\n",
    "#.::..o.#\n",
    "#:..t..:#\n",
    "#..::...#\n",
    "#.x...:.#\n",
    "####+####",
);

const LODGE_ROOM: &str = concat!(
    "###########\n",
    "#.o..:..x.#\n",
    "#::..t....#\n",
    "#....:..::#\n",
    "#.n......:#\n",
    "#.u...x...#\n",
    "#####+#####",
);

const CAVE_ROOM: &str = concat!(
    "  %%%%%%%%%%%  \n",
    " %%:::::::::%% \n",
    " %@::::::::::% \n",
    " %::~~~::::::% \n",
    " %::~~~:::&::% \n",
    " %:::::::::::% \n",
    " %%:::::::!:%% \n",
    "  %%%%:::%%%%  \n",
    "     %:::%     \n",
    "     %%:%%     ",
);

// Zone indices: the four overworld zones, then every room behind a door.
pub const EMBERWICK: usize = 0;
pub const WHISPERING_WOODS: usize = 1;
pub const SILVERFORD: usize = 2;
pub const HEARTHSPIRE: usize = 3;
pub const BAKERY: usize = 4;
pub const SORREL_COTTAGE: usize = 5;
pub const CARPENTER_HOUSE: usize = 6;
pub const TILLY_COTTAGE: usize = 7;
pub const STOREHOUSE: usize = 8;
pub const ECHO_CAVE: usize = 9;
pub const GREAT_LIBRARY: usize = 10;
pub const STOREHOUSE_CELLAR: usize = 11;
pub const WOODS_RUIN: usize = 12;
pub const WOODS_LODGE: usize = 13;

/// The lived-in rooms with a fire going — the shell loops a soft hearth
/// crackle in these instead of leaving the indoors dead silent. Caves, the
/// bare storehouse and the moss-eaten ruins stay quiet; they have no fire lit.
pub fn has_hearth(zone: usize) -> bool {
    matches!(
        zone,
        BAKERY | SORREL_COTTAGE | CARPENTER_HOUSE | TILLY_COTTAGE | GREAT_LIBRARY
    )
}

/// Places with no light of their own: only the storm-lantern gets you in.
pub fn needs_light(zone: usize) -> bool {
    matches!(zone, ECHO_CAVE | STOREHOUSE_CELLAR)
}

// Every interior room is stamped at the same spot mid-map (each has its own
// map, so they never collide) — the camera centers on it from any door.
const ROOM_AT: (i32, i32) = (112, 28);

// Exterior door tiles (derived from the house stamps in each zone builder).
const BAKERY_DOOR: (i32, i32) = (66, 21);
const SORREL_DOOR: (i32, i32) = (93, 16);
const CARPENTER_DOOR: (i32, i32) = (130, 23);
const TILLY_DOOR: (i32, i32) = (77, 48);
const STOREHOUSE_DOOR: (i32, i32) = (136, 49);
const CAVE_MOUTH: (i32, i32) = (122, 54);
const LIBRARY_DOORS: [(i32, i32); 2] = [(210, 32), (211, 32)];
// The abandoned houses in the Whispering Woods (prefab origin + door offset).
const RUIN_STONE_DOOR: (i32, i32) = (67, 13);
const RUIN_LODGE_DOOR: (i32, i32) = (173, 49);

// Interior door tiles (stamp origin + the '+' offset in each room's art).
const BAKERY_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 7, ROOM_AT.1 + 8);
const COTTAGE_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 5, ROOM_AT.1 + 7);
const WORKSHOP_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1 + 8);
const STORE_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1 + 8);
const STORE_CELLAR_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1);
const CELLAR_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1 + 6);
// The Great Library is built room-by-room (see `great_library`); it is 40×24,
// stamped at ROOM_AT, with its two entrance doors in the bottom wall.
const LIBRARY_W: i32 = 40;
const LIBRARY_H: i32 = 24;
const LIBRARY_ROOM_DOORS: [(i32, i32); 2] = [
    (ROOM_AT.0 + 19, ROOM_AT.1 + LIBRARY_H - 1),
    (ROOM_AT.0 + 20, ROOM_AT.1 + LIBRARY_H - 1),
];
const CAVE_ROOM_EXIT: (i32, i32) = (ROOM_AT.0 + 7, ROOM_AT.1 + 9);
const RUIN_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 4, ROOM_AT.1 + 5);
const LODGE_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 5, ROOM_AT.1 + 6);

/// Stepping on the outside door lands just inside the room's own door...
fn enter(outside: (i32, i32), interior: usize, inside_door: (i32, i32)) -> Warp {
    Warp {
        at: outside,
        to_zone: interior,
        to_pos: (inside_door.0, inside_door.1 - 1),
    }
}

/// ...and the room's door leads back to just outside the house.
fn exit(inside_door: (i32, i32), back_to: usize, outside: (i32, i32)) -> Warp {
    Warp {
        at: inside_door,
        to_zone: back_to,
        to_pos: (outside.0, outside.1 + 1),
    }
}

pub fn zones() -> Vec<Zone> {
    vec![
        emberwick(),
        whispering_woods(),
        silverford(),
        hearthspire(),
        bakery(),
        sorrel_cottage(),
        carpenter_house(),
        tilly_cottage(),
        storehouse(),
        echo_cave(),
        great_library(),
        storehouse_cellar(),
        woods_ruin(),
        woods_lodge(),
    ]
}

/// Where the seven standing runestones were planted, in stone-id order
/// (`content/stones.rs` ids 1..=7; the eighth sleeps in the cellar chest).
pub fn runestone_spots() -> [(usize, (i32, i32)); 7] {
    [
        (EMBERWICK, (60, 46)),         // among the chickens
        (WHISPERING_WOODS, (46, 15)),  // the edge of Wren's clearing
        (WHISPERING_WOODS, (217, 10)), // Old Nettle's hollow
        (SILVERFORD, (124, 59)),       // half-sunk in Morrow's beach
        (HEARTHSPIRE, (130, 10)),      // alone in the northern crags
        (ECHO_CAVE, at(2, 2)),         // humming in the dark
        (GREAT_LIBRARY, at(34, 19)),   // catalogued in the showcase gallery
    ]
}

/// The stone id (1-based) standing at this spot, if any.
pub fn runestone_id(zone: usize, pos: (i32, i32)) -> Option<u8> {
    runestone_spots()
        .iter()
        .position(|&(z, p)| z == zone && p == pos)
        .map(|i| i as u8 + 1)
}

/// A tree line across the map with an opening for the gate tiles, so the gate
/// can't simply be strolled around.
fn barrier(b: &mut MapBuilder, x: i32, gate_ys: std::ops::RangeInclusive<i32>, tile: Tile) {
    for y in 0..MAP_H {
        if gate_ys.contains(&y) {
            b.set(x, y, Tile::Gate);
        } else {
            b.set(x, y, tile);
        }
    }
}

fn emberwick() -> Zone {
    let seed = 11;
    let mut b = MapBuilder::new(seed);
    b.scatter_all(Tile::Tree, 35);
    b.scatter_all(Tile::Bush, 18);
    b.scatter_all(Tile::TallGrass, 70);
    b.scatter_all(Tile::Flower, 30);
    b.edge_band(Tile::Tree, 3);

    b.road(&[
        (0, 38),
        (40, 38),
        (55, 30),
        (100, 30),
        (115, 40),
        (170, 40),
        (205, 36),
        (239, 36),
    ]);

    // Houses along the road, each with a lane down to it. All of them are
    // perspective-drawn prefabs (see MapBuilder::prefab); the enterable ones
    // put their walkable doorway exactly where the old stamps kept theirs,
    // so every Warp constant below still holds.
    use crate::gfx::atlas;
    let door = Some(atlas::HOUSE_DOOR_AT);
    b.prefab(64, 17, atlas::HOUSE_A, atlas::HOUSE_SIZE, door); // the bakery
    b.prefab(91, 12, atlas::HOUSE_A, atlas::HOUSE_SIZE, door); // Granny Sorrel's
    b.prefab(128, 19, atlas::HOUSE_A, atlas::HOUSE_SIZE, door); // Alder's workshop
    b.prefab(75, 44, atlas::HOUSE_A, atlas::HOUSE_SIZE, door); // Tilly's cottage
    b.prefab(134, 45, atlas::HOUSE_B_DOOR, atlas::HOUSE_SIZE, door); // the storehouse
    b.road(&[(66, 23), (66, 29)]);
    b.road(&[(130, 26), (130, 39)]);

    // Homes nobody needs to enter — doors shut, curtains drawn — plus the
    // old lean-to shed behind the storehouse. A real village's worth of them
    // now: the Ninja Adventure homes (thatched, plain, flat-roofed, the tall
    // townhouse, a shopfront and the tavern) cluster tight along the roads
    // and around the square, so Emberwick reads as streets rather than five
    // quest-relevant houses in a field.
    b.prefab(150, 14, atlas::HOUSE_A_SHUT, atlas::HOUSE_SIZE, None);
    b.prefab(160, 43, atlas::HOUSE_B, atlas::HOUSE_SIZE, None);
    b.prefab(143, 46, atlas::SHED, atlas::SHED_SIZE, None);
    // North of the main road: a row of homes between the bakery and the well.
    b.prefab(56, 24, atlas::NA_HOUSE_PLAIN, atlas::NA_HOUSE_SIZE, None);
    b.prefab(72, 25, atlas::NA_HOUSE_THATCH, atlas::NA_HOUSE_SIZE, None);
    b.prefab(84, 20, atlas::NA_HOUSE_FLAT, atlas::NA_HOUSE_SIZE, None);
    b.prefab(100, 17, atlas::NA_HOUSE_TALL, atlas::NA_TALL_SIZE, None);
    // Around the square: a shopfront on the west edge, the tavern by the
    // fountain, and a shut cottage anchoring the south-west corner.
    b.prefab(76, 33, atlas::NA_SHOP, atlas::NA_SHOP_SIZE, None);
    b.prefab(98, 33, atlas::NA_TAVERN, atlas::NA_SHOP_SIZE, None);
    b.prefab(67, 33, atlas::HOUSE_A_SHUT, atlas::HOUSE_SIZE, None);
    // The east lane, both sides of the road out toward the woods.
    b.prefab(118, 33, atlas::NA_HOUSE_THATCH, atlas::NA_HOUSE_SIZE, None);
    b.prefab(146, 33, atlas::HOUSE_A_SHUT, atlas::HOUSE_SIZE, None);
    b.prefab(152, 36, atlas::NA_HOUSE_FLAT, atlas::NA_HOUSE_SIZE, None);
    // And the south quarter, filling in around Tilly and the storehouse.
    b.prefab(109, 43, atlas::NA_SHOP, atlas::NA_SHOP_SIZE, None);
    b.prefab(122, 43, atlas::NA_HOUSE_PLAIN, atlas::NA_HOUSE_SIZE, None);
    b.prefab(98, 47, atlas::HOUSE_A_SHUT, atlas::HOUSE_SIZE, None);

    // Old growth: big blossoming trees between the houses, and garden
    // bushes tucked against the lanes.
    b.prefab(48, 24, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(60, 10, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(116, 12, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(124, 25, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(68, 52, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(90, 52, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(118, 52, atlas::TREE_BIG_ORANGE, atlas::TREE_BIG_SIZE, None);
    b.prefab(170, 30, atlas::TREE_BIG_ORANGE, atlas::TREE_BIG_SIZE, None);
    // And more of them out where the village thins (a playtest asked for
    // more old growth): the west approach, the north meadows, the south
    // fields and the long east stretch toward the woods.
    b.prefab(18, 26, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(30, 46, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(26, 12, atlas::TREE_BIG_ORANGE, atlas::TREE_BIG_SIZE, None);
    b.prefab(140, 8, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(74, 6, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(150, 56, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(186, 16, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(196, 44, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(210, 26, atlas::TREE_BIG_ORANGE, atlas::TREE_BIG_SIZE, None);
    b.prefab(50, 34, atlas::BUSH_BIG, atlas::BUSH_BIG_SIZE, None);
    b.prefab(102, 34, atlas::BUSH_BIG, atlas::BUSH_BIG_SIZE, None);
    b.prefab(89, 24, atlas::BUSH_BIG, atlas::BUSH_BIG_SIZE, None);

    // Festival square, now a proper market: cobbles, the unlit lantern, a
    // cosy campfire, the big blue-awninged market stall, and a round
    // fountain burbling at the east end. The square hangs just south of the
    // main road (which runs y29–31 here) rather than paving over it, so the
    // slabs stop at the road's edge and the lane behind the stall stays
    // open road, one texture end to end.
    b.rect(80, 32, 15, 6, Tile::Plaza);
    b.prefab(80, 32, atlas::STALL, atlas::STALL_SIZE, None);
    b.prefab(89, 32, atlas::FOUNTAIN, atlas::FOUNTAIN_SIZE, None);
    b.set(86, 34, Tile::Lantern);
    b.set(90, 36, Tile::Campfire);
    for x in [79, 95] {
        for y in [32, 34, 36] {
            b.set(x, y, Tile::Flower);
        }
    }
    // Flowerbeds flanking the square's south edge, dense in the middle.
    b.rect(81, 38, 3, 2, Tile::Flower);
    b.rect(88, 38, 3, 2, Tile::Flower);

    // The old well, up a short lane: a proper roofed draw-well now, not a
    // ring of rocks around a puddle.
    b.prefab(109, 22, atlas::WELL, atlas::WELL_SIZE, None);
    b.road(&[(110, 26), (110, 30)]);

    // Village clutter: barrels and crates left out beside the doors.
    b.set(63, 22, Tile::Barrel);
    b.set(69, 22, Tile::Crate);
    b.set(139, 50, Tile::Crate);
    b.set(140, 50, Tile::Barrel);

    // Chicken pen.
    b.rect(52, 44, 12, 7, Tile::Grass);
    for x in 52..64 {
        b.set(x, 44, Tile::Fence);
        b.set(x, 50, Tile::Fence);
    }
    for y in 44..=50 {
        b.set(52, y, Tile::Fence);
        b.set(63, y, Tile::Fence);
    }
    b.set(57, 44, Tile::Grass); // pen opening

    // Fallen oak blocking the east road (plus a tree line so it can't be skirted).
    barrier(&mut b, 231, 34..=38, Tile::Tree);

    b.set(10, 36, Tile::Sign);
    b.set(224, 34, Tile::Sign);

    b.clearing(86, 32, 1); // Elder Rowan, on the square by his lantern
    b.clearing(66, 25, 1);
    b.clearing(112, 27, 1);
    b.clearing(85, 36, 1); // Tansy, by the market stall
    b.clearing(222, 36, 1); // Watchman Fitch, by the east archway
    b.clearing(104, 22, 1); // Toll-keeper Hobb, just shy of the well
    b.clearing(116, 22, 1); // Cartographer Reed, just past the well

    // Granny Sorrel's lane, and a clear doorstep in front of every front door.
    b.road(&[(93, 18), (93, 29)]);
    for door in [
        BAKERY_DOOR,
        SORREL_DOOR,
        CARPENTER_DOOR,
        TILLY_DOOR,
        STOREHOUSE_DOOR,
    ] {
        b.clearing(door.0, door.1 + 1, 0);
    }

    // A runestone in the chicken pen, glossy from generations of hens.
    b.set(60, 46, Tile::Runestone);

    Zone {
        id: 0,
        name: "Emberwick Village",
        tiles: b.tiles,
        spawn: (6, 38),
        gate: Some((231, 36)),
        locked_msg: "A fallen oak blocks the road. Maybe help the villagers first — starting with Elder Rowan at the festival square.",
        unlock_msg: "The villagers roll the old oak aside, cheering. The road east lies open!",
        weather: None,
        daylight: 0.95,
        interior: false,
        border: Border::Forest,
        seed,
        warps: vec![
            enter(BAKERY_DOOR, BAKERY, BAKERY_ROOM_DOOR),
            enter(SORREL_DOOR, SORREL_COTTAGE, COTTAGE_ROOM_DOOR),
            enter(CARPENTER_DOOR, CARPENTER_HOUSE, WORKSHOP_ROOM_DOOR),
            enter(TILLY_DOOR, TILLY_COTTAGE, COTTAGE_ROOM_DOOR),
            enter(STOREHOUSE_DOOR, STOREHOUSE, STORE_ROOM_DOOR),
        ],
        npcs: vec![
            Npc {
                name: "Elder Rowan",
                pos: (86, 32),
                quest: Some(1),
                idle: &[
                    "The lantern has hung dark for years. Tonight, maybe, it glows again.",
                    "The festival lantern's been glowing since you lit it — folk keep drifting to the square just to stand in the warm of it. You did that.",
                ],
            },
            Npc {
                name: "Tansy",
                pos: (85, 36),
                quest: Some(2),
                idle: &[
                    "Nine more apples came in this morning and my sign won't say so. Rude, honestly.",
                    "Twenty-one apples, seven pears, and the sign finally agrees with me. You're good at this.",
                ],
            },
            Npc {
                name: "Baker Poppy",
                pos: (66, 25),
                quest: Some(3),
                idle: &[
                    "Smell that? Honey-oat loaves. The recipe is older than the village.",
                    "The ledger balances to the last crumb now, thanks to you. Here — mind the heat — a loaf for the road.",
                ],
            },
            Npc {
                name: "Watchman Fitch",
                pos: (222, 36),
                quest: Some(4),
                idle: &[
                    "Everyone pays the toll. Well. Almost everyone. It's complicated.",
                    "The gate-rune waves the right folk through now, free and fair. Pigeon's still recovering from the shock of my smiling.",
                ],
            },
            Npc {
                name: "Toll-keeper Hobb",
                pos: (104, 22),
                quest: Some(5),
                idle: &[
                    "Four coins, four slots, thirty years running. Some things shouldn't wobble.",
                    "Carved in proper now — const, not chalk. I could kiss you, but I won't. Have a good day instead.",
                ],
            },
            Npc {
                name: "Well-keeper Bram",
                pos: (112, 27),
                quest: Some(6),
                idle: &[
                    "Deepest well in the valley, this. Probably. Nobody's ever checked.",
                    "Water's running clear and cold again since you sorted the well. The whole lane's grateful — and the storm-lantern's yours to keep.",
                ],
            },
            Npc {
                name: "Cartographer Reed",
                pos: (116, 22),
                quest: Some(7),
                idle: &[
                    "A landmark's just two numbers that agree to travel together. Simple, once it works.",
                    "Every pin sits true now. This map's the first thing I've trusted in months — thank you.",
                ],
            },
            // Market folk — no errands, just the pleasant noise of a square.
            Npc {
                name: "Greengrocer Marla",
                pos: (85, 33),
                quest: None,
                idle: &[
                    "Apples from the east orchard, pears from the west. The stall knows the difference even when I don't.",
                ],
            },
            Npc {
                name: "Old Tam",
                pos: (93, 34),
                quest: None,
                idle: &["I remember when this fountain was a bucket. Progress! Mostly wetter."],
            },
            Npc {
                name: "Juno",
                pos: (87, 36),
                quest: None,
                idle: &[
                    "If you put a leaf in the fountain it sails ALL the way around before it sinks. I've timed it.",
                ],
            },
        ],
        critters: vec![
            Critter::new(CritterKind::Chicken, (55, 46)),
            Critter::new(CritterKind::Chicken, (58, 48)),
            Critter::new(CritterKind::Chicken, (61, 47)),
            Critter::new(CritterKind::Chicken, (56, 49)),
        ],
        signs: vec![
            Sign {
                pos: (10, 36),
                text: "Welcome to Emberwick. Mind the chickens — they mind you.",
            },
            Sign {
                pos: (224, 34),
                text: "East: the Whispering Woods. Bring a kind word for the trees.",
            },
        ],
    }
}

fn whispering_woods() -> Zone {
    let seed = 22;
    let mut b = MapBuilder::new(seed);
    // The deep woods press close: half again as many trees as the old
    // airy version, hardly a flower, and the light stays down at dusk.
    b.scatter_all(Tile::Tree, 560);
    b.scatter_all(Tile::Bush, 80);
    b.scatter_all(Tile::TallGrass, 90);
    b.scatter_all(Tile::Flower, 4);
    b.scatter_all(Tile::Rock, 12); // mossgrown boulders between the trees
    b.edge_band(Tile::Tree, 4);

    // Ambrose's meadow (before the road, which passes through it).
    b.rect(162, 22, 26, 13, Tile::Grass);
    b.scatter(Tile::TallGrass, 220, (162, 22, 26, 13));
    b.scatter(Tile::Flower, 80, (162, 22, 26, 13));
    for x in 162..188 {
        b.set(x, 22, Tile::Fence);
    }

    // No proper road here — a single-file footpath that winds where the
    // trees allow, and the forest presses right up to the wayside.
    b.trail(&[
        (0, 36),
        (14, 35),
        (30, 36),
        (45, 22),
        (56, 19),
        (70, 23),
        (90, 22),
        (105, 44),
        (112, 42),
        (126, 45),
        (150, 44),
        (157, 38),
        (165, 30),
        (180, 32),
        (196, 28),
        (210, 30),
        (222, 33),
        (233, 32),
        (239, 32),
    ]);
    b.clearing(8, 35, 1); // the welcome sign keeps its little glade

    // Wren's stump clearing.
    b.clearing(49, 18, 4);
    b.set(51, 16, Tile::Rock);
    b.scatter(Tile::Flower, 140, (45, 14, 9, 9));

    // Maren's mushroom hollow.
    b.clearing(101, 47, 4);
    b.scatter(Tile::Bush, 200, (97, 43, 9, 9));

    // The echo cave, just off the road — its mouth leads inside. The footpath
    // leaves the road, loops around the hill and walks up into the mouth from
    // the south; it goes down first so the stamp's rocky jaw closes over any
    // stray paving while the mouth itself stays walkable.
    b.trail(&[
        (CAVE_MOUTH.0, 42),
        (112, 46),
        (112, 57),
        (CAVE_MOUTH.0, 57),
        (CAVE_MOUTH.0, CAVE_MOUTH.1 + 1),
    ]);
    b.clearing(111, 52, 1); // the moon-mint stays a step off the cave path
    b.stamp(118, 52, CAVE);

    // Mossy old gate across the east road.
    barrier(&mut b, 233, 30..=34, Tile::Tree);

    b.set(8, 34, Tile::Sign);
    b.set(120, 41, Tile::Sign);

    b.clearing(49, 20, 1);
    b.clearing(101, 46, 1);
    b.clearing(172, 28, 1);
    b.clearing(53, 20, 1); // Pip, playing near Wren's clearing
    b.clearing(105, 46, 1); // Basket-weaver Briar, past the mushroom hollow
    b.clearing(200, 29, 1); // Hollow-keeper Yew, along the meadow road
    b.clearing(225, 31, 1); // Woodward Sable, near the mossy gate

    // Old Nettle's hollow, deep in the woods where no road goes: a winding
    // thread of gaps in the trees, findable but never signposted.
    for (x, y) in [
        (210, 27),
        (210, 25),
        (209, 23),
        (210, 21),
        (211, 19),
        (212, 17),
        (213, 15),
        (214, 13),
    ] {
        b.clearing(x, y, 1);
    }
    b.clearing(215, 12, 2);

    // Old growth: towering pines and triple crowns between the everyday
    // trees, kept well clear of the road, the clearings and Nettle's trail.
    use crate::gfx::atlas;
    b.prefab(30, 12, atlas::TREE_TALL_PINE, atlas::TREE_TALL_SIZE, None);
    b.prefab(64, 40, atlas::TREE_TALL_CANOPY, atlas::TREE_TALL_SIZE, None);
    b.prefab(76, 8, atlas::TREE_TALL_CANOPY, atlas::TREE_TALL_SIZE, None);
    b.prefab(120, 16, atlas::TREE_TALL_PINE, atlas::TREE_TALL_SIZE, None);
    b.prefab(140, 28, atlas::TREE_BIG_ORANGE, atlas::TREE_BIG_SIZE, None);
    b.prefab(152, 52, atlas::TREE_TALL_PINE, atlas::TREE_TALL_SIZE, None);
    b.prefab(
        196,
        44,
        atlas::TREE_TALL_CANOPY,
        atlas::TREE_TALL_SIZE,
        None,
    );
    b.prefab(224, 20, atlas::TREE_TALL_PINE, atlas::TREE_TALL_SIZE, None);
    // And the dead old growth between them: bare gnarled crowns and tall
    // bramble snags, thicker the deeper in you go.
    for &(x, y) in &[(38, 6), (70, 30), (160, 56), (184, 12), (226, 44)] {
        b.prefab(x, y, atlas::TREE_DEAD_BIG, atlas::TREE_DEAD_BIG_SIZE, None);
    }
    for &(x, y) in &[(14, 14), (52, 44), (90, 52), (132, 8), (200, 52)] {
        b.prefab(x, y, atlas::TREE_GNARLED, atlas::TREE_GNARLED_SIZE, None);
    }

    // Two houses the woods took back, long before anyone alive remembers:
    // a moss-eaten stone cottage north of Wren's stretch of road, and a
    // timber lodge sunk in the deep southeast. Both doorways still open —
    // dim rooms, old notes, nobody home. Unsignposted trails of gaps in
    // the trees lead to each, same as Nettle's hollow.
    b.prefab(
        66,
        11,
        atlas::RUIN_STONE,
        atlas::RUIN_SIZE,
        Some(atlas::RUIN_DOOR_AT),
    );
    for y in [15, 17, 19, 21] {
        b.clearing(67, y, 1);
    }
    b.prefab(
        172,
        47,
        atlas::RUIN_LODGE,
        atlas::RUIN_SIZE,
        Some(atlas::RUIN_DOOR_AT),
    );
    for (x, y) in [
        (173, 51),
        (170, 50),
        (168, 48),
        (166, 47),
        (163, 46),
        (160, 45),
        (158, 44),
    ] {
        b.clearing(x, y, 1);
    }

    // Moon-mint for Granny Sorrel's kettle, just off the cave path,
    // and two runestones for sharp-eyed wanderers.
    b.set(110, 52, Tile::Herb);
    b.set(46, 15, Tile::Runestone);
    b.set(217, 10, Tile::Runestone);

    // A traveller's campfire off the road, for resting the day away.
    b.clearing(140, 46, 1);
    b.set(140, 46, Tile::Campfire);

    Zone {
        id: 1,
        name: "Whispering Woods",
        tiles: b.tiles,
        spawn: (5, 36),
        gate: Some((233, 32)),
        locked_msg: "An old mossy gate, swollen shut. The woods aren't done with you yet, it seems.",
        unlock_msg: "The mossy gate creaks open, almost politely. Onward, to the river!",
        weather: Some(Weather::Fireflies),
        daylight: 0.32,
        interior: false,
        border: Border::Forest,
        seed,
        warps: vec![
            enter(CAVE_MOUTH, ECHO_CAVE, CAVE_ROOM_EXIT),
            enter(RUIN_STONE_DOOR, WOODS_RUIN, RUIN_ROOM_DOOR),
            enter(RUIN_LODGE_DOOR, WOODS_LODGE, LODGE_ROOM_DOOR),
        ],
        npcs: vec![
            Npc {
                name: "Pip",
                pos: (53, 20),
                quest: Some(8),
                idle: &[
                    "The trick is patience. They won't be caught in a hurry, fireflies.",
                    "Jar's full and glowing! Best night-light I've ever had. Thank you, truly.",
                ],
            },
            Npc {
                name: "Wren",
                pos: (49, 20),
                quest: Some(9),
                idle: &[
                    "When I grow up I'm going to walk to BOTH ends of the road.",
                    "You made the spell WORK! I've been showing everyone. Even the stump. The stump was very impressed, I could tell.",
                ],
            },
            Npc {
                name: "Basket-weaver Briar",
                pos: (105, 46),
                quest: Some(10),
                idle: &[
                    "Every basket past my table for its stamp before it goes out. Lent is not lost — that's the whole trade.",
                    "In and back out, stamped and counted, every single time now. Take an apple, go on.",
                ],
            },
            Npc {
                name: "Forager Maren",
                pos: (101, 46),
                quest: Some(11),
                idle: &[
                    "Rule one of foraging: when in doubt, don't. Rule two: see rule one.",
                    "Not one bad mushroom in the whole basket now — your sorting saw to that. The hollow smells like supper.",
                ],
            },
            Npc {
                name: "Shepherd Ambrose",
                pos: (172, 28),
                quest: Some(12),
                idle: &[
                    "*yaaawn* ...I wasn't sleeping. I was counting very slowly.",
                    "Every last sheep home and counted, and I owe the nap I'm about to take entirely to you. *yaaawn*",
                ],
            },
            Npc {
                name: "Hollow-keeper Yew",
                pos: (200, 29),
                quest: Some(13),
                idle: &[
                    "Some years three good gathering days, some years thirty. Never know till the season's out.",
                    "A winter's worth of acorns, properly counted, safely kept. The hollow thanks you. So do I.",
                ],
            },
            Npc {
                name: "Woodward Sable",
                pos: (225, 31),
                quest: Some(14),
                idle: &[
                    "Some evenings I find the old bell. Some evenings, nothing. A body can only walk three grounds a night, is the trouble.",
                    "Three grounds an evening, honestly planned. No more pretending the whole wood fits in one night. I'll find that bell yet.",
                ],
            },
            // Hidden in the deep woods; her dialogue lives in content/sides.rs.
            Npc {
                name: "Old Nettle",
                pos: (215, 12),
                quest: None,
                idle: &["Still here. So are the trees."],
            },
        ],
        critters: vec![
            Critter::new(CritterKind::Sheep, (168, 26)),
            Critter::new(CritterKind::Sheep, (176, 30)),
            Critter::new(CritterKind::Sheep, (182, 27)),
        ],
        signs: vec![
            Sign {
                pos: (8, 34),
                text: "The Whispering Woods. The trees gossip, but they mean well.",
            },
            Sign {
                pos: (120, 41),
                text: "Echo Cave, south. Please do not teach the echo any bad words.",
            },
        ],
    }
}

fn silverford() -> Zone {
    let seed = 33;
    let mut b = MapBuilder::new(seed);
    b.scatter_all(Tile::TallGrass, 150);
    b.scatter_all(Tile::Reed, 40);
    b.scatter_all(Tile::Flower, 25);
    b.scatter_all(Tile::Tree, 30);
    b.edge_band(Tile::Tree, 2);

    b.road(&[(0, 40), (60, 40), (80, 34), (140, 34)]);
    b.road(&[(170, 34), (239, 34)]);
    b.road(&[(130, 36), (130, 46), (134, 47), (139, 47)]);
    b.road(&[(138, 32), (138, 22), (146, 22)]);
    b.road(&[(114, 42), (114, 54)]);

    // The Silverford itself (drawn after the roads: the river wins) — and a
    // tributary winding in from the northwest hills to join it, so the
    // Riverlands finally earn their plural.
    b.river(155, 6.0, 5);
    b.stream(&[(0, 10), (40, 14), (80, 8), (120, 14), (155, 12)], 2);

    // The old bridge — its west end is also the gate east.
    for x in 146..=168 {
        for y in 33..=35 {
            b.set(x, y, Tile::Bridge);
        }
    }
    for y in 33..=35 {
        b.set(146, y, Tile::Gate);
    }

    // The harbor: a cobbled quay along the west bank below the bridge, three
    // timber piers walking out over the water, the ferry tied up along the
    // long one, and a couple of skiffs riding at their moorings.
    use crate::gfx::atlas;
    b.rect(139, 44, 7, 11, Tile::Plaza);
    for x in 145..=151 {
        b.set(x, 46, Tile::Pier);
    }
    for x in 145..=153 {
        b.set(x, 50, Tile::Pier);
    }
    for x in 145..=149 {
        b.set(x, 53, Tile::Pier);
    }
    b.prefab(147, 51, atlas::BOAT, atlas::BOAT_SIZE, None);
    b.prefab(147, 47, atlas::SKIFF, atlas::SKIFF_SIZE, None);
    // A third skiff upstream, pulled in by Juniper's fishing hole.
    b.prefab(157, 21, atlas::SKIFF, atlas::SKIFF_SIZE, None);

    // Morrow's little beach.
    b.rect(112, 55, 14, 6, Tile::Sand);
    b.scatter(Tile::Reed, 180, (108, 53, 22, 10));

    // Old growth along the banks: broad crowns leaning over the meadows.
    b.prefab(30, 28, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(56, 20, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);
    b.prefab(84, 44, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(
        100,
        24,
        atlas::TREE_TALL_CANOPY,
        atlas::TREE_TALL_SIZE,
        None,
    );
    b.prefab(186, 22, atlas::TREE_BIG_GREEN, atlas::TREE_BIG_SIZE, None);
    b.prefab(206, 44, atlas::TREE_BIG_PINK, atlas::TREE_BIG_SIZE, None);

    b.set(8, 38, Tile::Sign);
    b.set(140, 38, Tile::Sign);

    b.clearing(136, 45, 1);
    b.clearing(148, 22, 1);
    b.clearing(118, 57, 1);
    b.clearing(130, 42, 1); // Dockhand Fenn, on the lane down to the ferry
    b.clearing(152, 25, 1); // Net-mender Sil, just downstream of the fishing spot

    // A runestone half-sunk at the far end of Morrow's beach.
    b.set(124, 59, Tile::Runestone);

    // A driftwood campfire down on Morrow's beach, for watching the river.
    b.set(121, 58, Tile::Campfire);

    Zone {
        id: 2,
        name: "Silverford Riverlands",
        tiles: b.tiles,
        spawn: (5, 40),
        gate: Some((146, 34)),
        locked_msg: "The bridge planks are drawn up on the far side. Ferryman Wick shrugs at you meaningfully.",
        unlock_msg: "Wick lowers the planks with a satisfied nod. The far bank awaits!",
        weather: Some(Weather::Rain),
        daylight: 0.55,
        interior: false,
        border: Border::Meadow,
        seed,
        warps: vec![],
        npcs: vec![
            Npc {
                name: "Dockhand Fenn",
                pos: (130, 42),
                quest: Some(15),
                idle: &[
                    "Every crate that lands gets a record: label, weight, seal. Three facts, one crate — you'd think a ledger could keep them together.",
                    "Label, weight and seal, one bundle to a crate now. Proper paper trail. Wick's just down at the landing, if the ferry's your business.",
                ],
            },
            Npc {
                name: "Ferryman Wick",
                pos: (136, 45),
                quest: Some(16),
                idle: &[
                    "River's high today. River's always high today, if you ask the river.",
                    "Token's sorted, planks are down, and the ferry runs on time — well, on river-time. Cross whenever you like now.",
                ],
            },
            Npc {
                name: "Fisher Juniper",
                pos: (148, 22),
                quest: Some(17),
                idle: &[
                    "The trick to fishing is patience. The other trick is bait. Mostly bait.",
                    "Keep the spare rod — you've the patience for it. Reedy banks all down the river are yours to try.",
                ],
            },
            Npc {
                name: "Net-mender Sil",
                pos: (152, 25),
                quest: Some(18),
                idle: &[
                    "The tide chart only wants the first three days. The rest is just noise, far as I'm concerned.",
                    "Three days, one snug bundle, and not a number more. That's how I like my charts, and my nets.",
                ],
            },
            Npc {
                name: "Hermit Morrow",
                pos: (118, 57),
                quest: Some(19),
                idle: &[
                    "The river brings me letters. I write back, but slowly.",
                    "The letter reached the end of its sentence at last, thanks to you. The river seems pleased. So am I.",
                ],
            },
            // The harbor's own folk: two anglers who hold down the pier
            // ends, lines in the water, all day long.
            Npc {
                name: "Angler Rush",
                pos: (152, 50),
                quest: None,
                idle: &["Shh. Not because of the fish — they can't hear you. It's the principle."],
            },
            Npc {
                name: "Angler Minnow",
                pos: (149, 46),
                quest: None,
                idle: &["Caught one this big yesterday. The pier was shorter then, mind."],
            },
        ],
        critters: vec![
            Critter::new(CritterKind::Frog, (144, 42)),
            Critter::new(CritterKind::Frog, (126, 54)),
            Critter::new(CritterKind::Frog, (150, 28)),
        ],
        signs: vec![
            Sign {
                pos: (8, 38),
                text: "Silverford. The river is chatty in the rain. It's mostly saying 'shhh'.",
            },
            Sign {
                pos: (140, 38),
                text: "Bridge east to the Hearthspire road. Dock: south. Good moods: everywhere.",
            },
        ],
    }
}

fn hearthspire() -> Zone {
    let seed = 44;
    let mut b = MapBuilder::new(seed);
    b.scatter_all(Tile::TallGrass, 70);
    b.scatter_all(Tile::Rock, 45);
    b.scatter_all(Tile::Tree, 25);
    b.scatter_all(Tile::Flower, 15);
    // Craggy highland bands north and south, crumbling into loose
    // boulders where they meet the meadow.
    b.rect(0, 0, 240, 8, Tile::Cliff);
    b.rect(0, 62, 240, 8, Tile::Cliff);
    b.scatter(Tile::Rock, 250, (0, 8, 240, 6));
    b.scatter(Tile::Rock, 250, (0, 56, 240, 6));
    b.edge_band(Tile::Cliff, 2);

    b.road(&[
        (0, 36),
        (50, 36),
        (65, 50),
        (110, 50),
        (125, 22),
        (170, 22),
        (185, 34),
        (206, 34),
    ]);

    // The Great Library of Hearthspire.
    b.stamp(198, 25, LIBRARY);
    for x in 208..=211 {
        b.set(x, 34, Tile::Plaza); // cobbled forecourt up to the door
    }

    // Old growth on the highland: snowbound canopies braced against the mist.
    use crate::gfx::atlas;
    b.prefab(32, 42, atlas::TREE_TALL_SNOW, atlas::TREE_TALL_SIZE, None);
    b.prefab(60, 28, atlas::TREE_BIG_WHITE, atlas::TREE_BIG_SIZE, None);
    b.prefab(96, 34, atlas::TREE_TALL_SNOW, atlas::TREE_TALL_SIZE, None);
    b.prefab(140, 28, atlas::TREE_BIG_WHITE, atlas::TREE_BIG_SIZE, None);
    b.prefab(150, 40, atlas::TREE_TALL_SNOW, atlas::TREE_TALL_SIZE, None);
    b.prefab(180, 12, atlas::TREE_BIG_WHITE, atlas::TREE_BIG_SIZE, None);

    b.set(8, 34, Tile::Sign);
    b.set(190, 32, Tile::Sign);

    b.clearing(68, 48, 1);
    b.clearing(172, 24, 1);
    b.clearing(206, 35, 1);
    b.clearing(213, 35, 1); // Scribe Faye, at the desk near the Library forecourt

    // A lone runestone up in the northern crags, with just enough of a
    // scramble cleared through the rocks to reach it.
    for (x, y) in [(129, 18), (130, 15), (129, 13), (130, 11)] {
        b.clearing(x, y, 1);
    }
    b.set(130, 10, Tile::Runestone);

    // A wayfarer's campfire beside the highland road, ringed with loose stone.
    b.clearing(88, 52, 1);
    b.set(88, 52, Tile::Campfire);

    Zone {
        id: 3,
        name: "Hearthspire Approach",
        tiles: b.tiles,
        spawn: (5, 36),
        gate: None,
        locked_msg: "",
        unlock_msg: "",
        weather: Some(Weather::Mist),
        daylight: 0.3,
        interior: false,
        border: Border::Cliffs,
        seed,
        warps: vec![
            enter(LIBRARY_DOORS[0], GREAT_LIBRARY, LIBRARY_ROOM_DOORS[0]),
            enter(LIBRARY_DOORS[1], GREAT_LIBRARY, LIBRARY_ROOM_DOORS[1]),
        ],
        npcs: vec![
            Npc {
                name: "Archivist Elm",
                pos: (68, 48),
                quest: Some(20),
                idle: &[
                    "I catalogue everything. Even this conversation. Especially this conversation.",
                    "The lost book is catalogued and shelved, and I've noted your name beside it. In triplicate. It seemed important.",
                ],
            },
            Npc {
                name: "The Stone Golem",
                pos: (172, 24),
                quest: Some(21),
                idle: &[
                    "...zzz... shelf... twelve... zzz...",
                    "...mmh? Oh. Shelf twelve. Sorted. Good. *the golem settles back into a contented, gravelly doze* ...thank... you...",
                ],
            },
            Npc {
                name: "Sage Alderly",
                pos: (208, 35),
                quest: Some(22),
                idle: &[
                    "Every book comes home eventually. Some just take the scenic route.",
                    "The spellbooks are all sorted at last — and there's a certain rune-smith I owe my thanks to as well.",
                ],
            },
            Npc {
                name: "Scribe Faye",
                pos: (213, 35),
                quest: Some(23),
                idle: &[
                    "A library's only as good as its catalogue. Mine has... gaps. Had gaps.",
                    "Every page accounted for, every record true. You've done more for this library in an afternoon than most manage in a year.",
                ],
            },
        ],
        critters: vec![
            Critter::new(CritterKind::Moth, (100, 48)),
            Critter::new(CritterKind::Moth, (150, 24)),
        ],
        signs: vec![
            Sign {
                pos: (8, 34),
                text: "The Hearthspire road. Mist ahead — it's friendlier than it looks.",
            },
            Sign {
                pos: (190, 32),
                text: "The Great Library. Quiet please — the mist is listening too.",
            },
        ],
    }
}
// ── interiors ───────────────────────────────────────────────────────────────

/// A room floating in the dark: stamp the art at `ROOM_AT`, spawn just inside
/// the door. Everything else about the zone is quiet — no sky, no gate.
#[allow(clippy::too_many_arguments)]
fn room(
    id: usize,
    name: &'static str,
    seed: u32,
    art: &str,
    daylight: f32,
    warps: Vec<Warp>,
    npcs: Vec<Npc>,
    critters: Vec<Critter>,
    signs: Vec<Sign>,
) -> Zone {
    let mut b = MapBuilder::new(seed);
    b.rect(0, 0, MAP_W, MAP_H, Tile::Void);
    b.stamp(ROOM_AT.0, ROOM_AT.1, art);
    let door = warps.first().expect("every room has a way out").at;
    Zone {
        id,
        name,
        tiles: b.tiles,
        spawn: (door.0, door.1 - 1),
        gate: None,
        locked_msg: "",
        unlock_msg: "",
        weather: None,
        daylight,
        interior: true,
        border: Border::Void,
        seed,
        warps,
        npcs,
        critters,
        signs,
    }
}

/// Furniture offsets are relative to `ROOM_AT`, mirroring each room's art.
fn at(dx: i32, dy: i32) -> (i32, i32) {
    (ROOM_AT.0 + dx, ROOM_AT.1 + dy)
}

fn bakery() -> Zone {
    room(
        BAKERY,
        "Poppy's Bakery",
        55,
        BAKERY_ROOM,
        0.85,
        vec![exit(BAKERY_ROOM_DOOR, EMBERWICK, BAKERY_DOOR)],
        vec![],
        vec![Critter::new(CritterKind::Cat, at(4, 1))],
        vec![Sign {
            pos: at(4, 3),
            text: "A recipe card, dusted with flour: 'Honey-oat loaves. Flour, oats, honey, patience. If it looks wrong, add butter. If it looks right: also butter.'",
        }],
    )
}

fn sorrel_cottage() -> Zone {
    room(
        SORREL_COTTAGE,
        "Granny Sorrel's Cottage",
        66,
        COTTAGE_ROOM,
        0.85,
        vec![exit(COTTAGE_ROOM_DOOR, EMBERWICK, SORREL_DOOR)],
        vec![Npc {
            name: "Granny Sorrel",
            pos: at(8, 3),
            quest: None,
            idle: &["Sit down, sit down. The kettle's just... well, it's thinking about boiling."],
        }],
        vec![Critter::new(CritterKind::Cat, at(4, 4))],
        vec![],
    )
}

fn carpenter_house() -> Zone {
    room(
        CARPENTER_HOUSE,
        "Alder's Workshop",
        77,
        WORKSHOP_ROOM,
        0.8,
        vec![exit(WORKSHOP_ROOM_DOOR, EMBERWICK, CARPENTER_DOOR)],
        vec![Npc {
            name: "Carpenter Alder",
            pos: at(6, 4),
            quest: None,
            idle: &["Measure twice, saw once, sweep thrice. Nobody warns you about the sweeping."],
        }],
        vec![],
        vec![Sign {
            pos: at(3, 4),
            text: "A pencilled note on the workbench, underlined three times: 'Gate hinge 34 and a HALF. Not 43 and a half. We do not speak of the tall gate.'",
        }],
    )
}

fn tilly_cottage() -> Zone {
    room(
        TILLY_COTTAGE,
        "Tilly's Cottage",
        88,
        COTTAGE_ROOM,
        0.85,
        vec![exit(COTTAGE_ROOM_DOOR, EMBERWICK, TILLY_DOOR)],
        vec![Npc {
            name: "Hen-keeper Tilly",
            pos: at(7, 4),
            quest: None,
            idle: &["The chickens have opinions about the rain. Strong ones. Don't ask Henrietta."],
        }],
        vec![Critter::new(CritterKind::Chicken, at(3, 3))],
        vec![],
    )
}

fn storehouse() -> Zone {
    room(
        STOREHOUSE,
        "The Old Storehouse",
        99,
        STORE_ROOM,
        0.7,
        vec![
            exit(STORE_ROOM_DOOR, EMBERWICK, STOREHOUSE_DOOR),
            enter(STORE_CELLAR_DOOR, STOREHOUSE_CELLAR, CELLAR_ROOM_DOOR),
        ],
        vec![],
        vec![Critter::new(CritterKind::Cat, at(8, 3))],
        vec![Sign {
            pos: at(4, 4),
            text: "An inventory, in a fading hand: preserves - CELLAR. Cellar key - LOST (went walking in the deep woods; the key did not come back, and neither did my afternoon).",
        }],
    )
}

fn storehouse_cellar() -> Zone {
    room(
        STOREHOUSE_CELLAR,
        "The Storehouse Cellar",
        133,
        CELLAR_ROOM,
        0.4,
        vec![exit(CELLAR_ROOM_DOOR, STOREHOUSE, STORE_CELLAR_DOOR)],
        vec![],
        vec![],
        vec![],
    )
}

fn echo_cave() -> Zone {
    room(
        ECHO_CAVE,
        "The Echo Cave",
        111,
        CAVE_ROOM,
        0.35,
        vec![exit(CAVE_ROOM_EXIT, WHISPERING_WOODS, CAVE_MOUTH)],
        vec![],
        vec![Critter::new(CritterKind::Frog, at(4, 5))],
        vec![Sign {
            pos: at(10, 6),
            text: "Someone has carved: HELLO. Below it, smaller: hello. Below that, tiny: hello.",
        }],
    )
}

fn woods_ruin() -> Zone {
    room(
        WOODS_RUIN,
        "A Tumbledown Cottage",
        133,
        RUIN_ROOM,
        0.5,
        vec![exit(RUIN_ROOM_DOOR, WHISPERING_WOODS, RUIN_STONE_DOOR)],
        vec![],
        vec![Critter::new(CritterKind::Moth, at(6, 2))],
        vec![Sign {
            pos: at(4, 2),
            text: "A note on the table, the ink gone the color of weak tea: 'The roof lets in more sky than it keeps out, and the moss has opinions about the pantry. Gone down to Emberwick. Whoever finds this: the kettle's yours. Mind the third floorboard.'",
        }],
    )
}

fn woods_lodge() -> Zone {
    room(
        WOODS_LODGE,
        "The Old Forester's Lodge",
        144,
        LODGE_ROOM,
        0.45,
        vec![exit(LODGE_ROOM_DOOR, WHISPERING_WOODS, RUIN_LODGE_DOOR)],
        vec![],
        vec![Critter::new(CritterKind::Moth, at(8, 3))],
        vec![Sign {
            pos: at(5, 2),
            text: "A note nailed to the table, in a hand that pressed hard: 'Gone deeper in. The trees talk less nonsense than the town does, and the quiet keeps better company. Don't come looking — but if you do, follow the gaps, not the road. — N.'",
        }],
    )
}

/// The Great Library: three connected chambers built room-by-room rather than
/// stamped from one art block — a stacks reading-room to the west, a grand
/// lamplit entrance hall in the middle, and a showcase gallery to the east,
/// all under a top wall of tall sunlit windows.
fn great_library() -> Zone {
    let seed = 122;
    let mut b = MapBuilder::new(seed);
    b.rect(0, 0, MAP_W, MAP_H, Tile::Void);
    let (ox, oy) = ROOM_AT;
    let (w, h) = (LIBRARY_W, LIBRARY_H);

    // Floor throughout, then a wall all around.
    b.rect(ox, oy, w, h, Tile::Floor);
    for x in ox..ox + w {
        b.set(x, oy, Tile::Wall);
        b.set(x, oy + h - 1, Tile::Wall);
    }
    for y in oy..oy + h {
        b.set(ox, y, Tile::Wall);
        b.set(ox + w - 1, y, Tile::Wall);
    }

    // Two internal walls divide the hall into three chambers, each pierced by
    // a two-tile archway low down so you can wander between them.
    for y in oy + 1..oy + h - 1 {
        b.set(ox + 13, y, Tile::Wall);
        b.set(ox + 26, y, Tile::Wall);
    }
    for y in [oy + h - 4, oy + h - 3] {
        b.set(ox + 13, y, Tile::Floor);
        b.set(ox + 26, y, Tile::Floor);
    }

    // Tall windows along the top wall — the sun (and the moon) pour through.
    for dx in [3, 4, 9, 10, 18, 19, 21, 22, 30, 31, 36, 37] {
        b.set(ox + dx, oy, Tile::Window);
    }

    // Entrance doors in the bottom wall (the central hall), back to Hearthspire.
    b.set(LIBRARY_ROOM_DOORS[0].0, LIBRARY_ROOM_DOORS[0].1, Tile::Door);
    b.set(LIBRARY_ROOM_DOORS[1].0, LIBRARY_ROOM_DOORS[1].1, Tile::Door);

    // ── West chamber: the reading stacks. Shelf rows with aisles between. ──
    for row in [3, 7, 11] {
        for dx in 3..=8 {
            b.set(ox + dx, oy + row, Tile::Bookshelf);
        }
    }
    // A reading nook: a table and stools, and a warm rug underfoot.
    b.rect(ox + 3, oy + 15, 4, 3, Tile::Rug);
    b.set(ox + 4, oy + 16, Tile::Table);
    b.set(ox + 3, oy + 16, Tile::Stool);
    b.set(ox + 5, oy + 16, Tile::Stool);
    b.set(ox + 9, oy + 15, Tile::Bookshelf);
    b.set(ox + 10, oy + 15, Tile::Bookshelf);
    // The music corner: an upright piano against the west wall.
    b.set(ox + 1, oy + 15, Tile::Piano);

    // ── Central hall: lamps, a long rug, the librarian, a cat. ──
    b.rect(ox + 17, oy + 15, 6, 5, Tile::Rug);
    for (dx, dy) in [(15, 2), (24, 2), (15, 18), (24, 18)] {
        b.set(ox + dx, oy + dy, Tile::Lantern);
    }
    // A grand central exhibit for arriving visitors, and the hall's tall
    // case clock ticking between the window bays.
    b.set(ox + 19, oy + 4, Tile::Pedestal);
    b.set(ox + 20, oy + 4, Tile::Pedestal);
    b.set(ox + 17, oy + 1, Tile::Clock);

    // ── East chamber: the showcase gallery — plants, art, curios. ──
    for (dx, dy) in [(29, 3), (36, 3), (29, 9), (34, 12), (30, 16)] {
        b.set(ox + dx, oy + dy, Tile::Plant);
    }
    for (dx, dy) in [(32, 3), (32, 9), (36, 16), (29, 19)] {
        b.set(ox + dx, oy + dy, Tile::Pedestal);
    }
    // Framed art on the gallery's outer and dividing walls.
    for dy in [3, 7, 11, 15] {
        b.set(ox + w - 1, oy + dy, Tile::Painting);
    }
    for dy in [4, 9, 14] {
        b.set(ox + 26, oy + dy, Tile::Painting);
    }
    // The library's catalogued runestone, set on the gallery floor.
    b.set(ox + 34, oy + 19, Tile::Runestone);

    Zone {
        id: GREAT_LIBRARY,
        name: "The Great Library",
        tiles: b.tiles,
        spawn: (LIBRARY_ROOM_DOORS[0].0, LIBRARY_ROOM_DOORS[0].1 - 1),
        gate: None,
        locked_msg: "",
        unlock_msg: "",
        weather: None,
        daylight: 0.75,
        interior: true,
        border: Border::Void,
        seed,
        warps: vec![
            exit(LIBRARY_ROOM_DOORS[0], HEARTHSPIRE, LIBRARY_DOORS[0]),
            exit(LIBRARY_ROOM_DOORS[1], HEARTHSPIRE, LIBRARY_DOORS[1]),
        ],
        npcs: vec![Npc {
            name: "Under-librarian Twill",
            pos: at(19, 20),
            quest: None,
            idle: &[
                "Shhh — not for quiet. The books are napping. Nudge any shelf (e) and one will happily read itself to you. The east gallery's worth a wander, too.",
            ],
        }],
        critters: vec![
            Critter::new(CritterKind::Cat, at(21, 6)),
            Critter::new(CritterKind::Cat, at(30, 12)),
        ],
        signs: vec![Sign {
            pos: at(33, 16),
            text: "THE SHOWCASE GALLERY. Curiosities gathered from the four roads: pressed ferns, a river-smoothed stone, a study in three greens. Please admire with your eyes.",
        }],
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, VecDeque};

    use super::*;
    use crate::world::map::{MAP_H as H, MAP_W as W};

    /// Flood-fill of everywhere the player can stand, from the zone's spawn.
    /// NPCs block their own tile, just like in the game.
    fn reachable(zone: &Zone) -> HashSet<(i32, i32)> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([zone.spawn]);
        seen.insert(zone.spawn);
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (x + dx, y + dy);
                if n.0 < 0 || n.1 < 0 || n.0 >= W || n.1 >= H || seen.contains(&n) {
                    continue;
                }
                if zone.tile(n.0, n.1).walkable() && zone.npc_at(n.0, n.1).is_none() {
                    seen.insert(n);
                    queue.push_back(n);
                }
            }
        }
        seen
    }

    fn adjacent_reachable(seen: &HashSet<(i32, i32)>, pos: (i32, i32)) -> bool {
        (-1..=1).any(|dy| {
            (-1..=1).any(|dx| (dx, dy) != (0, 0) && seen.contains(&(pos.0 + dx, pos.1 + dy)))
        })
    }

    #[test]
    fn every_npc_sign_and_gate_can_be_walked_to() {
        for zone in zones() {
            let seen = reachable(&zone);
            for npc in &zone.npcs {
                assert!(
                    adjacent_reachable(&seen, npc.pos),
                    "{} is unreachable at {:?} in {}",
                    npc.name,
                    npc.pos,
                    zone.name
                );
            }
            for sign in &zone.signs {
                assert!(
                    adjacent_reachable(&seen, sign.pos),
                    "sign at {:?} unreachable in {}",
                    sign.pos,
                    zone.name
                );
            }
            if let Some(gate) = zone.gate {
                assert!(
                    adjacent_reachable(&seen, gate),
                    "gate at {:?} unreachable in {} — the journey would dead-end!",
                    gate,
                    zone.name
                );
            }
        }
    }

    #[test]
    fn doorway_warps_keep_their_door_tiles() {
        // A lane's grass fringe once shaved the facade row a door lived in
        // off the bakery and Sorrel's cottage (the "sunken house" playtest
        // note). Pin that every doorway warp still sits on door art; the
        // Echo Cave's mouth and its inward exit are the deliberate
        // non-doors.
        for (i, zone) in zones().iter().enumerate() {
            for warp in &zone.warps {
                if warp.to_zone == ECHO_CAVE || i == ECHO_CAVE {
                    continue;
                }
                let tile = zone.tile(warp.at.0, warp.at.1);
                assert!(
                    matches!(tile, Tile::Door | Tile::FacadeDoor(_)),
                    "warp at {:?} in {} sits on {tile:?}, not a door",
                    warp.at,
                    zone.name
                );
            }
        }
    }

    #[test]
    fn hearth_rooms_are_all_interiors() {
        let zones = zones();
        for (i, zone) in zones.iter().enumerate() {
            if has_hearth(i) {
                assert!(zone.interior, "{} has a hearth but open sky", zone.name);
            }
        }
        assert!(has_hearth(BAKERY), "Poppy's ovens never go cold");
        assert!(!has_hearth(ECHO_CAVE), "no fire lit in the Echo Cave");
        assert!(!has_hearth(EMBERWICK), "the overworld has zone music instead");
    }

    #[test]
    fn gates_cannot_be_walked_around() {
        // Everything reachable from spawn must be strictly west of the gate
        // column (except the gate tiles themselves, which transition).
        for zone in zones().into_iter().take(2) {
            let gate_x = zone.gate.unwrap().0;
            let seen = reachable(&zone);
            assert!(
                seen.iter().all(|&(x, _)| x < gate_x),
                "the gate in {} can be skirted",
                zone.name
            );
        }
    }

    #[test]
    fn every_door_tile_leads_somewhere() {
        // A door you can't walk through is a broken promise.
        for zone in zones() {
            for y in 0..H {
                for x in 0..W {
                    if matches!(zone.tile(x, y), Tile::Door | Tile::FacadeDoor(_)) {
                        assert!(
                            zone.warp_at(x, y).is_some(),
                            "the door at {:?} in {} opens onto nothing",
                            (x, y),
                            zone.name
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_warp_lands_on_reachable_ground_and_has_a_way_back() {
        let all = zones();
        let seen: Vec<_> = all.iter().map(reachable).collect();
        for (i, zone) in all.iter().enumerate() {
            for warp in &zone.warps {
                let dest = &all[warp.to_zone];
                assert!(
                    seen[warp.to_zone].contains(&warp.to_pos),
                    "warp from {} lands at {:?} in {}, which can't be walked from its spawn",
                    zone.name,
                    warp.to_pos,
                    dest.name
                );
                assert!(
                    dest.warps.iter().any(|w| w.to_zone == i),
                    "{} has a way in from {} but no way back",
                    dest.name,
                    zone.name
                );
            }
        }
    }

    #[test]
    fn every_runestone_stands_where_the_catalogue_says() {
        let all = zones();
        // The spots table and the actual tiles must agree, both ways.
        for (i, (zone, (x, y))) in runestone_spots().iter().enumerate() {
            assert_eq!(
                all[*zone].tile(*x, *y),
                Tile::Runestone,
                "stone {} missing at {:?} in {}",
                i + 1,
                (x, y),
                all[*zone].name
            );
            assert_eq!(runestone_id(*zone, (*x, *y)), Some(i as u8 + 1));
        }
        let standing: usize = all
            .iter()
            .map(|z| {
                (0..H)
                    .flat_map(|y| (0..W).map(move |x| (x, y)))
                    .filter(|&(x, y)| z.tile(x, y) == Tile::Runestone)
                    .count()
            })
            .sum();
        assert_eq!(
            standing,
            runestone_spots().len(),
            "an uncatalogued runestone is standing somewhere"
        );
        // Seven standing stones + one in the cellar chest = the full set.
        assert_eq!(
            standing + 1,
            crate::content::stones::RUNESTONES.len(),
            "the stones content and the world disagree on the count"
        );
    }

    #[test]
    fn every_secret_can_be_walked_to() {
        let all = zones();
        let seen: Vec<_> = all.iter().map(reachable).collect();
        for (zone, pos) in runestone_spots() {
            assert!(
                adjacent_reachable(&seen[zone], pos),
                "runestone at {:?} unreachable in {}",
                pos,
                all[zone].name
            );
        }
        // The moon-mint patch, and the cellar chest.
        for (zone, tile) in [
            (WHISPERING_WOODS, Tile::Herb),
            (STOREHOUSE_CELLAR, Tile::Chest),
        ] {
            let z = &all[zone];
            let spot = (0..H)
                .flat_map(|y| (0..W).map(move |x| (x, y)))
                .find(|&(x, y)| z.tile(x, y) == tile)
                .unwrap_or_else(|| panic!("no {tile:?} anywhere in {}", z.name));
            assert!(
                adjacent_reachable(&seen[zone], spot),
                "{tile:?} at {spot:?} unreachable in {}",
                z.name
            );
        }
    }

    #[test]
    fn the_library_has_a_book_for_every_shelf() {
        // No shelf should have to reuse a book — the collection must be at
        // least as large as the number of shelves that read from it.
        let lib = &zones()[GREAT_LIBRARY];
        let shelves = (0..H)
            .flat_map(|y| (0..W).map(move |x| (x, y)))
            .filter(|&(x, y)| lib.tile(x, y) == Tile::Bookshelf)
            .count();
        assert!(shelves > 0, "the Library has no shelves at all");
        assert!(
            shelves <= crate::content::books::BOOKS.len(),
            "the Library has {shelves} shelves but only {} books — some would duplicate",
            crate::content::books::BOOKS.len()
        );
    }

    #[test]
    fn every_zone_keeps_its_own_hour() {
        for zone in zones() {
            assert!(
                (0.0..=1.0).contains(&zone.daylight),
                "{} has daylight {} outside 0..=1",
                zone.name,
                zone.daylight
            );
            // Interiors never carry weather; the open air may be clear
            // (Emberwick's skies) or hold a drifting effect.
            assert!(
                !(zone.interior && zone.weather.is_some()),
                "{}: interiors never carry weather",
                zone.name
            );
        }
    }
}
