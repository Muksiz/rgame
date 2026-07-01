use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc::Receiver;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::checker::{self, Outcome};
use crate::content::quests::{self, QUESTS, Quest};
use crate::save::{self, SaveData};
use crate::world::map::hash2;
use crate::world::map::{MAP_H, MAP_W, Tile, Zone};
use crate::world::zones;

/// How long a toast lingers, in ticks (~50ms each).
const TOAST_TICKS: u64 = 110;
/// Typewriter reveal speed, characters per tick.
const REVEAL_PER_TICK: usize = 2;

pub static EPILOGUE: &[&str] = &[
    "The tall doors of the Great Library swing wide, and warm lamplight spills down the steps and into the mist. Somewhere above, the shelves go up and up until they look like a night full of square stars.",
    "You think of the whole road at once: a lantern blooming gold over Emberwick, sheep folding into meadow grass, a token handed back across a rain-specked dock, a letter mended mid-sentence. Twelve small runes. One quiet journey.",
    "Ferris climbs out of your satchel and settles on your shoulder, watching the lamplight. \"You know,\" the little crab says, \"most spellbooks end where the good part starts. Ownership, lifetimes, traits... whole wings of this place we haven't touched.\"",
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
        Self {
            speaker: speaker.to_string(),
            pages,
            page: 0,
            revealed: 0,
            kind,
        }
    }
}

pub enum Screen {
    Title {
        selected: usize,
    },
    World,
    Dialogue(Dialogue),
    Journal,
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

pub struct App {
    pub screen: Screen,
    pub tick: u64,
    pub play_ticks: u64,
    pub zones: Vec<Zone>,
    pub zone_idx: usize,
    pub player: (i32, i32),
    pub completed: BTreeSet<u8>,
    pub accepted: BTreeSet<u8>,
    pub hints: BTreeMap<u8, usize>,
    pub toast: Option<(String, u64)>,
    pub cast_rx: Option<Receiver<Outcome>>,
    pub has_save: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let zones = zones::zones();
        let player = zones[0].spawn;
        Self {
            screen: Screen::Title { selected: 0 },
            tick: 0,
            play_ticks: 0,
            zones,
            zone_idx: 0,
            player,
            completed: BTreeSet::new(),
            accepted: BTreeSet::new(),
            hints: BTreeMap::new(),
            toast: None,
            cast_rx: None,
            has_save: save::exists(),
            should_quit: false,
        }
    }

