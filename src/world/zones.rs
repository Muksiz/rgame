use crate::world::entity::{Critter, CritterKind, Npc, Sign};
use crate::world::map::{Border, MAP_H, MAP_W, MapBuilder, Tile, Warp, Weather, Zone};

const HOUSE: &str = concat!("rrrrrrrrr\n", "#.......#\n", "#.......#\n", "####+####",);

const HOUSE_SMALL: &str = concat!("rrrrrrr\n", "#.....#\n", "###+###",);

const WELL: &str = concat!(" %%% \n", "%%~%%\n", " %%% ",);

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

const DOCK: &str = concat!("____      \n", "==========\n", "____      ",);

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

const STORE_ROOM: &str = concat!(
    "#############\n",
    "#.oo.....oo.#\n",
    "#.oox....xo.#\n",
    "#...........#\n",
    "#..xx.......#\n",
    "#........t..#\n",
    "#...........#\n",
    "#...........#\n",
    "######+######",
);

const LIBRARY_ROOM: &str = concat!(
    "##########################\n",
    "#BBBB.BBBBB....BBBBB.BBBB#\n",
    "#........................#\n",
    "#..stt.....RRRR.....tts..#\n",
    "#..........RRRR..........#\n",
    "#..stt.....RRRR.....tts..#\n",
    "#........................#\n",
    "#BBBB.BBBBB....BBBBB.BBBB#\n",
    "#........................#\n",
    "#..L..................L..#\n",
    "#........................#\n",
    "###########++#############",
);

