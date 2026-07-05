// ══════════════════════════════════════════════════════════════════
//   Quest 19: A Message in a Bottle         ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Hermit Morrow: "The river brought me a letter. In two pieces,
//   as usual — I mend those by hand. It's the ARCHIVE that's
//   defeated me: I want the archive-rune to read a whole record
//   aloud, every field at once, without me writing the formatting
//   out like a scribe with a quill cramp."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   The `{:?}` rune-marks print a value in *debug form* — the whole
//   struct, every field named. But a struct must OPT IN by deriving
//   the ability. One line, directly above the definition:
//
//       #[derive(Debug)]
//       struct Letter { ... }
//
//   That attribute asks the compiler to write the entire
//   printing-spell for you. (It's very good at it. Try `{:#?}`
//   afterwards, too — same record, spread airily over many lines.)
//
//   Add the line, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

// TODO: the archive-rune can't read this record aloud until it
// derives Debug.
struct Letter {
    sender: String,
    pieces: u32,
    year: u32,
}

fn todays_delivery() -> Letter {
    Letter {
        sender: String::from("M."),
        pieces: 2, // the river is enthusiastic about delivery
        year: 892,
    }
}

fn archive_line(letter: &Letter) -> String {
    format!("{letter:?}")
}

fn main() {
    let letter = todays_delivery();
    println!("The archive-rune reads: {}", archive_line(&letter));
    println!("And in the airy form:\n{letter:#?}");
}

// ─── Morrow's careful records (leave this part alone) ─────────────
#[test]
fn the_record_reads_aloud_whole() {
    assert_eq!(
        archive_line(&todays_delivery()),
        "Letter { sender: \"M.\", pieces: 2, year: 892 }"
    );
}

#[test]
fn the_letter_is_from_an_old_friend() {
    let letter = todays_delivery();
    assert_eq!(letter.sender, "M.");
    assert_eq!(letter.pieces, 2);
}