    pub fn zone(&self) -> &Zone {
        &self.zones[self.zone_idx]
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

    pub fn toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), self.tick + TOAST_TICKS));
    }

    // ── ticking ────────────────────────────────────────────────────────────

    pub fn on_tick(&mut self) {
        self.tick += 1;
        if !matches!(self.screen, Screen::Title { .. }) {
            self.play_ticks += 1;
        }
        if let Some((_, until)) = &self.toast
            && self.tick > *until
        {
            self.toast = None;
        }
        if let Screen::Dialogue(d) = &mut self.screen {
            d.revealed = d.revealed.saturating_add(REVEAL_PER_TICK);
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

    pub fn on_key(&mut self, key: KeyEvent) {
        match &mut self.screen {
            Screen::Title { .. } => self.title_key(key.code),
            Screen::World => self.world_key(key.code),
            Screen::Dialogue(_) => self.dialogue_key(key.code),
            Screen::Journal => self.journal_key(key.code),
            Screen::Casting { .. } => {} // the runes are busy
            Screen::CastResult { .. } => self.cast_result_key(key.code),
            Screen::Paused { .. } => self.paused_key(key.code),
            Screen::Epilogue { .. } => self.epilogue_key(key.code),
        }
    }

    fn title_key(&mut self, code: KeyCode) {
        let items = if self.has_save { 3 } else { 2 };
        let Screen::Title { selected } = &mut self.screen else {
            return;
        };
        match code {
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                *selected = (*selected + items - 1) % items;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                *selected = (*selected + 1) % items;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                // With a save: [Continue, New Journey, Quit]; without: [New Journey, Quit].
                let choice = *selected;
                match (self.has_save, choice) {
                    (true, 0) => self.continue_game(),
                    (true, 1) | (false, 0) => self.new_game(),
                    _ => self.should_quit = true,
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn new_game(&mut self) {
        self.zone_idx = 0;
        self.player = self.zones[0].spawn;
        self.completed.clear();
        self.accepted.clear();
        self.hints.clear();
        self.play_ticks = 0;
        self.screen = Screen::World;
        self.toast("A quiet morning in Emberwick. Someone near the festival square could use a hand. (Arrows/WASD to walk, e to talk.)");
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
        self.completed = data.completed.into_iter().collect();
        self.accepted = data.accepted.into_iter().collect();
        self.hints = data.hints;
        self.zone_idx = data.zone.min(self.zones.len() - 1);
        self.play_ticks = data.play_ticks;
        let (x, y) = data.pos;
        self.player = if in_bounds((x, y)) && self.zone().tile(x, y).walkable() {
            (x, y)
        } else {
            self.zone().spawn
        };
    }

    fn autosave(&mut self) {
        let data = SaveData {
            completed: self.completed.iter().copied().collect(),
            accepted: self.accepted.iter().copied().collect(),
            hints: self.hints.clone(),
            zone: self.zone_idx,
            pos: self.player,
            play_ticks: self.play_ticks,
        };
        match save::save(&data) {
            Ok(()) => self.has_save = true,
            Err(e) => self.toast(format!("Couldn't write the save scroll: {e}")),
        }
    }

    fn world_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => self.try_move(0, -1),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => self.try_move(0, 1),
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => self.try_move(-1, 0),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => self.try_move(1, 0),
            KeyCode::Enter | KeyCode::Char('e') => self.interact(),
            KeyCode::Char('c') => self.start_cast(),
            KeyCode::Char('q') => self.screen = Screen::Journal,
            KeyCode::Char('f') => self.ferris_hint(),
            KeyCode::Esc => self.screen = Screen::Paused { selected: 0 },
            _ => {}
        }
    }

    fn try_move(&mut self, dx: i32, dy: i32) {
        let target = (self.player.0 + dx, self.player.1 + dy);

        // Stepping off the west edge walks back toward the previous zone.
        if target.0 < 0 {
            if self.zone_idx > 0 {
                self.zone_idx -= 1;
                let gate = self.zone().gate.unwrap_or(self.zone().spawn);
                self.player = (gate.0 - 2, gate.1);
                let name = self.zone().name;
                self.toast(format!("Back along the road, into {name}."));
                self.autosave();
            } else {
                self.toast("Home lies that way — but the Library first! You promised.");
            }
            return;
        }
        if !in_bounds(target) {
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
            self.player = target;
        }
    }

    fn try_gate(&mut self) {
        if self.zone_cleared(self.zone_idx) {
            let unlock = self.zone().unlock_msg;
            if self.zone_idx + 1 < self.zones.len() {
                self.zone_idx += 1;
                self.player = self.zone().spawn;
                self.toast(unlock);
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
        let sign = spots.iter().find_map(|(x, y)| {
            (self.zone().tile(*x, *y) == Tile::Sign)
                .then(|| self.zone().sign_at(*x, *y).map(|s| s.text))
                .flatten()
        });
        if let Some(text) = sign {
            self.screen = Screen::Dialogue(Dialogue::new(
                "Signpost",
                vec![text.to_string()],
                DialogueKind::Flavor,
            ));
        }
    }

    fn npc_dialogue(&self, x: i32, y: i32) -> Dialogue {
        let npc = self.zone().npc_at(x, y).expect("checked above");
        let idle = npc.idle.first().copied().unwrap_or("...").to_string();
        let Some(qid) = npc.quest else {
            return Dialogue::new(npc.name, vec![idle], DialogueKind::Flavor);
        };
        if self.completed.contains(&qid) {
            return Dialogue::new(npc.name, vec![idle], DialogueKind::Flavor);
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
                "Ferris peeks out: \"No quest yet! Go chat with {} first.\"",
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

    fn dialogue_key(&mut self, code: KeyCode) {
        let Screen::Dialogue(d) = &mut self.screen else {
            return;
        };
        match code {
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('e') => {
                let page_len = d.pages[d.page].chars().count();
                if d.revealed < page_len {
                    d.revealed = page_len; // skip the typewriter
                } else if d.page + 1 < d.pages.len() {
                    d.page += 1;
                    d.revealed = 0;
                } else {
                    self.end_dialogue();
                }
            }
            KeyCode::Esc => self.end_dialogue(),
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
            DialogueKind::Success(qid) => {
                self.screen = Screen::World;
                if qid == 12 {
                    self.screen = Screen::Epilogue { page: 0 };
                } else if self.zone_cleared(self.zone_idx) && self.zone().gate.is_some() {
                    let msg = self.zone().unlock_msg;
                    self.toast(msg);
                }
            }
            _ => self.screen = Screen::World,
        }
    }

    fn journal_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => self.screen = Screen::World,
            KeyCode::Char('f') => self.ferris_hint(),
            _ => {}
        }
    }

    fn cast_result_key(&mut self, code: KeyCode) {
        let Screen::CastResult {
            quest,
            outcome,
            scroll,
        } = &mut self.screen
        else {
            return;
        };
        match code {
            KeyCode::Up => *scroll = scroll.saturating_sub(1),
            KeyCode::Down => *scroll = scroll.saturating_add(1),
            KeyCode::PageUp => *scroll = scroll.saturating_sub(10),
            KeyCode::PageDown => *scroll = scroll.saturating_add(10),
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') => {
                let quest_id = *quest;
                if matches!(outcome, Outcome::Pass { .. }) {
                    let q = quests::quest(quest_id);
                    let pages = q.success.iter().map(|p| p.to_string()).collect();
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

    fn paused_key(&mut self, code: KeyCode) {
        let Screen::Paused { selected } = &mut self.screen else {
            return;
        };
        match code {
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Char('k')
            | KeyCode::Char('j')
            | KeyCode::Char('w')
            | KeyCode::Char('s') => *selected = 1 - *selected,
            KeyCode::Esc => self.screen = Screen::World,
            KeyCode::Enter | KeyCode::Char(' ') => {
                if *selected == 0 {
                    self.screen = Screen::World;
                } else {
                    self.autosave();
                    self.should_quit = true;
                }
            }
            _ => {}
        }
    }

    fn epilogue_key(&mut self, code: KeyCode) {
        let Screen::Epilogue { page } = &mut self.screen else {
            return;
        };
        match code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if *page + 1 < EPILOGUE.len() {
                    *page += 1;
                } else {
                    self.screen = Screen::World;
                    self.toast("The road stays open. Wander as long as you like.");
                    self.autosave();
                }
            }
            KeyCode::Esc => {
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
        app.completed.extend([1, 2, 3]);
        assert!(app.zone_cleared(0));
        assert!(!app.zone_cleared(1));
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
