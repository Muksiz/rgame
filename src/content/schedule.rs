//! Where the named folk spend the night. The sky waits at day or night and
//! only a campfire rest turns it — so a schedule here is just the *other*
//! anchor: by day everyone stands at their authored post; after dark the
//! ones with somewhere to go, go there. Interiors are ordinary zones, so a
//! night spot can be indoors — which finally gives the furnished rooms
//! their owners come evening.
//!
//! Positions are derived, never stored (`App::apply_schedule` re-derives
//! them from the phase and the active quest), so old saves stay valid for
//! free — and while an NPC's errand is the active quest they ignore the
//! hour and keep watch at their post instead.

use crate::world::zones::{
    BAKERY, FERNSHADE_COTTAGE, GLOWWORM, GREAT_LIBRARY, ROWAN_COTTAGE, SILVERFORD, room_spot,
};

/// The night anchor for a named NPC, if they have somewhere to be after
/// dark: `(zone index, tile)`. `None` means they keep their daytime post
/// (and, outdoors, sleep on it — Ambrose would have it no other way).
pub fn night_spot(npc: &str) -> Option<(usize, (i32, i32))> {
    Some(match npc {
        // Emberwick: the elder and the baker head home; the rest of the
        // village nods off where the evening finds them, as it always has.
        "Elder Rowan" => (ROWAN_COTTAGE, room_spot(6, 5)),
        "Baker Poppy" => (BAKERY, room_spot(5, 1)),
        // Fernshade: the kids are called in to Grandmother Ivy's, and the
        // grown folk take their evening at the Glowworm.
        "Pip" => (FERNSHADE_COTTAGE, room_spot(2, 3)),
        "Wren" => (FERNSHADE_COTTAGE, room_spot(7, 5)),
        "Weaver Sallow" => (GLOWWORM, room_spot(6, 3)),
        "Forager Maren" => (GLOWWORM, room_spot(7, 2)),
        // Silverford has no doors to close, so the river folk make camp:
        // the dock pair down on the piers by the ferry (Wick on the short
        // south pier, where he blocks nobody's way out), Morrow at his fire.
        "Ferryman Wick" => (SILVERFORD, (147, 53)),
        "Dockhand Fenn" => (SILVERFORD, (144, 49)),
        "Hermit Morrow" => (SILVERFORD, (119, 58)),
        // Hearthspire's scholars withdraw into the Library after dark (the
        // Golem sleeps where it stands; that is its whole occupation).
        "Archivist Elm" => (GREAT_LIBRARY, room_spot(7, 13)),
        "Sage Alderly" => (GREAT_LIBRARY, room_spot(21, 10)),
        "Scribe Faye" => (GREAT_LIBRARY, room_spot(30, 13)),
        _ => return None,
    })
}