const CAVE_ROOM: &str = concat!(
    "  %%%%%%%%%%%  \n",
    " %%:::::::::%% \n",
    " %:::::::::::% \n",
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

// Interior door tiles (stamp origin + the '+' offset in each room's art).
const BAKERY_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 7, ROOM_AT.1 + 8);
const COTTAGE_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 5, ROOM_AT.1 + 7);
const WORKSHOP_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1 + 8);
const STORE_ROOM_DOOR: (i32, i32) = (ROOM_AT.0 + 6, ROOM_AT.1 + 8);
const LIBRARY_ROOM_DOORS: [(i32, i32); 2] = [
    (ROOM_AT.0 + 11, ROOM_AT.1 + 11),
    (ROOM_AT.0 + 12, ROOM_AT.1 + 11),
];
const CAVE_ROOM_EXIT: (i32, i32) = (ROOM_AT.0 + 7, ROOM_AT.1 + 9);

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
    ]
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

    // Houses along the road, each with a lane down to it.
    b.stamp(62, 18, HOUSE); // the bakery
    b.stamp(90, 14, HOUSE_SMALL);
    b.stamp(126, 20, HOUSE);
    b.stamp(74, 46, HOUSE_SMALL);
    b.stamp(132, 46, HOUSE);
    b.road(&[(66, 23), (66, 29)]);
    b.road(&[(130, 26), (130, 39)]);
    b.road(&[(77, 45), (77, 42)]);

    // Festival square: paving, the unlit lantern, a cosy campfire.
    b.rect(80, 32, 13, 6, Tile::Path);
    b.set(86, 34, Tile::Lantern);
    b.set(90, 36, Tile::Campfire);
    for x in [79, 93] {
        for y in [32, 34, 36] {
            b.set(x, y, Tile::Flower);
        }
    }

    // The old well, up a short lane.
    b.stamp(108, 22, WELL);
    b.road(&[(110, 26), (110, 30)]);

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

    b.clearing(86, 31, 1);
    b.clearing(66, 25, 1);
    b.clearing(112, 27, 1);

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

    Zone {
        id: 0,
        name: "Emberwick Village",
        tiles: b.tiles,
        spawn: (6, 38),
        gate: Some((231, 36)),
        locked_msg: "A fallen oak blocks the road. Maybe help the villagers first — starting with Elder Rowan at the festival square.",
        unlock_msg: "The villagers roll the old oak aside, cheering. The road east lies open!",
        weather: Some(Weather::Petals),
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
                glyph: 'R',
                color: (216, 186, 130),
                pos: (86, 31),
                quest: Some(1),
                idle: &["The lantern has hung dark for years. Tonight, maybe, it glows again."],
            },
            Npc {
                name: "Baker Poppy",
                glyph: 'P',
                color: (234, 156, 146),
                pos: (66, 25),
                quest: Some(2),
                idle: &["Smell that? Honey-oat loaves. The recipe is older than the village."],
            },
            Npc {
                name: "Well-keeper Bram",
                glyph: 'B',
                color: (150, 184, 214),
                pos: (112, 27),
                quest: Some(3),
                idle: &["Deepest well in the valley, this. Probably. Nobody's ever checked."],
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
    b.scatter_all(Tile::Tree, 380);
    b.scatter_all(Tile::Bush, 80);
    b.scatter_all(Tile::TallGrass, 90);
    b.scatter_all(Tile::Flower, 15);
    b.edge_band(Tile::Tree, 4);

    // Ambrose's meadow (before the road, which passes through it).
    b.rect(162, 22, 26, 13, Tile::Grass);
    b.scatter(Tile::TallGrass, 220, (162, 22, 26, 13));
    b.scatter(Tile::Flower, 80, (162, 22, 26, 13));
    for x in 162..188 {
        b.set(x, 22, Tile::Fence);
    }

    b.road(&[
        (0, 36),
        (30, 36),
        (45, 22),
        (90, 22),
        (105, 44),
        (150, 44),
        (165, 30),
        (210, 30),
        (239, 32),
    ]);

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
    b.road(&[
        (CAVE_MOUTH.0, 44),
        (112, 46),
        (112, 57),
        (CAVE_MOUTH.0, 57),
        (CAVE_MOUTH.0, CAVE_MOUTH.1 + 1),
    ]);
    b.stamp(118, 52, CAVE);

    // Mossy old gate across the east road.
    barrier(&mut b, 233, 30..=34, Tile::Tree);

    b.set(8, 34, Tile::Sign);
    b.set(120, 40, Tile::Sign);

    b.clearing(49, 20, 1);
    b.clearing(101, 46, 1);
    b.clearing(172, 28, 1);

    Zone {
        id: 1,
        name: "Whispering Woods",
        tiles: b.tiles,
        spawn: (5, 36),
        gate: Some((233, 32)),
        locked_msg: "An old mossy gate, swollen shut. The woods aren't done with you yet, it seems.",
        unlock_msg: "The mossy gate creaks open, almost politely. Onward, to the river!",
        weather: Some(Weather::Fireflies),
        daylight: 0.45,
        interior: false,
        border: Border::Forest,
        seed,
        warps: vec![enter(CAVE_MOUTH, ECHO_CAVE, CAVE_ROOM_EXIT)],
        npcs: vec![
            Npc {
                name: "Wren",
                glyph: 'F',
                color: (152, 214, 122),
                pos: (49, 20),
                quest: Some(4),
                idle: &["When I grow up I'm going to walk to BOTH ends of the road."],
            },
            Npc {
                name: "Forager Maren",
                glyph: 'M',
                color: (206, 130, 170),
                pos: (101, 46),
                quest: Some(5),
                idle: &["Rule one of foraging: when in doubt, don't. Rule two: see rule one."],
            },
            Npc {
                name: "Shepherd Ambrose",
                glyph: 'A',
                color: (196, 186, 156),
                pos: (172, 28),
                quest: Some(6),
                idle: &["*yaaawn* ...I wasn't sleeping. I was counting very slowly."],
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
                pos: (120, 40),
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
    b.road(&[(130, 36), (130, 46), (134, 47)]);
    b.road(&[(138, 32), (138, 22), (146, 22)]);
    b.road(&[(114, 42), (114, 54)]);

    // The Silverford itself (drawn after the roads: the river wins).
    b.river(155, 6.0, 4);

    // The old bridge — its west end is also the gate east.
    for x in 146..=168 {
        for y in 33..=35 {
            b.set(x, y, Tile::Bridge);
        }
    }
    for y in 33..=35 {
        b.set(146, y, Tile::Gate);
    }

    // Ferry dock, south along the west bank.
    b.stamp(136, 46, DOCK);

    // Morrow's little beach.
    b.rect(112, 55, 14, 6, Tile::Sand);
    b.scatter(Tile::Reed, 180, (108, 53, 22, 10));

    b.set(8, 38, Tile::Sign);
    b.set(140, 38, Tile::Sign);

    b.clearing(136, 45, 1);
    b.clearing(148, 22, 1);
    b.clearing(118, 57, 1);

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
                name: "Ferryman Wick",
                glyph: 'W',
                color: (126, 168, 190),
                pos: (136, 45),
                quest: Some(7),
                idle: &["River's high today. River's always high today, if you ask the river."],
            },
            Npc {
                name: "Fisher Juniper",
                glyph: 'J',
                color: (142, 196, 196),
                pos: (148, 22),
                quest: Some(8),
                idle: &["The trick to fishing is patience. The other trick is bait. Mostly bait."],
            },
            Npc {
                name: "Hermit Morrow",
                glyph: 'O',
                color: (176, 156, 208),
                pos: (118, 57),
                quest: Some(9),
                idle: &["The river brings me letters. I write back, but slowly."],
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
    // Craggy highland bands north and south.
    b.rect(0, 0, 240, 8, Tile::Cliff);
    b.rect(0, 62, 240, 8, Tile::Cliff);
    b.scatter(Tile::Cliff, 250, (0, 8, 240, 6));
    b.scatter(Tile::Cliff, 250, (0, 56, 240, 6));
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
        b.set(x, 34, Tile::Path); // forecourt up to the door
    }

    b.set(8, 34, Tile::Sign);
    b.set(190, 32, Tile::Sign);

    b.clearing(68, 48, 1);
    b.clearing(172, 24, 1);
    b.clearing(206, 35, 1);

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
                glyph: 'E',
                color: (188, 176, 146),
                pos: (68, 48),
                quest: Some(10),
                idle: &[
                    "I catalogue everything. Even this conversation. Especially this conversation.",
                ],
            },
            Npc {
                name: "The Stone Golem",
                glyph: 'G',
                color: (168, 168, 180),
                pos: (172, 24),
                quest: Some(11),
                idle: &["...zzz... shelf... twelve... zzz..."],
            },
            Npc {
                name: "Sage Alderly",
                glyph: 'S',
                color: (226, 204, 152),
                pos: (208, 35),
                quest: Some(12),
                idle: &["Every book comes home eventually. Some just take the scenic route."],
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
        vec![],
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
            glyph: 'N',
            color: (208, 178, 188),
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
            glyph: 'C',
            color: (188, 168, 138),
            pos: at(6, 4),
            quest: None,
            idle: &["Measure twice, saw once, sweep thrice. Nobody warns you about the sweeping."],
        }],
        vec![],
        vec![],
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
            glyph: 'T',
            color: (170, 190, 160),
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
        vec![exit(STORE_ROOM_DOOR, EMBERWICK, STOREHOUSE_DOOR)],
        vec![],
        vec![Critter::new(CritterKind::Cat, at(8, 3))],
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

fn great_library() -> Zone {
    room(
        GREAT_LIBRARY,
        "The Great Library",
        122,
        LIBRARY_ROOM,
        0.75,
        vec![
            exit(LIBRARY_ROOM_DOORS[0], HEARTHSPIRE, LIBRARY_DOORS[0]),
            exit(LIBRARY_ROOM_DOORS[1], HEARTHSPIRE, LIBRARY_DOORS[1]),
        ],
        vec![Npc {
            name: "Under-librarian Twill",
            glyph: 'U',
            color: (188, 168, 138),
            pos: at(13, 8),
            quest: None,
            idle: &[
                "Shhh — not for quiet. The books are napping. Nudge any shelf (e) and one will happily read itself to you.",
            ],
        }],
        vec![Critter::new(CritterKind::Cat, at(20, 4))],
        vec![],
    )
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
                    if zone.tile(x, y) == Tile::Door {
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
    fn every_zone_keeps_its_own_hour() {
        for zone in zones() {
            assert!(
                (0.0..=1.0).contains(&zone.daylight),
                "{} has daylight {} outside 0..=1",
                zone.name,
                zone.daylight
            );
            assert_eq!(
                zone.interior,
                zone.weather.is_none(),
                "{}: only interiors go without weather",
                zone.name
            );
        }
    }
}
