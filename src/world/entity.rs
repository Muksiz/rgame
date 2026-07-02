pub struct Npc {
    pub name: &'static str,
    pub glyph: char,
    pub color: (u8, u8, u8),
    pub pos: (i32, i32),
    /// The quest this NPC hands out, if any.
    pub quest: Option<u8>,
    /// Flavor lines for when there is no quest business with them.
    pub idle: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CritterKind {
    Chicken,
    Sheep,
    Frog,
    Moth,
    Cat,
}

impl CritterKind {
    pub fn glyph(self) -> char {
        match self {
            CritterKind::Chicken => 'c',
            CritterKind::Sheep => 'S',
            CritterKind::Frog => 'f',
            CritterKind::Moth => 'm',
            CritterKind::Cat => 'k',
        }
    }

    pub fn color(self) -> (u8, u8, u8) {
        match self {
            CritterKind::Chicken => (235, 220, 190),
            CritterKind::Sheep => (240, 238, 230),
            CritterKind::Frog => (120, 190, 90),
            CritterKind::Moth => (210, 205, 230),
            CritterKind::Cat => (214, 164, 110),
        }
    }
}

pub struct Critter {
    pub kind: CritterKind,
    pub pos: (i32, i32),
    pub home: (i32, i32),
}

impl Critter {
    pub fn new(kind: CritterKind, home: (i32, i32)) -> Self {
        Self {
            kind,
            pos: home,
            home,
        }
    }
}

pub struct Sign {
    pub pos: (i32, i32),
    pub text: &'static str,
}
