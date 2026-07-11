use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc::Receiver;

use crate::checker::{self, Outcome};
use crate::content::items::{self, Item};
use crate::content::quests::{self, QUESTS, Quest};
use crate::content::{books, critters, ferris, lore, schedule, sides, stones, wilds};
use crate::gfx::atlas::PLAYABLE;
use crate::save::{self, SaveData};
use crate::world::entity::CritterKind;
use crate::world::map::hash2;
use crate::world::map::{MAP_H, MAP_W, Tile, Zone};
use crate::world::zones;

/// How long a toast lingers, in ticks (~50ms each).
/// The game clock's tick length — the shell calls `on_tick` at this cadence,
/// and the renderer's step-glide converts seconds to ticks with it.
pub const TICK_SECS: f32 = 0.05;
/// How long one walking step takes while a movement key is held. The shell
/// repeats held keys at this pace and the renderer glides the player across
/// exactly this window, so feet and pixels agree. (Eased from 0.12 after a
/// playtest called the pace a tiny bit too fast.)
pub const STEP_SECS: f32 = 0.135;
/// Walking both axes at once covers √2 ground per step, so diagonal steps come
/// a touch slower to keep the traveller honest. The shell stretches the held
/// repeat by this and the renderer stretches the glide to match, so a diagonal
/// hold glides corner-to-corner with no freeze between steps.
pub const DIAGONAL_STRETCH: f32 = 1.4;

const TOAST_TICKS: u64 = 110;
/// How long a zone-arrival banner slides across the screen.
pub const BANNER_TICKS: u64 = 55;
/// Typewriter reveal speed by option: slow, normal, fast (characters per tick).
const REVEAL_SPEEDS: [usize; 3] = [1, 2, 4];
/// One tall-grass step in this many rustles up a wild rune (on average). Kept
/// deliberately uncommon so the grass stays a place to wander, not a gauntlet
/// (doubled from 18 after a playtest found runes stirring way too often).
const ENCOUNTER_RARITY: u32 = 36;

// ── the day/night clock ─────────────────────────────────────────────────────
// Time no longer flows on its own (a playtest chose a world that waits): the
// clock parks at an anchor and only a campfire rest moves it, toggling
// between a bright day and a starry night. The phase lengths below keep
// defining the sky's brightness arc (`sky_daylight`) and where the anchors
// sit within it. Outdoor places follow this shared sky; interiors keep their
// own steady lamplight (see `App::daylight`).
const TICKS_PER_MIN: u32 = 20 * 60;
const MORNING_LEN: u32 = 10 * TICKS_PER_MIN; // 10 real minutes
const DAY_LEN_MIN: u32 = 20 * TICKS_PER_MIN; // 20
const EVENING_LEN: u32 = 10 * TICKS_PER_MIN; // 10
const NIGHT_LEN: u32 = 15 * TICKS_PER_MIN; // 15
/// Ticks in one whole day (55 real minutes).
pub const DAY_LEN: u32 = MORNING_LEN + DAY_LEN_MIN + EVENING_LEN + NIGHT_LEN;
const DAY_START: u32 = MORNING_LEN; // when the sun is fully up
const EVENING_START: u32 = MORNING_LEN + DAY_LEN_MIN;
const NIGHT_START: u32 = MORNING_LEN + DAY_LEN_MIN + EVENING_LEN;
/// Where a campfire rest parks the clock: early midday, sun high.
const DAY_ANCHOR: u32 = MORNING_LEN + DAY_LEN_MIN / 4;
/// And the deep of night, stars out, folk asleep.
const NIGHT_ANCHOR: u32 = NIGHT_START + NIGHT_LEN / 2;
/// A fresh journey opens on a bright late morning.
const JOURNEY_START: u32 = MORNING_LEN * 9 / 10;

/// The four times of day. Which one it is drives the sky, the HUD clock, and
/// whether the folk of the world are up and about or fast asleep.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DayPhase {
    Morning,
    Day,
    Evening,
    Night,
}

impl DayPhase {
    pub fn label(self) -> &'static str {
        match self {
            DayPhase::Morning => "Morning",
            DayPhase::Day => "Midday",
            DayPhase::Evening => "Evening",
            DayPhase::Night => "Night",
        }
    }

    fn at(t: u32) -> DayPhase {
        match t % DAY_LEN {
            t if t < DAY_START => DayPhase::Morning,
            t if t < EVENING_START => DayPhase::Day,
            t if t < NIGHT_START => DayPhase::Evening,
            _ => DayPhase::Night,
        }
    }
}

/// The open-sky brightness at clock position `t`: a smooth arc from a soft
/// dawn, up to full midday, down through a golden evening, into a deep — but
/// never pitch — night, and back. Piecewise-linear between a handful of
/// anchors so the transitions read as the hours sliding by.
pub fn sky_daylight(t: u32) -> f32 {
    const ANCHORS: [(u32, f32); 7] = [
        (0, 0.52),
        (MORNING_LEN, 0.95),
        (MORNING_LEN + DAY_LEN_MIN / 2, 1.0),
        (EVENING_START, 0.9),
        (NIGHT_START, 0.34),
        (NIGHT_START + NIGHT_LEN / 2, 0.16),
        (DAY_LEN, 0.52),
    ];
    let t = t % DAY_LEN;
    for w in ANCHORS.windows(2) {
        let (t0, l0) = w[0];
        let (t1, l1) = w[1];
        if t >= t0 && t < t1 {
            let f = (t - t0) as f32 / (t1 - t0) as f32;
            return l0 + (l1 - l0) * f;
        }
    }
    ANCHORS[0].1
}

/// The game's own input alphabet. The windowing shell translates whatever
/// the platform reports into these; tests feed them to `App::on_key` directly.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    PageUp,
    PageDown,
    Backspace,
    Char(char),
}

pub static EPILOGUE: &[&str] = &[
    "The tall doors of the Great Library swing wide, and warm lamplight spills down the steps and into the mist. Somewhere above, the shelves go up and up until they look like a night full of square stars.",
    "You think of the whole road at once: a lantern blooming gold over Emberwick, sheep folding into meadow grass, a token handed back across a rain-specked dock, a letter mended mid-sentence. Twenty-three small runes. One quiet journey.",
    "Ferris scuttles up your sleeve and settles on your shoulder, watching the lamplight — the same little crab who has walked every step of this road beside you. \"You know,\" he says, \"most spellbooks end where the good part starts. Enums, lifetimes, traits... whole wings of this place we haven't touched.\"",
    "\"But that,\" Ferris yawns, \"is a journey for another evening. For now: armchairs.\"\n\n~ Thank you for playing RUNE & ROAD ~\n\nThe world stays open — wander back down the road whenever you like.",
];

pub enum DialogueKind {
    /// Ends by accepting the quest (scaffolding its file).
    Intro(u8),
    /// A nudge about the currently accepted quest.
    Reminder,
    /// Post-pass celebration; may unlock the gate or roll the credits.
    Success(u8),
    /// Idle chatter, signposts.
    Flavor,
    /// A book taken down from a Library shelf.
    Book,
    /// Side-quest talk; closing it may set a world flag.
    Side(Option<&'static str>),
    /// A runestone read aloud (gets the stone portrait).
    Stone,
}

pub struct Dialogue {
    pub speaker: String,
    pub pages: Vec<String>,
    pub page: usize,
    /// Characters of the current page revealed by the typewriter so far.
    pub revealed: usize,
    pub kind: DialogueKind,
}

impl Dialogue {
    fn new(speaker: &str, pages: Vec<String>, kind: DialogueKind) -> Self {
        // Re-flow the authored pages against the dialogue box's actual
        // capacity (the larger reading face fits less per page than the old
        // small one): every page is wrapped to the box's columns and split
        // into as many full pages as it needs, so nothing authored is ever
        // cut off — long book pages simply turn more page dots.
        use crate::gfx::font;
        use crate::gfx::scene::{DIALOGUE_COLS, DIALOGUE_ROWS};
        let mut pages: Vec<String> = pages
            .iter()
            .flat_map(|page| {
                font::wrap(page, DIALOGUE_COLS)
                    .chunks(DIALOGUE_ROWS)
                    .map(|lines| lines.join("\n"))
                    .collect::<Vec<_>>()
            })
            .collect();
        if pages.is_empty() {
            pages.push(String::new());
        }
        Self {
            speaker: speaker.to_string(),
            pages,
            page: 0,
            revealed: 0,
            kind,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EncounterPhase {
    /// The rune has posed its question and awaits an answer.
    Asking,
    /// Answered true — inscribed in the grimoire.
    Caught,
    /// Answered wrong — the rune skitters off, no harm done.
    Fizzled,
}

pub enum Screen {
    Title {
        selected: usize,
    },
    /// Choosing who you'll be before setting out: a look from the roster and a
    /// name of your own.
    CharSelect {
        idx: usize,
        name: String,
    },
    World,
    Dialogue(Dialogue),
    Journal,
    /// A wild rune met in the tall grass.
    Encounter {
        rune: u8,
        selected: usize,
        phase: EncounterPhase,
    },
    /// The collection of wild runes inscribed so far.
    Grimoire,
    /// The parchment map of the journey (`m`): the four zones downsampled
    /// honestly from their real tiles, uncharted until first entered.
    WorldMap,
    /// Resting at a campfire: the screen fades to ember-dark, a scrap of Rust
    /// lore drifts past, and waking flips the world's clock.
    Resting {
        /// Index into `content::lore::LORE`.
        lore: usize,
        /// Ticks since lying down — drives the fade-in and the ember glow.
        t: u32,
        /// The phase you'll wake into (Night after a daytime rest; a bright
        /// Day after sleeping through the night).
        wake: DayPhase,
    },
    Casting {
        quest: u8,
    },
    CastResult {
        quest: u8,
        outcome: Outcome,
        scroll: u16,
    },
    Paused {
        selected: usize,
    },
    Epilogue {
        page: usize,
    },
}

/// What the ground underfoot is made of — the shell picks footstep foley
/// by this; the lib only names it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    /// Grass, flowers, forest floor — the soft world.
    Soft,
    /// Packed-earth paths and roads.
    Earth,
    /// Beach and riverbank sand.
    Sand,
    /// Floorboards, bridges, piers, thresholds.
    Wood,
    /// Cobbles, cave rock, cellar stone.
    Stone,
}

/// Semantic sound events: the game pushes them as things happen, the shell
/// drains them (`App::drain_sounds`) into actual audio once a frame. The
/// lib and the tests stay silent and window-free — tests can assert that
/// walking on wood *sounds* different from grass without hearing a thing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SoundEvent {
    /// A step landed on this kind of ground.
    Stepped(Terrain),
    /// A doorway (or the cave mouth) was walked through.
    DoorUsed,
    /// The cellar chest finally gave in.
    ChestOpened,
    /// A keepsake changed hands at a quest's end.
    KeepsakeGiven,
    /// A wild rune joined the grimoire.
    RuneCaught,
    /// A runestone's rune was rubbed into the journal.
    StoneFound,
    /// A menu selection moved (title, rest menu, encounter answers…).
    MenuMoved,
    /// A dialogue page turned.
    PageTurned,
}

/// The footstep flavor of a tile. Paths harden to stone in the lightless
/// places (the cave and the cellar floor are hewn rock, not garden earth).
fn terrain_of(tile: Tile, dark_zone: bool) -> Terrain {
    match tile {
        Tile::Path if dark_zone => Terrain::Stone,
        Tile::Path => Terrain::Earth,
        Tile::Sand => Terrain::Sand,
        Tile::Floor | Tile::Bridge | Tile::Pier | Tile::Rug | Tile::Door | Tile::FacadeDoor(_) => {
            Terrain::Wood
        }
        Tile::Plaza | Tile::CaveMouth => Terrain::Stone,
        _ => Terrain::Soft,
    }
}

/// Gate-reveal cutscene timing, in 50ms ticks: the camera pans out to the
/// gate, the barrier takes its time rolling aside, the open road holds for
/// a beat, and the camera pans home. The renderer reads these phases.
pub const REVEAL_PAN: u64 = 14;
pub const REVEAL_CLEAR: u64 = 26;
pub const REVEAL_HOLD: u64 = 10;
pub const REVEAL_TICKS: u64 = REVEAL_PAN + REVEAL_CLEAR + REVEAL_HOLD + REVEAL_PAN;

pub struct App {
    pub screen: Screen,
    pub tick: u64,
    pub play_ticks: u64,
    /// Position within the day/night cycle, 0..DAY_LEN. Advances with play,
    /// and a campfire's rest can leap it to the next phase.
    pub day_ticks: u32,
    pub zones: Vec<Zone>,
    pub zone_idx: usize,
    pub player: (i32, i32),
    /// Who the player chose to be: an index into `atlas::PLAYABLE`, and the
    /// name they gave themselves.
    pub player_char: usize,
    pub player_name: String,
    /// Which way the player faces (a unit step vector); purely cosmetic,
    /// so it is never saved.
    pub facing: (i32, i32),
    /// Tick of the last successful step — while fresh, the walk cycle plays.
    pub walked_at: u64,
    /// The square the last step departed from — the renderer glides the
    /// player (and camera) from here to `player` over the step, so walking
    /// reads as motion instead of tile-snaps. Cosmetic, never saved.
    pub prev_player: (i32, i32),
    /// How far into the current 50ms tick the shell is (0..1), so the glide
    /// above stays butter-smooth at any frame rate. Headless renders leave
    /// it at zero and lose nothing.
    pub subtick: f32,
    /// The sub-tick fraction captured at the moment of the last step. Held
    /// steps fire off a wall clock that isn't aligned to the tick grid, so
    /// `walked_at` alone rounds the departure to the nearest tick and the
    /// glide would start a pixel or two ahead — snapping every step. Pairing
    /// it with this fraction lets the glide start exactly at the departure
    /// square. Cosmetic, never saved; headless leaves it zero.
    pub walk_subtick: f32,
    /// The companion's tile — the square the player last vacated, so Ferris
    /// walks exactly one step behind. He has been at your heels since before
    /// the road began; his spot is ephemeral like `walked_at`: never saved,
    /// he simply reappears at your side on load.
    pub companion: (i32, i32),
    /// The square the companion stepped from, for its own render glide.
    /// Cosmetic, never saved.
    pub companion_prev: (i32, i32),
    pub completed: BTreeSet<u8>,
    pub accepted: BTreeSet<u8>,
    pub hints: BTreeMap<u8, usize>,
    /// Wild runes inscribed from tall-grass encounters.
    pub grimoire: BTreeSet<u8>,
    /// Fish met (and released) with Juniper's spare rod.
    pub fish: u32,
    /// Steps taken through tall grass; part of the deterministic encounter roll.
    pub grass_steps: u32,
    /// World-state flags: side quests, runestones, opened chests. Anything
    /// that isn't quest completion but must be remembered.
    pub flags: BTreeSet<String>,
    pub toast: Option<(String, u64)>,
    /// A zone-name banner that slides in when you arrive somewhere new.
    pub banner: Option<(String, u64)>,
    /// The little cutscene when a zone's gate first opens: (zone index,
    /// start tick). The camera glides out to the gate, the barrier rolls
    /// aside, and the view glides home; any key skips it. Cosmetic and
    /// ephemeral — never saved, and an old save simply finds its cleared
    /// gates already standing open.
    pub gate_reveal: Option<(usize, u64)>,
    /// Every NPC's authored daytime post, captured from the fresh world:
    /// (name, zone, tile). `apply_schedule` re-derives live positions from
    /// this plus the hour and the active quest — derived, never stored.
    schedule_home: Vec<(&'static str, usize, (i32, i32))>,
    /// Sound events queued since the shell last drained them. The lib never
    /// plays audio; it only says what happened (see `SoundEvent`).
    sounds: Vec<SoundEvent>,
    /// The sound option from the rest menu: 0 off, 1 quiet, 2 full. The
    /// shell scales every loop and one-shot by it.
    pub sound_level: usize,
    /// Typewriter reveal speed, chosen in the options: 0 slow, 1 normal, 2 fast.
    pub text_speed: usize,
    pub cast_rx: Option<Receiver<Outcome>>,
    pub has_save: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let zones = zones::zones();
        let player = zones[0].spawn;
        let schedule_home = zones
            .iter()
            .enumerate()
            .flat_map(|(zi, z)| z.npcs.iter().map(move |n| (n.name, zi, n.pos)))
            .collect();
        Self {
            screen: Screen::Title { selected: 0 },
            tick: 0,
            play_ticks: 0,
            day_ticks: 0,
            zones,
            zone_idx: 0,
            player,
            player_char: 0,
            player_name: String::new(),
            facing: (0, 1),
            walked_at: 0,
            prev_player: player,
            subtick: 0.0,
            walk_subtick: 0.0,
            companion: player,
            companion_prev: player,
            completed: BTreeSet::new(),
            accepted: BTreeSet::new(),
            hints: BTreeMap::new(),
            grimoire: BTreeSet::new(),
            fish: 0,
            grass_steps: 0,
            flags: BTreeSet::new(),
            toast: None,
            banner: None,
            gate_reveal: None,
            schedule_home,
            sounds: Vec::new(),
            sound_level: 2,
            text_speed: 1,
            cast_rx: None,
            has_save: save::exists(),
            should_quit: false,
        }
    }

