pub struct Npc {
    pub name: &'static str,
    pub pos: (i32, i32),
    /// The quest this NPC hands out, if any.
    pub quest: Option<u8>,
    /// Flavor lines for when there is no quest business with them.
    pub idle: &'static [&'static str],
}

/// Small lives that wander near their homes. Their sprites live in the atlas
/// (`gfx/atlas.rs`), matched by kind in `gfx/scene.rs`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CritterKind {
    Chicken,
    Sheep,
    Frog,
    Moth,
    Cat,
    Dog,
    Boar,
    Duck,
    Donkey,
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