    pub fn zone(&self) -> &Zone {
        &self.zones[self.zone_idx]
    }

    /// How bright it is right now. Outdoors the whole world shares one sky,
    /// swinging through the day/night clock; each open zone keeps a little of
    /// its own character (the woods stay shadier than the village), a gentle
    /// canopy factor riding on top of the shared hour. Interiors ignore the
    /// clock entirely and keep their own steady lamplight.
    pub fn daylight(&self) -> f32 {
        let zone = self.zone();
        if zone.interior {
            zone.daylight
        } else {
            let canopy = 0.82 + 0.18 * zone.daylight;
            (sky_daylight(self.day_ticks) * canopy).clamp(0.0, 1.0)
        }
    }

    /// Which quarter of the day it is right now.
    pub fn phase(&self) -> DayPhase {
        DayPhase::at(self.day_ticks)
    }

    /// True when the outdoor world has gone to sleep — folk are abed, the sky
    /// is dark. (Interiors keep their own hour, so this is an outdoor notion.)
    pub fn is_night(&self) -> bool {
        !self.zone().interior && self.phase() == DayPhase::Night
    }

    /// Keepsakes are earned, never lost: owning one is derived from the
    /// quests completed, so old saves get their items for free.
    pub fn has_item(&self, item: Item) -> bool {
        self.completed
            .iter()
            .any(|&id| items::reward(id) == Some(item))
    }

    /// The name to call the player by — with a gentle fallback for the
    /// nameless (old saves, or anyone who skipped the naming).
    pub fn player_name(&self) -> &str {
        if self.player_name.is_empty() {
            "the Wanderer"
        } else {
            &self.player_name
        }
    }

    /// The player's chosen look, clamped so a stray save index can't panic.
    pub fn player_look(&self) -> &'static crate::gfx::atlas::Playable {
        &PLAYABLE[self.player_char.min(PLAYABLE.len() - 1)]
    }

    /// Gather the companion in to the player's square — through doors, gates
    /// and zone edges it scurries along rather than being left behind, and on
    /// load it simply reappears at your side.
    fn companion_snap(&mut self) {
        self.companion = self.player;
        self.companion_prev = self.player;
    }

    /// World flags: the memory for everything off the main quest road.
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    pub fn set_flag(&mut self, flag: &str) {
        self.flags.insert(flag.to_string());
    }

    /// Note the current overworld zone as charted on the parchment map.
    /// Rides the flags (and so the existing autosave milestones) — first
    /// entry to a zone is always a gate crossing, which already saves.
    fn mark_visited(&mut self) {
        if !self.zone().interior && self.zone_idx <= 3 {
            self.set_flag(&sides::visited_flag(self.zone_idx));
        }
    }

    /// Where the player reads on the parchment map: their own tile under
    /// the open sky, or — from a room behind a door — the door they came
    /// in by, resolved back out to the overworld.
    pub fn map_spot(&self) -> (usize, (i32, i32)) {
        let (mut zone, mut pos) = (self.zone_idx, self.player);
        for _ in 0..self.zones.len() {
            if zone <= 3 {
                break;
            }
            let Some((pz, at)) = self.zones.iter().enumerate().find_map(|(zi, z)| {
                z.warps
                    .iter()
                    .find(|w| w.to_zone == zone)
                    .map(|w| (zi, w.at))
            }) else {
                break;
            };
            zone = pz;
            pos = at;
        }
        (zone, pos)
    }

    /// The next quest on the road: the first one not yet completed.
    pub fn active_quest(&self) -> Option<&'static Quest> {
        QUESTS.iter().find(|q| !self.completed.contains(&q.id))
    }

    pub fn zone_cleared(&self, zone: usize) -> bool {
        QUESTS
            .iter()
            .filter(|q| q.zone == zone)
            .all(|q| self.completed.contains(&q.id))
    }

    /// Put every named NPC where the hour says they should be: their
    /// authored post by day, their `content::schedule` spot after dark —
    /// except the active quest's giver, who ignores the hour and keeps
    /// watch at their post ("she's been watching the road for you").
    /// Positions are re-derived whole from phase + active quest, so this
    /// runs at the moments either can change (a campfire rest, a quest
    /// passing, a save loading) and nothing about it is ever written to
    /// disk — an old save wakes up already sorted.
    pub fn apply_schedule(&mut self) {
        let night = self.phase() == DayPhase::Night;
        let active = self.active_quest().map(|q| q.npc);
        for i in 0..self.schedule_home.len() {
            let (name, home_zone, home_pos) = self.schedule_home[i];
            let target = match schedule::night_spot(name) {
                Some(spot) if night && active != Some(name) => spot,
                _ => (home_zone, home_pos),
            };
            self.place_npc(name, target);
        }
    }

    /// Move a named NPC to `(zone, tile)`, across zones if need be — but
    /// never onto the player's own square (they simply stay put till the
    /// next turn of the clock rather than share boots).
    fn place_npc(&mut self, name: &str, (tz, tp): (usize, (i32, i32))) {
        if tz == self.zone_idx && tp == self.player {
            return;
        }
        let Some(cz) = self
            .zones
            .iter()
            .position(|z| z.npcs.iter().any(|n| n.name == name))
        else {
            return;
        };
        if cz == tz {
            if let Some(npc) = self.zones[cz].npcs.iter_mut().find(|n| n.name == name) {
                npc.pos = tp;
            }
        } else {
            let i = self.zones[cz]
                .npcs
                .iter()
                .position(|n| n.name == name)
                .expect("just found above");
            let mut npc = self.zones[cz].npcs.remove(i);
            npc.pos = tp;
            self.zones[tz].npcs.push(npc);
        }
    }

    pub fn toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), self.tick + TOAST_TICKS));
    }

    /// Announce the place you've just arrived in with a sliding banner.
    fn show_banner(&mut self) {
        self.banner = Some((self.zone().name.to_string(), self.tick + BANNER_TICKS));
    }

    /// Characters revealed per tick, per the chosen text speed.
    fn reveal_step(&self) -> usize {
        REVEAL_SPEEDS[self.text_speed.min(REVEAL_SPEEDS.len() - 1)]
    }

    /// Cycle the typewriter speed (the options toggle).
    fn cycle_text_speed(&mut self) {
        self.text_speed = (self.text_speed + 1) % REVEAL_SPEEDS.len();
    }

    fn cycle_sound_level(&mut self) {
        self.sound_level = (self.sound_level + 1) % 3;
    }

    /// Queue a sound for the shell; the lib itself stays silent.
    fn sound(&mut self, e: SoundEvent) {
        self.sounds.push(e);
    }

    /// Hand over (and clear) the queued sound events — the shell calls this
    /// once a frame and turns them into audio.
    pub fn drain_sounds(&mut self) -> Vec<SoundEvent> {
        std::mem::take(&mut self.sounds)
    }

    // ── ticking ────────────────────────────────────────────────────────────

    pub fn on_tick(&mut self) {
        self.tick += 1;
        if !matches!(self.screen, Screen::Title { .. }) {
            self.play_ticks += 1;
            // `day_ticks` deliberately holds still: day and night wait for
            // the player, and only a campfire rest turns the clock.
        }
        if let Some((_, until)) = &self.toast
            && self.tick > *until
        {
            self.toast = None;
        }
        if let Some((_, until)) = &self.banner
            && self.tick > *until
        {
            self.banner = None;
        }
        if let Some((_, started)) = self.gate_reveal
            && self.tick.saturating_sub(started) >= REVEAL_TICKS
        {
            self.gate_reveal = None;
        }
        let step = self.reveal_step();
        if let Screen::Dialogue(d) = &mut self.screen {
            d.revealed = d.revealed.saturating_add(step);
        }
        if let Screen::Resting { t, .. } = &mut self.screen {
            *t = t.saturating_add(1);
        }
        if matches!(self.screen, Screen::World) && self.tick.is_multiple_of(12) {
            self.wander_critters();
        }
        if let Screen::Casting { quest } = self.screen
            && let Some(rx) = &self.cast_rx
            && let Ok(outcome) = rx.try_recv()
        {
            self.cast_rx = None;
            if matches!(outcome, Outcome::Pass { .. }) {
                self.completed.insert(quest);
                self.autosave();
                // The torch passes: if it's night, the next errand's giver
                // comes out to their post to watch the road for you.
                self.apply_schedule();
            }
            self.screen = Screen::CastResult {
                quest,
                outcome,
                scroll: 0,
            };
        }
    }

    fn wander_critters(&mut self) {
        let npc_positions: Vec<(i32, i32)> = self.zones[self.zone_idx]
            .npcs
            .iter()
            .map(|n| n.pos)
            .collect();
        let player = self.player;
        let critter_positions: Vec<(i32, i32)> = self.zones[self.zone_idx]
            .critters
            .iter()
            .map(|c| c.pos)
            .collect();
        let zone_idx = self.zone_idx;
        let tick = self.tick as u32;

        for i in 0..self.zones[zone_idx].critters.len() {
            let (pos, home) = {
                let c = &self.zones[zone_idx].critters[i];
                (c.pos, c.home)
            };
            let h = hash2(pos.0, pos.1, tick.wrapping_add(i as u32 * 977));
            if h % 10 >= 4 {
                continue; // mostly they just stand there, being pleasant
            }
            let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)][(h / 16) as usize % 4];
            let target = (pos.0 + dir.0, pos.1 + dir.1);
            let roams_free = (target.0 - home.0).abs() <= 6 && (target.1 - home.1).abs() <= 6;
            let tile_ok = self.zones[zone_idx].tile(target.0, target.1).walkable();
            let unoccupied = target != player
                && !npc_positions.contains(&target)
                && !critter_positions.contains(&target);
            if roams_free && tile_ok && unoccupied && in_bounds(target) {
                self.zones[zone_idx].critters[i].pos = target;
            }
        }
    }

    // ── input ──────────────────────────────────────────────────────────────

    pub fn on_key(&mut self, key: Key) {
        match &mut self.screen {
            Screen::Title { .. } => self.title_key(key),
            Screen::CharSelect { .. } => self.char_select_key(key),
            Screen::World => self.world_key(key),
            Screen::Dialogue(_) => self.dialogue_key(key),
            Screen::Journal => self.journal_key(key),
            Screen::Encounter { .. } => self.encounter_key(key),
            Screen::Grimoire => self.grimoire_key(key),
            Screen::WorldMap => self.world_map_key(key),
            Screen::Resting { .. } => self.resting_key(key),
            Screen::Casting { .. } => {} // the runes are busy
            Screen::CastResult { .. } => self.cast_result_key(key),
            Screen::Paused { .. } => self.paused_key(key),
            Screen::Epilogue { .. } => self.epilogue_key(key),
        }
    }

    fn title_key(&mut self, code: Key) {
        let items = if self.has_save { 3 } else { 2 };
        let Screen::Title { selected } = &mut self.screen else {
            return;
        };
        match code {
            Key::Up | Key::Char('k') => {
                *selected = (*selected + items - 1) % items;
                self.sounds.push(SoundEvent::MenuMoved);
            }
            Key::Down | Key::Char('j') => {
                *selected = (*selected + 1) % items;
                self.sounds.push(SoundEvent::MenuMoved);
            }
            Key::Enter | Key::Char(' ') | Key::Char('e') => {
                // With a save: [Continue, New Journey, Quit]; without: [New Journey, Quit].
                let choice = *selected;
                match (self.has_save, choice) {
                    (true, 0) => self.continue_game(),
                    (true, 1) | (false, 0) => self.new_game(),
                    _ => self.should_quit = true,
                }
            }
            Key::Esc | Key::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    /// "A new journey" opens the character chooser rather than dropping you
    /// straight into the world — first decide who you are.
    fn new_game(&mut self) {
        self.screen = Screen::CharSelect {
            idx: 0,
            name: String::new(),
        };
    }

    fn char_select_key(&mut self, code: Key) {
        let Screen::CharSelect { idx, name } = &mut self.screen else {
            return;
        };
        let n = PLAYABLE.len();
        match code {
            // Left/right leaf through the roster (arrows only — the letter keys
            // are busy spelling your name).
            Key::Left | Key::Up => {
                *idx = (*idx + n - 1) % n;
                self.sounds.push(SoundEvent::MenuMoved);
            }
            Key::Right | Key::Down => {
                *idx = (*idx + 1) % n;
                self.sounds.push(SoundEvent::MenuMoved);
            }
            Key::Backspace => {
                name.pop();
            }
            // Enter commits; here `e`/space are letters you might be typing, so
            // only Enter sets you off. A blank name won't do — the road needs
            // something to call you.
            Key::Enter => {
                let (idx, name) = (*idx, name.trim().to_string());
                if name.is_empty() {
                    self.toast(
                        "A nameless hero? The ballads would be awfully short. Type a name first.",
                    );
                } else {
                    self.begin_journey(idx, name);
                }
            }
            Key::Esc => self.screen = Screen::Title { selected: 0 },
            Key::Char(c)
                if (c.is_ascii_alphabetic() || c == '-' || c == '\'')
                    && name.chars().count() < 14 =>
            {
                // Capitalize the first letter, lowercase the rest — names come
                // out looking like names whatever the caps-lock is doing.
                let c = if name.is_empty() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                };
                name.push(c);
            }
            _ => {}
        }
    }

    /// Lock in the chosen look and name, wipe the slate, and step into a fresh
    /// Emberwick morning.
    fn begin_journey(&mut self, idx: usize, name: String) {
        self.player_char = idx.min(PLAYABLE.len() - 1);
        self.player_name = name.trim().to_string();
        self.zone_idx = 0;
        self.player = self.zones[0].spawn;
        self.completed.clear();
        self.accepted.clear();
        self.hints.clear();
        self.grimoire.clear();
        self.fish = 0;
        self.grass_steps = 0;
        self.flags.clear();
        self.play_ticks = 0;
        self.day_ticks = JOURNEY_START; // every journey opens on a bright morning
        self.companion_snap();
        self.mark_visited(); // Emberwick charts itself from the first step
        self.screen = Screen::World;
        self.toast(format!(
            "A quiet morning in Emberwick, {}. Someone near the festival square could use a hand. (Arrows or H J K L to walk, e to talk.)",
            self.player_name
        ));
    }

    fn continue_game(&mut self) {
        if let Some(data) = save::load() {
            self.apply_save(data);
            self.screen = Screen::World;
            self.toast("The road remembers you. Welcome back.");
        } else {
            self.toast("The save scroll was blank... starting fresh.");
            self.new_game();
        }
    }

    fn apply_save(&mut self, data: SaveData) {
        self.player_char = data.player_char.min(PLAYABLE.len() - 1);
        self.player_name = data.player_name;
        self.completed = data.completed.into_iter().collect();
        self.accepted = data.accepted.into_iter().collect();
        self.hints = data.hints;
        self.grimoire = data.grimoire.into_iter().collect();
        self.fish = data.fish;
        self.flags = data.flags.into_iter().collect();
        self.zone_idx = data.zone.min(self.zones.len() - 1);
        self.play_ticks = data.play_ticks;
        self.day_ticks = data.day_ticks % DAY_LEN;
        self.text_speed = data.text_speed.min(REVEAL_SPEEDS.len() - 1);
        self.sound_level = data.sound_level.min(2);
        let (x, y) = data.pos;
        self.player = if in_bounds((x, y)) && self.zone().tile(x, y).walkable() {
            (x, y)
        } else {
            self.zone().spawn
        };
        // The companion's spot is never saved: it reappears at your side.
        self.companion_snap();
        // Neither are the folk's whereabouts: re-derive them from the
        // loaded hour and quest — a night save wakes with everyone home.
        self.apply_schedule();
        // A save from before the map existed backfills its charted zones:
        // the road is linear, so everywhere up to here has been walked.
        let (here, _) = self.map_spot();
        for z in 0..=here {
            self.set_flag(&sides::visited_flag(z));
        }
    }

    fn autosave(&mut self) {
        let data = SaveData {
            player_char: self.player_char,
            player_name: self.player_name.clone(),
            completed: self.completed.iter().copied().collect(),
            accepted: self.accepted.iter().copied().collect(),
            hints: self.hints.clone(),
            grimoire: self.grimoire.iter().copied().collect(),
            fish: self.fish,
            flags: self.flags.iter().cloned().collect(),
            zone: self.zone_idx,
            pos: self.player,
            play_ticks: self.play_ticks,
            day_ticks: self.day_ticks,
            text_speed: self.text_speed,
            sound_level: self.sound_level,
        };
        match save::save(&data) {
            Ok(()) => self.has_save = true,
            Err(e) => self.toast(format!("Couldn't write the save scroll: {e}")),
        }
    }

    fn world_key(&mut self, code: Key) {
        // While the gate-reveal cutscene has the camera, the first key
        // simply hands it back — nobody is made to watch.
        if self.gate_reveal.is_some() {
            self.gate_reveal = None;
            return;
        }
        match code {
            Key::Up | Key::Char('k') => self.try_move(0, -1),
            Key::Down | Key::Char('j') => self.try_move(0, 1),
            Key::Left | Key::Char('h') => self.try_move(-1, 0),
            Key::Right | Key::Char('l') => self.try_move(1, 0),
            Key::Enter | Key::Char('e') | Key::Char(' ') => self.interact(),
            Key::Char('c') => self.start_cast(),
            Key::Char('q') => self.screen = Screen::Journal,
            Key::Char('g') => self.screen = Screen::Grimoire,
            Key::Char('m') => self.screen = Screen::WorldMap,
            Key::Char('f') => self.ferris_hint(),
            Key::Esc => self.screen = Screen::Paused { selected: 0 },
            _ => {}
        }
    }

    fn encounter_key(&mut self, code: Key) {
        let Screen::Encounter {
            rune,
            selected,
            phase,
        } = &mut self.screen
        else {
            return;
        };
        let rune_id = *rune;
        match *phase {
            EncounterPhase::Asking => match code {
                Key::Up | Key::Char('k') => {
                    *selected = (*selected + 2) % 3;
                    self.sounds.push(SoundEvent::MenuMoved);
                }
                Key::Down | Key::Char('j') => {
                    *selected = (*selected + 1) % 3;
                    self.sounds.push(SoundEvent::MenuMoved);
                }
                Key::Enter | Key::Char(' ') | Key::Char('e') => {
                    if *selected == wilds::wild(rune_id).answer {
                        *phase = EncounterPhase::Caught;
                        self.grimoire.insert(rune_id);
                        self.sounds.push(SoundEvent::RuneCaught);
                    } else {
                        *phase = EncounterPhase::Fizzled;
                    }
                }
                Key::Esc => {
                    // Fleeing is always free.
                    self.screen = Screen::World;
                    self.toast("You back away slowly. The grass settles. No harm done.");
                }
                _ => {}
            },
            _ => match code {
                Key::Enter | Key::Char(' ') | Key::Char('e') | Key::Esc => {
                    self.screen = Screen::World;
                }
                _ => {}
            },
        }
    }

    fn grimoire_key(&mut self, code: Key) {
        match code {
            Key::Esc
            | Key::Char('g')
            | Key::Char('q')
            | Key::Char('e')
            | Key::Enter
            | Key::Char(' ') => {
                self.screen = Screen::World;
            }
            _ => {}
        }
    }

    fn try_move(&mut self, dx: i32, dy: i32) {
        // Face the way you push, even into a wall — then step if you can.
        self.facing = (dx, dy);
        let target = (self.player.0 + dx, self.player.1 + dy);

        // Stepping off the west edge walks back toward the previous zone
        // (interiors have no edges worth walking to — only their door).
        if target.0 < 0 && !self.zone().interior {
            if self.zone_idx > 0 {
                self.zone_idx -= 1;
                let gate = self.zone().gate.unwrap_or(self.zone().spawn);
                self.player = (gate.0 - 2, gate.1);
                self.companion_snap();
                self.mark_visited();
                let name = self.zone().name;
                self.toast(format!("Back along the road, into {name}."));
                self.show_banner();
                self.autosave();
            } else {
                self.toast("Home lies that way — but the Library first! You promised.");
            }
            return;
        }
        if !in_bounds(target) {
            return;
        }

        // Doorways: step through, and the world changes around you.
        if let Some(warp) = self.zone().warp_at(target.0, target.1) {
            // Dark places (the Echo Cave, the storehouse cellar): only a
            // steady light gets you in.
            if zones::needs_light(warp.to_zone) {
                if !self.has_item(Item::StormLantern) {
                    self.toast(
                        "The dark inside is absolute. Something with a steady light would help.",
                    );
                    return;
                }
                self.toast("You raise Bram's storm-lantern and step into the dark.");
            }
            self.zone_idx = warp.to_zone;
            self.player = warp.to_pos;
            self.companion_snap();
            self.sound(SoundEvent::DoorUsed);
            self.show_banner();
            if self.zone().interior && !zones::needs_light(self.zone_idx) {
                let name = self.zone().name;
                self.toast(format!("You step inside — {name}."));
            }
            return;
        }

        let tile = self.zone().tile(target.0, target.1);
        if tile == Tile::Gate {
            self.try_gate();
            return;
        }
        let occupied = self.zone().npc_at(target.0, target.1).is_some()
            || self.zone().critters.iter().any(|c| c.pos == target);
        if tile.walkable() && !occupied {
            // A second step landing on the same tick (the shell walking a
            // held diagonal) keeps the original departure square, so the
            // glide runs corner-to-corner instead of kinking mid-step.
            // The companion takes that same departure square — always exactly
            // one step behind, never on a tile the player couldn't stand, and
            // never the mid-diagonal square, which would put it two tiles from
            // `companion_prev` and past what its render glide can cover.
            if self.walked_at != self.tick {
                self.prev_player = self.player;
                self.companion_prev = self.companion;
                self.companion = self.player;
            }
            self.player = target;
            self.walked_at = self.tick;
            self.walk_subtick = self.subtick;
            self.sound(SoundEvent::Stepped(terrain_of(
                tile,
                zones::needs_light(self.zone_idx),
            )));
            if tile == Tile::TallGrass && !self.zone().interior {
                self.rustle_grass();
            }
        }
    }

    /// Each step through tall grass rolls (deterministically — same walk,
    /// same runes) for a wild rune encounter from this zone's grass. Rustles
    /// are uncommon — a small event, not something underfoot every few paces —
    /// and a rune you've already inscribed has moved on: the grass only stirs
    /// with the questions you haven't answered true yet.
    fn rustle_grass(&mut self) {
        self.grass_steps = self.grass_steps.wrapping_add(1);
        let h = hash2(self.player.0, self.player.1, 0xB1AD ^ self.grass_steps);
        if !h.is_multiple_of(ENCOUNTER_RARITY) {
            return;
        }
        let pool: Vec<_> = wilds::in_zone(self.zone().id)
            .into_iter()
            .filter(|r| !self.grimoire.contains(&r.id))
            .collect();
        if pool.is_empty() {
            return; // every wild rune in this grass is already in the grimoire
        }
        let rune = pool[(h / ENCOUNTER_RARITY) as usize % pool.len()];
        self.screen = Screen::Encounter {
            rune: rune.id,
            selected: 0,
            phase: EncounterPhase::Asking,
        };
    }

    fn try_gate(&mut self) {
        if self.zone_cleared(self.zone_idx) {
            let unlock = self.zone().unlock_msg;
            if self.zone_idx + 1 < self.zones.len() {
                self.zone_idx += 1;
                self.player = self.zone().spawn;
                self.companion_snap();
                self.mark_visited();
                self.toast(unlock);
                self.show_banner();
                self.autosave();
            }
        } else {
            let msg = self.zone().locked_msg;
            self.toast(msg);
        }
    }

    fn interact(&mut self) {
        let (px, py) = self.player;
        // Anything on an adjacent tile (or underfoot, for signs).
        let mut spots: Vec<(i32, i32)> = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                spots.push((px + dx, py + dy));
            }
        }
        let npc_spot = spots
            .iter()
            .find(|(x, y)| self.zone().npc_at(*x, *y).is_some())
            .copied();
        if let Some((x, y)) = npc_spot {
            let dialogue = self.npc_dialogue(x, y);
            self.screen = Screen::Dialogue(dialogue);
            return;
        }
        // Anything with writing on it: signposts outside, notes left on
        // tables and crates indoors.
        let note = spots.iter().find_map(|&(x, y)| {
            self.zone()
                .sign_at(x, y)
                .map(|s| (self.zone().tile(x, y), s.text))
        });
        if let Some((tile, text)) = note {
            let speaker = if tile == Tile::Sign {
                "Signpost"
            } else {
                "A note"
            };
            self.screen = Screen::Dialogue(Dialogue::new(
                speaker,
                vec![text.to_string()],
                DialogueKind::Flavor,
            ));
            return;
        }
        // A bookshelf within reach: take a book down and read. The shelf you
        // face (straight ahead of you in the aisle) wins over its neighbors,
        // so each step along a stack turns exactly one page of the catalogue.
        let shelf = [
            (0, -1),
            (-1, 0),
            (1, 0),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ]
        .iter()
        .map(|(dx, dy)| (px + dx, py + dy))
        .find(|&(x, y)| self.zone().tile(x, y) == Tile::Bookshelf);
        if let Some((x, y)) = shelf {
            self.read_shelf(x, y);
            return;
        }
        // The quiet secrets: herbs to pick, stones to find, chests to try.
        let secret = spots.iter().find_map(|&(x, y)| {
            let tile = self.zone().tile(x, y);
            matches!(tile, Tile::Herb | Tile::Chest | Tile::Runestone).then_some((tile, x, y))
        });
        match secret {
            Some((Tile::Herb, ..)) => {
                self.pick_herb();
                return;
            }
            Some((Tile::Chest, ..)) => {
                self.try_chest();
                return;
            }
            Some((Tile::Runestone, x, y)) => {
                self.touch_runestone(x, y);
                return;
            }
            _ => {}
        }
        // A campfire within reach: sit a while. The screen fades to embers, a
        // little Rust lore drifts past, and waking rolls the clock on.
        let fire = spots
            .iter()
            .any(|&(x, y)| self.zone().tile(x, y) == Tile::Campfire);
        if fire {
            self.rest_at_campfire();
            return;
        }
        // Water within reach and a rod in the satchel: that's fishing.
        let water = spots
            .iter()
            .any(|&(x, y)| matches!(self.zone().tile(x, y), Tile::Water | Tile::Reed));
        if water && self.has_item(Item::FishingRod) {
            self.go_fishing();
            return;
        }
        // Any small life within reach has something to say. Usually its noise.
        let critter = spots.iter().copied().find_map(|(x, y)| {
            self.zone()
                .critters
                .iter()
                .find(|c| c.pos == (x, y))
                .map(|c| (c.kind, x, y))
        });
        if let Some((kind, x, y)) = critter {
            let h = hash2(x, y, 0xC1CC ^ self.day_ticks);
            let (speaker, line) = match kind {
                CritterKind::Chicken => ("A chicken", critters::chicken(h)),
                CritterKind::Sheep => ("A sheep", critters::sheep(h)),
                CritterKind::Frog => ("A frog", critters::frog(h)),
                CritterKind::Moth => ("A moth", critters::moth(h)),
                CritterKind::Cat => ("The cat", critters::cat(h)),
                CritterKind::Dog => ("The village dog", critters::dog(h)),
                CritterKind::Boar => ("A wild boar", critters::boar(h)),
                CritterKind::Duck => ("A duck", critters::duck(h)),
                CritterKind::Donkey => ("The pack donkey", critters::donkey(h)),
            };
            self.screen = Screen::Dialogue(Dialogue::new(
                speaker,
                vec![line.to_string()],
                DialogueKind::Flavor,
            ));
            return;
        }
        // Nothing else in reach — then a word with your oldest friend, who is
        // never more than a step away. On an empty stretch of road, `e` is
        // Ferris's cue: he always has an opinion ready.
        self.chat_with_ferris();
    }

    /// Ferris pipes up: one remark, picked deterministically from where you
    /// stand and the hour, so the same corner at the same time of day always
    /// draws the same thought — with the local region's remarks (or the
    /// indoor set, under a roof) folded into the rotation.
    fn chat_with_ferris(&mut self) {
        let h = hash2(self.player.0, self.player.1, 0xFE44 ^ self.day_ticks);
        let line = ferris::chat(h, self.is_night(), self.zone_idx);
        self.screen = Screen::Dialogue(Dialogue::new(
            "Ferris",
            vec![line.to_string()],
            DialogueKind::Flavor,
        ));
    }

    /// Each shelf tile holds one book, assigned by walking order along the
    /// stacks — browse a row left to right and you read the collection in
    /// sequence (wrapping around once the titles run out).
    fn read_shelf(&mut self, sx: i32, sy: i32) {
        let zone = self.zone();
        let mut ordinal = 0usize;
        'scan: for y in 0..MAP_H {
            for x in 0..MAP_W {
                if zone.tile(x, y) == Tile::Bookshelf {
                    if (x, y) == (sx, sy) {
                        break 'scan;
                    }
                    ordinal += 1;
                }
            }
        }
        let book = &books::BOOKS[ordinal % books::BOOKS.len()];
        let pages = book.pages.iter().map(|p| p.to_string()).collect();
        self.screen = Screen::Dialogue(Dialogue::new(book.title, pages, DialogueKind::Book));
    }

    /// The moon-mint patch off the cave path — Granny Sorrel's favor.
    fn pick_herb(&mut self) {
        if !self.has_flag(sides::SORREL_ASKED) {
            self.toast(
                "A silvery patch of moon-mint. It smells like cool evenings and somebody's kettle.",
            );
        } else if !self.has_flag(sides::SORREL_MINT) {
            self.set_flag(sides::SORREL_MINT);
            self.toast(
                "You pick a sprig of moon-mint for Granny Sorrel. The patch barely notices.",
            );
        } else {
            self.toast("The moon-mint is regrowing, unhurried. One sprig was plenty.");
        }
    }

    /// The chest in the storehouse cellar: locked until Old Nettle's rusted
    /// key has been carried home to it.
    fn try_chest(&mut self) {
        if self.has_flag(sides::CHEST_OPENED) {
            self.toast("The chest stands open and empty, but keeps the shape of a secret.");
            return;
        }
        if !self.has_flag(sides::NETTLE_MET) {
            self.toast("A sturdy old chest, locked. The keyhole is small, rusted, and patient.");
            return;
        }
        self.set_flag(sides::CHEST_OPENED);
        self.sound(SoundEvent::ChestOpened);
        let id = stones::RUNESTONES.len() as u8; // the Keystone, the eighth
        self.flags.insert(sides::runestone_flag(id));
        self.sound(SoundEvent::StoneFound);
        let stone = stones::stone(id);
        let pages = vec![
            "Old Nettle's rusted key turns with a click the cellar has waited years to hear. Inside, wrapped in oilcloth: a runestone.".to_string(),
            stone.legend.to_string(),
            self.rubbing_line(),
        ];
        self.screen = Screen::Dialogue(Dialogue::new(stone.name, pages, DialogueKind::Stone));
    }

    /// A hidden runestone found: rub its rune into the journal.
    fn touch_runestone(&mut self, x: i32, y: i32) {
        let Some(id) = zones::runestone_id(self.zone_idx, (x, y)) else {
            return; // a stone nobody catalogued; leave it its mystery
        };
        let flag = sides::runestone_flag(id);
        if self.flags.contains(&flag) {
            self.toast("The stone hums contentedly. Its rune is already in your journal.");
            return;
        }
        self.flags.insert(flag);
        self.sound(SoundEvent::StoneFound);
        let stone = stones::stone(id);
        let pages = vec![stone.legend.to_string(), self.rubbing_line()];
        self.screen = Screen::Dialogue(Dialogue::new(stone.name, pages, DialogueKind::Stone));
    }

    fn rubbing_line(&self) -> String {
        format!(
            "(You rub the rune into your journal. Runestones found: {}/{}.)",
            stones::found(&self.flags),
            stones::RUNESTONES.len()
        )
    }

    /// Catch-and-release, strictly. The river keeps its residents; you keep
    /// the count and the stories.
    fn go_fishing(&mut self) {
        self.fish += 1;
        let (px, py) = self.player;
        let h = hash2(px, py, 0xF15 ^ self.fish);
        let catch = items::CATCHES[h as usize % items::CATCHES.len()];
        self.toast(format!(
            "You cast Juniper's rod... {catch} (fish met: {})",
            self.fish
        ));
    }

    /// Sit down by the fire. The campfires are the world's clock now: a
    /// daytime rest carries you into the night, a night-time rest back to a
    /// bright new day — and between fires, the sky simply waits.
    fn rest_at_campfire(&mut self) {
        let wake = if self.phase() == DayPhase::Night {
            DayPhase::Day
        } else {
            DayPhase::Night
        };
        // The same fire on the same evening tells the same tale; a later rest
        // draws another.
        let h = hash2(self.player.0, self.player.1, 0x71DE ^ self.day_ticks);
        self.screen = Screen::Resting {
            lore: h as usize % lore::LORE.len(),
            t: 0,
            wake,
        };
    }

    fn resting_key(&mut self, code: Key) {
        let Screen::Resting { wake, t, .. } = self.screen else {
            return;
        };
        // Let the fade settle before a keystroke can wake you, so a mashed
        // key doesn't skip the moment entirely.
        if t < 4 {
            return;
        }
        match code {
            Key::Enter | Key::Char(' ') | Key::Char('e') | Key::Esc => {
                self.day_ticks = match wake {
                    DayPhase::Night => NIGHT_ANCHOR,
                    _ => DAY_ANCHOR,
                };
                // The clock turned while you dozed — the folk of the world
                // went where their evening (or their morning) takes them.
                self.apply_schedule();
                self.screen = Screen::World;
                let msg = match wake {
                    DayPhase::Night => {
                        "You wake to a sky full of stars. The world has gone quiet and gone to sleep around you."
                    }
                    _ => {
                        "Morning is well along when you wake beside cold ashes — the world washed new, the folk of it already stirring."
                    }
                };
                self.toast(msg);
            }
            _ => {}
        }
    }

    fn npc_dialogue(&self, x: i32, y: i32) -> Dialogue {
        let npc = self.zone().npc_at(x, y).expect("checked above");
        // Side business first: some folk have favors going, off the quest road.
        if let Some(talk) = sides::talk(npc.name, &self.flags) {
            return Dialogue::new(npc.name, talk.pages, DialogueKind::Side(talk.set));
        }
        let idle = npc.idle.first().copied().unwrap_or("...").to_string();
        let Some(qid) = npc.quest else {
            return Dialogue::new(npc.name, vec![idle], DialogueKind::Flavor);
        };
        if self.completed.contains(&qid) {
            // Their errand is done: a warmer, grateful line if they have one.
            let thanks = npc
                .idle
                .get(1)
                .or(npc.idle.first())
                .copied()
                .unwrap_or("...")
                .to_string();
            return Dialogue::new(npc.name, vec![thanks], DialogueKind::Flavor);
        }
        let active = self.active_quest().map(|q| q.id);
        if active == Some(qid) {
            let quest = quests::quest(qid);
            if self.accepted.contains(&qid) {
                let pages = vec![
                    quest.reminder.to_string(),
                    format!(
                        "(Your quest scroll: {}/{} — edit it in your editor, then press c in the game to cast. q opens your journal, f asks Ferris for a hint.)",
                        checker::QUEST_DIR,
                        quest.file_name
                    ),
                ];
                return Dialogue::new(npc.name, pages, DialogueKind::Reminder);
            }
            let pages = quest.intro.iter().map(|p| p.to_string()).collect();
            return Dialogue::new(npc.name, pages, DialogueKind::Intro(qid));
        }
        // Their quest is further down the road; for now, pleasantries.
        Dialogue::new(npc.name, vec![idle], DialogueKind::Flavor)
    }

    fn start_cast(&mut self) {
        let Some(quest) = self.active_quest() else {
            self.toast("Every rune on the road is cast. Ferris suggests an armchair.");
            return;
        };
        if !self.accepted.contains(&quest.id) {
            self.toast(format!(
                "No rune prepared. {} in {} has work for you first.",
                quest.npc, self.zones[quest.zone].name
            ));
            return;
        }
        self.cast_rx = Some(checker::cast(quest));
        self.screen = Screen::Casting { quest: quest.id };
    }

    fn ferris_hint(&mut self) {
        let Some(quest) = self.active_quest() else {
            self.toast("Ferris waves a claw: \"Nothing left to hint about!\"");
            return;
        };
        if !self.accepted.contains(&quest.id) {
            self.toast(format!(
                "Ferris looks up from beside your boots: \"No quest yet! Go chat with {} first.\"",
                quest.npc
            ));
            return;
        }
        let shown = self.hints.entry(quest.id).or_insert(0);
        if *shown < quest.hints.len() {
            *shown += 1;
        }
        self.screen = Screen::Journal;
    }

    fn dialogue_key(&mut self, code: Key) {
        let Screen::Dialogue(d) = &mut self.screen else {
            return;
        };
        match code {
            Key::Enter | Key::Char(' ') | Key::Char('e') => {
                let page_len = d.pages[d.page].chars().count();
                if d.revealed < page_len {
                    d.revealed = page_len; // skip the typewriter
                } else if d.page + 1 < d.pages.len() {
                    d.page += 1;
                    d.revealed = 0;
                    self.sounds.push(SoundEvent::PageTurned);
                } else {
                    self.end_dialogue();
                }
            }
            Key::Esc => self.end_dialogue(),
            _ => {}
        }
    }

    fn end_dialogue(&mut self) {
        let Screen::Dialogue(d) = &self.screen else {
            return;
        };
        match d.kind {
            DialogueKind::Intro(qid) => {
                let quest = quests::quest(qid);
                match checker::scaffold(quest) {
                    Ok(path) => {
                        self.accepted.insert(qid);
                        self.autosave();
                        self.toast(format!(
                            "Quest scroll written: {} — open it in your editor, then press c here to cast.",
                            path.display()
                        ));
                    }
                    Err(e) => self.toast(format!("The quest scroll wouldn't write itself: {e}")),
                }
                self.screen = Screen::World;
            }
            DialogueKind::Side(set) => {
                if let Some(flag) = set {
                    self.set_flag(flag);
                }
                self.screen = Screen::World;
            }
            DialogueKind::Success(qid) => {
                self.screen = Screen::World;
                if items::reward(qid).is_some() {
                    // A keepsake changes hands with the thanks.
                    self.sound(SoundEvent::KeepsakeGiven);
                }
                if qid == QUESTS.len() as u8 {
                    self.screen = Screen::Epilogue { page: 0 };
                } else if self.zone_cleared(self.zone_idx) && self.zone().gate.is_some() {
                    let msg = self.zone().unlock_msg;
                    self.toast(msg);
                    // And the road visibly opens: the camera glides out to
                    // the gate and the barrier rolls aside on screen.
                    self.gate_reveal = Some((self.zone_idx, self.tick));
                }
            }
            _ => self.screen = Screen::World,
        }
    }

    fn world_map_key(&mut self, code: Key) {
        match code {
            Key::Esc
            | Key::Char('m')
            | Key::Char('q')
            | Key::Char('e')
            | Key::Enter
            | Key::Char(' ') => {
                self.screen = Screen::World;
            }
            _ => {}
        }
    }

    fn journal_key(&mut self, code: Key) {
        match code {
            Key::Char('f') => self.ferris_hint(),
            Key::Esc | Key::Char('q') | Key::Enter | Key::Char(' ') | Key::Char('e') => {
                self.screen = Screen::World
            }
            _ => {}
        }
    }

    fn cast_result_key(&mut self, code: Key) {
        let Screen::CastResult {
            quest,
            outcome,
            scroll,
        } = &mut self.screen
        else {
            return;
        };
        match code {
            Key::Up | Key::Char('k') => *scroll = scroll.saturating_sub(1),
            Key::Down | Key::Char('j') => *scroll = scroll.saturating_add(1),
            Key::PageUp => *scroll = scroll.saturating_sub(10),
            Key::PageDown => *scroll = scroll.saturating_add(10),
            Key::Enter | Key::Esc | Key::Char(' ') | Key::Char('e') => {
                let quest_id = *quest;
                if matches!(outcome, Outcome::Pass { .. }) {
                    let q = quests::quest(quest_id);
                    let mut pages: Vec<String> = q.success.iter().map(|p| p.to_string()).collect();
                    if let Some(item) = items::reward(quest_id) {
                        pages.push(format!(
                            "({} tucks {} into your satchel. {})",
                            q.npc,
                            item.name(),
                            item.blurb()
                        ));
                    }
                    self.screen = Screen::Dialogue(Dialogue::new(
                        q.npc,
                        pages,
                        DialogueKind::Success(quest_id),
                    ));
                } else {
                    self.screen = Screen::World;
                }
            }
            _ => {}
        }
    }

    fn paused_key(&mut self, code: Key) {
        // The rest menu: [Back to the road, Text speed, Sound, Save & sleep].
        let Screen::Paused { selected } = &self.screen else {
            return;
        };
        let mut sel = *selected;
        match code {
            Key::Up | Key::Char('k') => {
                sel = (sel + 3) % 4;
                self.sound(SoundEvent::MenuMoved);
            }
            Key::Down | Key::Char('j') => {
                sel = (sel + 1) % 4;
                self.sound(SoundEvent::MenuMoved);
            }
            // Left/right nudges the toggles without leaving the menu.
            Key::Left | Key::Right if sel == 1 => self.cycle_text_speed(),
            Key::Left | Key::Right if sel == 2 => self.cycle_sound_level(),
            Key::Esc => {
                self.screen = Screen::World;
                return;
            }
            Key::Enter | Key::Char(' ') | Key::Char('e') => match sel {
                0 => {
                    self.screen = Screen::World;
                    return;
                }
                1 => self.cycle_text_speed(),
                2 => self.cycle_sound_level(),
                _ => {
                    self.autosave();
                    self.should_quit = true;
                    return;
                }
            },
            _ => return,
        }
        if let Screen::Paused { selected } = &mut self.screen {
            *selected = sel;
        }
    }

    fn epilogue_key(&mut self, code: Key) {
        let Screen::Epilogue { page } = &mut self.screen else {
            return;
        };
        match code {
            Key::Enter | Key::Char(' ') | Key::Char('e') => {
                if *page + 1 < EPILOGUE.len() {
                    *page += 1;
                } else {
                    self.screen = Screen::World;
                    self.toast("The road stays open. Wander as long as you like.");
                    self.autosave();
                }
            }
            Key::Esc => {
                self.screen = Screen::World;
                self.autosave();
            }
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub fn in_bounds(pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < MAP_W && pos.1 < MAP_H
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choosing_a_character_and_typing_a_name_begins_the_journey() {
        let mut app = App::new();
        app.has_save = false; // title item 0 is "A new journey"
        app.screen = Screen::Title { selected: 0 };
        app.on_key(Key::Enter); // new journey -> chooser
        assert!(matches!(app.screen, Screen::CharSelect { .. }));
        app.on_key(Key::Right); // second look
        for c in ['f', 'e', 'r', 'n'] {
            app.on_key(Key::Char(c));
        }
        app.on_key(Key::Backspace); // "fer"
        let Screen::CharSelect { idx, name } = &app.screen else {
            unreachable!()
        };
        assert_eq!(*idx, 1);
        assert_eq!(name, "Fer", "first letter capitalized, rest lower");
        app.on_key(Key::Enter);
        assert!(matches!(app.screen, Screen::World));
        assert_eq!(app.player_char, 1);
        assert_eq!(app.player_name(), "Fer");
        assert_eq!(app.phase(), DayPhase::Morning);
    }

    #[test]
    fn setting_off_unnamed_is_gently_refused() {
        let mut app = App::new();
        app.new_game();
        // Enter with a blank name: no journey begins, just a funny nudge.
        app.on_key(Key::Enter);
        assert!(
            matches!(app.screen, Screen::CharSelect { .. }),
            "a nameless traveller shouldn't set off"
        );
        assert!(app.toast.is_some(), "the empty name should draw a nudge");
        // Type a name, and now Enter sets you on the road.
        for c in ['a', 'r', 'a'] {
            app.on_key(Key::Char(c));
        }
        app.on_key(Key::Enter);
        assert!(matches!(app.screen, Screen::World));
        assert_eq!(app.player_name(), "Ara");
    }

    #[test]
    fn the_day_turns_through_its_four_phases() {
        assert_eq!(DayPhase::at(0), DayPhase::Morning);
        assert_eq!(DayPhase::at(DAY_START), DayPhase::Day);
        assert_eq!(DayPhase::at(EVENING_START), DayPhase::Evening);
        assert_eq!(DayPhase::at(NIGHT_START), DayPhase::Night);
        assert_eq!(DayPhase::at(DAY_LEN), DayPhase::Morning); // wraps around
        // The campfire anchors land where they claim to.
        assert_eq!(DayPhase::at(NIGHT_ANCHOR), DayPhase::Night);
        assert_eq!(DayPhase::at(DAY_ANCHOR), DayPhase::Day);
        assert_eq!(DayPhase::at(JOURNEY_START), DayPhase::Morning);
        // The sky stays a real brightness the whole day round, dark at the
        // dead of night, bright at midday, and never leaves 0..=1.
        for t in (0..DAY_LEN).step_by(500) {
            let l = sky_daylight(t);
            assert!((0.0..=1.0).contains(&l), "sky {l} out of range at {t}");
        }
        assert!(
            sky_daylight(NIGHT_START + NIGHT_LEN / 2) < 0.3,
            "night too bright"
        );
        assert!(
            sky_daylight(MORNING_LEN + DAY_LEN_MIN / 2) > 0.9,
            "midday too dim"
        );
    }

    #[test]
    fn outdoors_follows_the_clock_indoors_keeps_its_lamp() {
        let mut app = App::new();
        app.zone_idx = zones::EMBERWICK;
        app.day_ticks = NIGHT_START + NIGHT_LEN / 2; // deep night
        let night = app.daylight();
        app.day_ticks = MORNING_LEN + DAY_LEN_MIN / 2; // high noon
        let noon = app.daylight();
        assert!(noon > night, "midday should outshine midnight outdoors");
        assert!(!app.is_night());
        // The cave keeps its own dark regardless of the hour outside.
        app.zone_idx = zones::ECHO_CAVE;
        let fixed = app.daylight();
        app.day_ticks = NIGHT_START;
        assert_eq!(app.daylight(), fixed, "interiors ignore the sky");
        assert!(!app.is_night(), "indoors is never 'night'");
    }

    #[test]
    fn active_quest_walks_the_road_in_order() {
        let mut app = App::new();
        assert_eq!(app.active_quest().unwrap().id, 1);
        app.completed.insert(1);
        app.completed.insert(2);
        assert_eq!(app.active_quest().unwrap().id, 3);
    }

    #[test]
    fn zones_gate_until_their_quests_are_done() {
        let mut app = App::new();
        assert!(!app.zone_cleared(0));
        app.completed.extend([1, 2, 3, 4, 5, 6, 7]);
        assert!(app.zone_cleared(0));
        assert!(!app.zone_cleared(1));
    }

    #[test]
    fn walking_sounds_like_the_ground_underfoot() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.drain_sounds(); // whatever setup queued, clear it
        // Emberwick's spawn sits on the west road: stepping east lands on
        // packed earth, and the event says so — without a speaker in sight.
        app.player = app.zones[0].spawn;
        app.try_move(1, 0);
        assert_eq!(
            app.drain_sounds(),
            vec![SoundEvent::Stepped(Terrain::Earth)],
            "a road step should sound like earth"
        );
        // A gate-crossing puts us just before Emberwick's east arch; find a
        // grass tile to hear the soft world instead.
        let zone = &app.zones[0];
        let (gx, gy) = (0..MAP_H)
            .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
            .find(|&(x, y)| zone.tile(x, y).walkable() && zone.tile(x + 1, y) == Tile::Grass)
            .expect("grass next to standing room");
        app.player = (gx, gy);
        app.try_move(1, 0);
        assert_eq!(
            app.drain_sounds(),
            vec![SoundEvent::Stepped(Terrain::Soft)],
            "a meadow step should sound soft"
        );
        // Stepping through the bakery door is a step and a door both.
        app.player = (66, 22); // just south of the bakery door
        app.try_move(0, -1);
        assert_eq!(app.zone_idx, zones::BAKERY);
        let sounds = app.drain_sounds();
        assert!(
            sounds.contains(&SoundEvent::DoorUsed),
            "warping through a doorway should creak: {sounds:?}"
        );
        // And once drained, the queue is empty — nothing plays twice.
        assert!(app.drain_sounds().is_empty());
    }

    #[test]
    fn milestones_ring_their_jingles() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.drain_sounds();
        // A caught wild rune sparkles...
        app.screen = Screen::Encounter {
            rune: 1,
            selected: crate::content::wilds::wild(1).answer,
            phase: EncounterPhase::Asking,
        };
        app.on_key(Key::Enter);
        assert!(app.drain_sounds().contains(&SoundEvent::RuneCaught));
        // ...a runestone gleams...
        app.screen = Screen::World;
        let (zi, spot) = zones::runestone_spots()[0];
        app.zone_idx = zi;
        app.touch_runestone(spot.0, spot.1);
        assert!(app.drain_sounds().contains(&SoundEvent::StoneFound));
        // ...and a keepsake changing hands chimes like coins.
        app.completed.extend(1..=6);
        app.screen = Screen::Dialogue(Dialogue::new(
            "Well-keeper Bram",
            vec!["The storm-lantern is yours.".into()],
            DialogueKind::Success(6),
        ));
        app.on_key(Key::Enter);
        app.on_key(Key::Enter);
        assert!(app.drain_sounds().contains(&SoundEvent::KeepsakeGiven));
    }

    #[test]
    fn the_sound_dial_cycles_and_survives_the_save() {
        let mut app = App::new();
        assert_eq!(app.sound_level, 2, "the world starts at full sound");
        app.screen = Screen::Paused { selected: 2 };
        app.on_key(Key::Left);
        assert_eq!(app.sound_level, 0, "full -> off");
        app.on_key(Key::Left);
        assert_eq!(app.sound_level, 1, "off -> quiet");
    }

    #[test]
    fn the_map_opens_charts_and_round_trips() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.set_flag(&sides::visited_flag(0));
        // m opens the parchment, m (or esc) folds it away.
        app.on_key(Key::Char('m'));
        assert!(matches!(app.screen, Screen::WorldMap));
        app.on_key(Key::Char('m'));
        assert!(matches!(app.screen, Screen::World));
        app.on_key(Key::Char('m'));
        app.on_key(Key::Esc);
        assert!(matches!(app.screen, Screen::World));

        // Crossing a gate charts the zone beyond it.
        assert!(!app.has_flag(&sides::visited_flag(1)));
        app.completed.extend(1..=7);
        let gate = app.zones[0].gate.unwrap();
        app.player = (gate.0 - 1, gate.1);
        app.try_move(1, 0);
        assert_eq!(app.zone_idx, 1);
        assert!(app.has_flag(&sides::visited_flag(1)));

        // From a room behind a door, the map dot resolves to the door
        // outside — walked all the way back to the open sky if need be.
        app.zone_idx = zones::STOREHOUSE_CELLAR;
        let (z, pos) = app.map_spot();
        assert_eq!(z, 0, "the cellar charts under Emberwick");
        assert!(
            app.zones[0].warp_at(pos.0, pos.1).is_some(),
            "the dot should sit on the storehouse door"
        );
    }

    #[test]
    fn after_dark_the_folk_go_home_and_the_giver_keeps_watch() {
        let mut app = App::new();
        app.screen = Screen::World;
        // Emberwick's errands are done, so Poppy is free to head home come
        // dark; the woods' first errand (Pip's) is now active, so he keeps
        // his post whatever the hour.
        app.completed.extend(1..=7);
        app.day_ticks = NIGHT_ANCHOR;
        app.apply_schedule();
        assert!(
            app.zones[zones::BAKERY]
                .npcs
                .iter()
                .any(|n| n.name == "Baker Poppy"),
            "Poppy should spend the night in her bakery"
        );
        assert!(
            !app.zones[0].npcs.iter().any(|n| n.name == "Baker Poppy"),
            "Poppy can't be in two places at once"
        );
        let pip = app.zones[1]
            .npcs
            .iter()
            .find(|n| n.name == "Pip")
            .expect("the active giver keeps watch at his post");
        assert_eq!(pip.pos, (73, 19), "Pip should be watching the road");

        // Morning puts everyone back at their authored posts.
        app.day_ticks = DAY_ANCHOR;
        app.apply_schedule();
        let poppy = app.zones[0]
            .npcs
            .iter()
            .find(|n| n.name == "Baker Poppy")
            .expect("morning brings Poppy back out");
        assert_eq!(poppy.pos, (66, 25));
        assert!(
            !app.zones[zones::BAKERY]
                .npcs
                .iter()
                .any(|n| n.name == "Baker Poppy")
        );
    }

    #[test]
    fn every_night_anchor_is_standable_reachable_and_unshared() {
        use std::collections::{HashSet, VecDeque};
        let mut app = App::new();
        app.screen = Screen::World;
        // The whole road done: nobody is pinned, every night spot is used.
        app.completed.extend(1..=23);
        app.day_ticks = NIGHT_ANCHOR;
        app.apply_schedule();
        for zone in &app.zones {
            // Flood-fill of everywhere the player can stand tonight, NPCs
            // blocking their own tiles just like in the game.
            let mut seen = HashSet::from([zone.spawn]);
            let mut queue = VecDeque::from([zone.spawn]);
            while let Some((x, y)) = queue.pop_front() {
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let n = (x + dx, y + dy);
                    if in_bounds(n)
                        && !seen.contains(&n)
                        && zone.tile(n.0, n.1).walkable()
                        && zone.npc_at(n.0, n.1).is_none()
                    {
                        seen.insert(n);
                        queue.push_back(n);
                    }
                }
            }
            let mut taken = HashSet::new();
            for npc in &zone.npcs {
                assert!(
                    zone.tile(npc.pos.0, npc.pos.1).walkable(),
                    "{} spends the night standing in furniture at {:?} in {}",
                    npc.name,
                    npc.pos,
                    zone.name
                );
                assert!(
                    taken.insert(npc.pos),
                    "two folk share the tile {:?} in {} tonight",
                    npc.pos,
                    zone.name
                );
                let talkable = (-1..=1).any(|dy| {
                    (-1..=1).any(|dx| {
                        (dx, dy) != (0, 0) && seen.contains(&(npc.pos.0 + dx, npc.pos.1 + dy))
                    })
                });
                assert!(
                    talkable,
                    "{} is out of talking reach at {:?} in {} tonight",
                    npc.name, npc.pos, zone.name
                );
            }
        }
    }

    #[test]
    fn the_road_east_opens_with_a_little_show() {
        let mut app = App::new();
        app.screen = Screen::World;
        let spawn = app.player;
        // The last Emberwick errand passes; closing the success dialogue is
        // the moment the fallen oak rolls aside on screen.
        app.completed.extend(1..=7);
        app.screen = Screen::Dialogue(Dialogue::new(
            "Watchman Fitch",
            vec!["Clean and honest, that.".into()],
            DialogueKind::Success(7),
        ));
        app.on_key(Key::Enter); // finish the typewriter
        app.on_key(Key::Enter); // close the dialogue
        assert!(matches!(app.screen, Screen::World));
        let (zone, started) = app.gate_reveal.expect("the unlock plays a gate reveal");
        assert_eq!(zone, 0);

        // Left alone, the little show plays itself out...
        for _ in 0..REVEAL_TICKS {
            assert!(app.gate_reveal.is_some());
            app.on_tick();
        }
        assert!(
            app.gate_reveal.is_none(),
            "the reveal outstayed its welcome"
        );

        // ...and if the player is impatient, the first key skips it (and is
        // not also a step — nobody walks off mid-cutscene by accident).
        app.gate_reveal = Some((0, started));
        app.on_key(Key::Left);
        assert!(app.gate_reveal.is_none(), "any key hands the camera back");
        assert_eq!(app.player, spawn, "the skipping key is swallowed");
    }

    #[test]
    fn player_cannot_walk_into_a_tree() {
        let mut app = App::new();
        app.screen = Screen::World;
        // Find a tree next to some walkable tile.
        let zone = app.zone();
        let mut found = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                if zone.tile(x, y).walkable() && zone.tile(x + 1, y) == Tile::Tree {
                    found = Some((x, y));
                    break 'outer;
                }
            }
        }
        let (x, y) = found.expect("a forest without a tree next to a path?");
        app.player = (x, y);
        app.try_move(1, 0);
        assert_eq!(app.player, (x, y), "trees are for hugging, not phasing");
    }

    #[test]
    fn tall_grass_hides_wild_runes() {
        let mut app = App::new();
        app.screen = Screen::World;
        // Find two adjacent tall-grass tiles and shuffle between them.
        let mut spot = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                if app.zones[0].tile(x, y) == Tile::TallGrass
                    && app.zones[0].tile(x + 1, y) == Tile::TallGrass
                {
                    spot = Some((x, y));
                    break 'outer;
                }
            }
        }
        app.player = spot.expect("Emberwick grows tall grass in pairs");
        for _ in 0..300 {
            if matches!(app.screen, Screen::Encounter { .. }) {
                break;
            }
            app.try_move(1, 0);
            if matches!(app.screen, Screen::Encounter { .. }) {
                break;
            }
            app.try_move(-1, 0);
        }
        let Screen::Encounter { rune, .. } = app.screen else {
            panic!("600 steps of tall grass and not a single rustle");
        };

        // Answer correctly, purely through keystrokes.
        for _ in 0..wilds::wild(rune).answer {
            app.on_key(Key::Down);
        }
        app.on_key(Key::Enter);
        assert!(matches!(
            app.screen,
            Screen::Encounter {
                phase: EncounterPhase::Caught,
                ..
            }
        ));
        assert!(app.grimoire.contains(&rune), "caught rune not inscribed");
        app.on_key(Key::Enter);
        assert!(matches!(app.screen, Screen::World));
    }

    #[test]
    fn caught_runes_stop_rustling() {
        // With every Emberwick rune already inscribed, no amount of walking
        // through the grass should stir one up again.
        let mut app = App::new();
        app.screen = Screen::World;
        for rune in wilds::in_zone(0) {
            app.grimoire.insert(rune.id);
        }
        let mut spot = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                if app.zones[0].tile(x, y) == Tile::TallGrass
                    && app.zones[0].tile(x + 1, y) == Tile::TallGrass
                {
                    spot = Some((x, y));
                    break 'outer;
                }
            }
        }
        app.player = spot.expect("Emberwick grows tall grass in pairs");
        for _ in 0..500 {
            app.try_move(1, 0);
            app.try_move(-1, 0);
            assert!(
                matches!(app.screen, Screen::World),
                "a mastered rune rustled anyway"
            );
        }
    }

    /// The campfires are the world's clock: the sky holds still between
    /// rests, and each rest toggles day and night.
    #[test]
    fn campfires_toggle_day_and_night_and_the_sky_waits_between() {
        let mut app = App::new();
        app.screen = Screen::World;
        // Left alone, the clock does not move.
        let before = app.day_ticks;
        for _ in 0..500 {
            app.on_tick();
        }
        assert_eq!(app.day_ticks, before, "the sky should wait for the player");
        // Emberwick's festival campfire.
        let fire = (0..MAP_H)
            .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
            .find(|&(x, y)| app.zones[0].tile(x, y) == Tile::Campfire)
            .expect("Emberwick keeps a campfire");
        app.player = (fire.0, fire.1 + 1);
        assert_eq!(app.phase(), DayPhase::Morning);
        app.on_key(Key::Char('e'));
        let Screen::Resting { wake, .. } = app.screen else {
            panic!("the campfire didn't invite a rest");
        };
        assert_eq!(
            wake,
            DayPhase::Night,
            "a daytime rest should reach nightfall"
        );
        // The fade has to settle before a keystroke wakes you.
        app.on_tick();
        app.on_tick();
        app.on_tick();
        app.on_tick();
        app.on_key(Key::Enter);
        assert!(matches!(app.screen, Screen::World));
        assert_eq!(app.phase(), DayPhase::Night);
        assert!(app.is_night());

        // Rest again from the night: wake to a bright new day.
        app.on_key(Key::Char('e'));
        for _ in 0..5 {
            app.on_tick();
        }
        app.on_key(Key::Enter);
        assert_eq!(app.phase(), DayPhase::Day);
        assert!(!app.is_night());
    }

    #[test]
    fn the_pause_menu_cycles_text_speed_and_saves_it() {
        let mut app = App::new();
        app.screen = Screen::Paused { selected: 1 }; // the text-speed row
        assert_eq!(app.text_speed, 1);
        app.on_key(Key::Enter); // cycle: normal -> fast
        assert_eq!(app.text_speed, 2);
        assert_eq!(app.reveal_step(), 4);
        app.on_key(Key::Right); // and again: fast -> slow (wraps)
        assert_eq!(app.text_speed, 0);
        // The row stays put while toggling.
        assert!(matches!(app.screen, Screen::Paused { selected: 1 }));
        // Selecting "Back to the road" leaves the menu.
        app.on_key(Key::Up);
        app.on_key(Key::Enter);
        assert!(matches!(app.screen, Screen::World));
    }

    #[test]
    fn arriving_somewhere_new_raises_a_banner() {
        let mut app = App::new();
        app.screen = Screen::World;
        assert!(app.banner.is_none());
        // Cross Emberwick's gate (with its quests done) into the woods.
        app.completed.extend([1, 2, 3, 4, 5, 6, 7]);
        let gate = app.zone().gate.unwrap();
        app.player = (gate.0 - 1, gate.1);
        app.on_key(Key::Right);
        assert_eq!(app.zone_idx, zones::WHISPERING_WOODS);
        let (name, _) = app.banner.as_ref().expect("a banner should have risen");
        assert!(name.contains("Whispering"), "banner named the wrong place");
    }

    #[test]
    fn fleeing_an_encounter_is_always_free() {
        let mut app = App::new();
        app.screen = Screen::Encounter {
            rune: 1,
            selected: 0,
            phase: EncounterPhase::Asking,
        };
        app.on_key(Key::Esc);
        assert!(matches!(app.screen, Screen::World));
        assert!(app.grimoire.is_empty());
    }

    #[test]
    fn a_fizzled_cast_scrolls_its_errors_with_vim_keys() {
        let mut app = App::new();
        app.screen = Screen::CastResult {
            quest: 1,
            outcome: Outcome::CompileFail {
                stderr: "error: line one\nerror: line two\n".into(),
            },
            scroll: 0,
        };
        // j scrolls down, k scrolls back up — just like the arrows.
        app.on_key(Key::Char('j'));
        app.on_key(Key::Char('j'));
        assert!(matches!(app.screen, Screen::CastResult { scroll: 2, .. }));
        app.on_key(Key::Char('k'));
        assert!(matches!(app.screen, Screen::CastResult { scroll: 1, .. }));
        // k saturates at the top rather than underflowing.
        app.on_key(Key::Char('k'));
        app.on_key(Key::Char('k'));
        assert!(matches!(app.screen, Screen::CastResult { scroll: 0, .. }));
    }

    #[test]
    fn the_echo_cave_needs_a_light() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.zone_idx = zones::WHISPERING_WOODS;
        let mouth = app.zones[zones::WHISPERING_WOODS].warps[0].at;
        app.player = (mouth.0, mouth.1 + 1);
        app.try_move(0, -1);
        assert_eq!(
            app.zone_idx,
            zones::WHISPERING_WOODS,
            "walked into pitch darkness without a lantern"
        );
        app.completed.insert(6); // Bram hands over the storm-lantern
        app.try_move(0, -1);
        assert_eq!(app.zone_idx, zones::ECHO_CAVE);
    }

    #[test]
    fn fishing_needs_the_rod() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.zone_idx = zones::SILVERFORD;
        // A standable spot by the water, with no NPC or sign to steal the interaction.
        let zone = &app.zones[zones::SILVERFORD];
        let mut spot = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                let clear = (-1..=1).all(|dy| {
                    (-1..=1).all(|dx| {
                        zone.npc_at(x + dx, y + dy).is_none()
                            && zone.tile(x + dx, y + dy) != Tile::Sign
                    })
                });
                if zone.tile(x, y).walkable() && zone.tile(x + 1, y) == Tile::Water && clear {
                    spot = Some((x, y));
                    break 'outer;
                }
            }
        }
        app.player = spot.expect("Silverford has a quiet riverbank somewhere");
        app.on_key(Key::Char('e'));
        assert_eq!(app.fish, 0, "fished without a rod");
        // Rodless, `e` at the bank falls through to a chat with Ferris.
        app.on_key(Key::Esc);
        app.completed.insert(17); // Juniper's spare rod
        app.on_key(Key::Char('e'));
        assert_eq!(app.fish, 1);
        assert!(app.toast.is_some(), "the catch deserves a mention");
    }

    #[test]
    fn library_shelves_read_aloud_in_order() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.zone_idx = zones::GREAT_LIBRARY;
        // Find two side-by-side shelves with standable floor beneath.
        let zone = app.zone();
        let mut spot = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                if zone.tile(x, y) == Tile::Bookshelf
                    && zone.tile(x + 1, y) == Tile::Bookshelf
                    && zone.tile(x, y + 1).walkable()
                    && zone.tile(x + 1, y + 1).walkable()
                {
                    spot = Some((x, y));
                    break 'outer;
                }
            }
        }
        let (x, y) = spot.expect("the Library has stacks with an aisle");

        app.player = (x, y + 1);
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the shelf had nothing to say");
        };
        assert!(matches!(d.kind, DialogueKind::Book));
        let first = d.speaker.clone();

        // One step along the stack: the next title in the collection.
        app.screen = Screen::World;
        app.player = (x + 1, y + 1);
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the second shelf had nothing to say");
        };
        assert_ne!(d.speaker, first, "neighboring shelves hold the same book");
    }

    /// Press Enter until the dialogue closes (side-quest flags land on close).
    fn click_through(app: &mut App) {
        for _ in 0..40 {
            if !matches!(app.screen, Screen::Dialogue(_)) {
                return;
            }
            app.on_key(Key::Enter);
        }
        panic!("dialogue never ended");
    }

    #[test]
    fn granny_sorrels_favor_walks_its_whole_arc() {
        let mut app = App::new();
        app.screen = Screen::World;

        // Tea with Granny Sorrel: she asks for moon-mint.
        app.zone_idx = zones::SORREL_COTTAGE;
        let granny = app.zone().npcs[0].pos;
        app.player = (granny.0, granny.1 + 1);
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("Granny Sorrel had nothing to say");
        };
        assert!(matches!(
            d.kind,
            DialogueKind::Side(Some(sides::SORREL_ASKED))
        ));
        click_through(&mut app);
        assert!(app.has_flag(sides::SORREL_ASKED));

        // The mint patch off the cave path in the woods.
        app.zone_idx = zones::WHISPERING_WOODS;
        let zone = app.zone();
        let mint = (0..MAP_H)
            .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
            .find(|&(x, y)| zone.tile(x, y) == Tile::Herb)
            .expect("the woods grow moon-mint");
        app.player = (mint.0 + 1, mint.1);
        app.on_key(Key::Char('e'));
        assert!(app.has_flag(sides::SORREL_MINT), "no sprig was picked");

        // Back to the kettle.
        app.zone_idx = zones::SORREL_COTTAGE;
        app.player = (granny.0, granny.1 + 1);
        app.on_key(Key::Char('e'));
        click_through(&mut app);
        assert!(app.has_flag(sides::SORREL_DONE), "the tea never happened");
    }

    #[test]
    fn the_cellar_chest_wants_nettles_key() {
        let mut app = App::new();
        app.screen = Screen::World;

        // The cellar door is as dark as the cave: lantern required.
        app.zone_idx = zones::STOREHOUSE;
        let cellar_door = app.zones[zones::STOREHOUSE].warps[1].at;
        app.player = (cellar_door.0, cellar_door.1 + 1);
        app.try_move(0, -1);
        assert_eq!(
            app.zone_idx,
            zones::STOREHOUSE,
            "walked into a pitch-dark cellar without a light"
        );
        app.completed.insert(6); // Bram's storm-lantern
        app.try_move(0, -1);
        assert_eq!(app.zone_idx, zones::STOREHOUSE_CELLAR);

        // The chest is locked without Old Nettle's key.
        let zone = app.zone();
        let chest = (0..MAP_H)
            .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
            .find(|&(x, y)| zone.tile(x, y) == Tile::Chest)
            .expect("the cellar keeps a chest");
        app.player = (chest.0 - 1, chest.1);
        app.on_key(Key::Char('e'));
        assert!(
            !app.has_flag(sides::CHEST_OPENED),
            "the lock gave way to nothing"
        );

        // Old Nettle's key turns it, and the Keystone is inside.
        app.set_flag(sides::NETTLE_MET);
        app.on_key(Key::Char('e'));
        assert!(app.has_flag(sides::CHEST_OPENED));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the chest opened silently");
        };
        assert!(matches!(d.kind, DialogueKind::Stone));
        assert!(
            app.has_flag(&sides::runestone_flag(8)),
            "the Keystone went missing"
        );
        click_through(&mut app);

        // Opening it twice would be greedy.
        app.on_key(Key::Char('e'));
        assert!(
            matches!(app.screen, Screen::World),
            "the empty chest reopened"
        );
    }

    #[test]
    fn runestones_rub_into_the_journal_once() {
        let mut app = App::new();
        app.screen = Screen::World;
        let (zone, pos) = zones::runestone_spots()[0]; // the Henstone
        app.zone_idx = zone;
        app.player = (pos.0, pos.1 + 1);
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the stone said nothing");
        };
        assert!(matches!(d.kind, DialogueKind::Stone));
        assert!(app.has_flag(&sides::runestone_flag(1)));
        assert_eq!(stones::found(&app.flags), 1);
        click_through(&mut app);

        // A second rub just hums.
        app.on_key(Key::Char('e'));
        assert!(matches!(app.screen, Screen::World));
        assert_eq!(stones::found(&app.flags), 1);
    }

    #[test]
    fn notes_indoors_read_like_signs() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.zone_idx = zones::STOREHOUSE;
        let note = app.zone().signs[0].pos;
        app.player = (note.0 + 1, note.1);
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the note was blank");
        };
        assert_eq!(d.speaker, "A note", "a crate is not a signpost");
    }

    #[test]
    fn doors_lead_inside_and_back_again() {
        let mut app = App::new();
        app.screen = Screen::World;
        let warp = app.zones[0].warps[0]; // the bakery's front door
        app.player = (warp.at.0, warp.at.1 + 1);
        app.try_move(0, -1);
        assert_eq!(app.zone_idx, warp.to_zone, "the door led nowhere");
        assert_eq!(app.player, warp.to_pos);
        assert!(app.zone().interior, "houses should be interiors");
        app.try_move(0, 1); // step back onto the room's own door
        assert_eq!(app.zone_idx, 0, "no way back out of the bakery");
        assert_eq!(app.player, (warp.at.0, warp.at.1 + 1));
    }

    /// Ferris has walked at your heels since before the road began — and he
    /// talks: with nothing else in reach, `e` is a word with your oldest
    /// friend.
    #[test]
    fn ferris_is_there_from_the_start_and_talks() {
        let mut app = App::new();
        app.screen = Screen::World;
        // A quiet corner of Emberwick: nothing interactable in the 3x3 reach,
        // so the chat fallback is what `e` finds.
        let zone = &app.zones[0];
        let mut spot = None;
        'outer: for y in 1..MAP_H - 1 {
            for x in 1..MAP_W - 1 {
                let clear = (-1..=1).all(|dy| {
                    (-1..=1).all(|dx| {
                        let (tx, ty) = (x + dx, y + dy);
                        zone.npc_at(tx, ty).is_none()
                            && zone.sign_at(tx, ty).is_none()
                            && !matches!(
                                zone.tile(tx, ty),
                                Tile::Bookshelf
                                    | Tile::Herb
                                    | Tile::Chest
                                    | Tile::Runestone
                                    | Tile::Campfire
                            )
                    })
                });
                if zone.tile(x, y).walkable() && clear {
                    spot = Some((x, y));
                    break 'outer;
                }
            }
        }
        app.player = spot.expect("Emberwick has a quiet corner somewhere");
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("Ferris had nothing to say");
        };
        assert_eq!(d.speaker, "Ferris", "the empty road should draw Ferris");
        assert!(matches!(d.kind, DialogueKind::Flavor));
        click_through(&mut app);

        // After dark he answers too, in his quieter voice.
        app.day_ticks = NIGHT_START + 10;
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("Ferris sleeps too soundly");
        };
        assert_eq!(d.speaker, "Ferris");
    }

    /// The pen's hens have things to say (mostly "cluck").
    #[test]
    fn the_pen_hens_answer_a_friendly_e() {
        let mut app = App::new();
        app.screen = Screen::World;
        let hen = app.zones[0]
            .critters
            .iter()
            .find(|c| c.kind == CritterKind::Chicken)
            .expect("Emberwick keeps hens")
            .pos;
        app.player = (hen.0 - 1, hen.1);
        assert!(
            app.zone().tile(app.player.0, app.player.1).walkable(),
            "no standing room beside the hen"
        );
        app.on_key(Key::Char('e'));
        let Screen::Dialogue(d) = &app.screen else {
            panic!("the hen said nothing at all");
        };
        assert_eq!(d.speaker, "A chicken");
        assert!(matches!(d.kind, DialogueKind::Flavor));
    }

    /// The companion trails exactly one step behind, never in scenery —
    /// every square it sits on is one the player just stood on.
    #[test]
    fn the_companion_follows_a_step_behind() {
        let mut app = App::new();
        app.screen = Screen::World;
        app.player = app.zones[0].spawn;
        app.companion_snap();
        let mut walked = 0;
        for (dx, dy) in [(1, 0), (1, 0), (0, 1), (-1, 0), (0, -1), (1, 0)] {
            let before = app.player;
            app.tick += 5; // separate ticks, like real steps
            app.try_move(dx, dy);
            if app.player != before {
                walked += 1;
                assert_eq!(app.companion, before, "not at the player's heels");
                assert!(
                    app.zone().tile(app.companion.0, app.companion.1).walkable(),
                    "companion left in scenery at {:?}",
                    app.companion
                );
            }
        }
        assert!(walked >= 2, "the walk never got going");
    }

    /// A walk turning diagonal fires two axis-moves on the same tick (the
    /// shell's held diagonal). The companion must take the corner's departure
    /// square — not the mid-diagonal one, which sits two tiles from
    /// `companion_prev` and past what its render glide covers: the frozen
    /// snap of playtest fame.
    #[test]
    fn the_companion_glides_when_a_walk_turns_diagonal() {
        let mut app = App::new();
        app.screen = Screen::World;
        // A spot with room to walk east, then east+south in one tick: the
        // whole path open ground, no doors, gates, folk or rustling grass.
        let open = |app: &App, x: i32, y: i32| {
            let tile = app.zone().tile(x, y);
            tile.walkable()
                && !matches!(tile, Tile::Gate | Tile::TallGrass)
                && app.zone().warp_at(x, y).is_none()
                && app.zone().npc_at(x, y).is_none()
                && !app.zone().critters.iter().any(|c| c.pos == (x, y))
        };
        let start = (0..MAP_H)
            .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
            .find(|&(x, y)| {
                open(&app, x, y)
                    && open(&app, x + 1, y)
                    && open(&app, x + 2, y)
                    && open(&app, x + 2, y + 1)
            })
            .expect("no open corner anywhere in Emberwick?");
        app.player = start;
        app.companion_snap();
        app.tick += 5;
        app.try_move(1, 0); // walking east...
        app.tick += 5;
        app.try_move(1, 0); // ...when south joins: both axes, one tick
        app.try_move(0, 1);
        assert_eq!(
            app.player,
            (start.0 + 2, start.1 + 1),
            "the diagonal never landed"
        );
        assert_eq!(
            app.companion,
            (start.0 + 1, start.1),
            "the companion should hold the corner's departure square"
        );
        let (dx, dy) = (
            app.companion_prev.0 - app.companion.0,
            app.companion_prev.1 - app.companion.1,
        );
        assert!(
            dx.abs() <= 1 && dy.abs() <= 1,
            "companion stepped {dx},{dy} — further than its glide can cover"
        );
    }

    /// Doors don't lose the little crab: it scurries through with you.
    #[test]
    fn the_companion_is_never_lost_across_a_warp() {
        let mut app = App::new();
        app.screen = Screen::World;
        let warp = app.zones[0].warps[0]; // the bakery's front door
        app.player = (warp.at.0, warp.at.1 + 1);
        app.companion_snap();
        app.try_move(0, -1); // step through the door
        assert_eq!(app.zone_idx, warp.to_zone);
        assert_eq!(
            app.companion, app.player,
            "the companion should be gathered in at the doorstep"
        );
        // And the first step deeper inside puts it right back at your heels
        // (not (0, 1) — that's the doorstep again, which would warp us out).
        let inside = app.player;
        let zone_now = app.zone_idx;
        app.tick += 5;
        for (dx, dy) in [(0, -1), (1, 0), (-1, 0)] {
            app.try_move(dx, dy);
            if app.player != inside {
                break;
            }
        }
        assert_eq!(app.zone_idx, zone_now);
        assert_ne!(app.player, inside, "no room to step inside the bakery?");
        assert_eq!(app.companion, inside);
    }

    #[test]
    fn all_npc_and_critter_spots_are_standable() {
        let app = App::new();
        for zone in &app.zones {
            for npc in &zone.npcs {
                assert!(
                    zone.tile(npc.pos.0, npc.pos.1).walkable(),
                    "{} is stuck in scenery at {:?} in {}",
                    npc.name,
                    npc.pos,
                    zone.name
                );
            }
            for critter in &zone.critters {
                assert!(
                    zone.tile(critter.home.0, critter.home.1).walkable()
                        || zone.tile(critter.home.0, critter.home.1) == Tile::Water,
                    "a critter is stuck in scenery at {:?} in {}",
                    critter.home,
                    zone.name
                );
            }
        }
    }

    #[test]
    fn every_zone_spawn_is_standable() {
        let app = App::new();
        for zone in &app.zones {
            assert!(
                zone.tile(zone.spawn.0, zone.spawn.1).walkable(),
                "spawn of {} is blocked",
                zone.name
            );
        }
    }
}
