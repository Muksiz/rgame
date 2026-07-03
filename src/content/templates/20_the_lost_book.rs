// ══════════════════════════════════════════════════════════════════
//   Quest 20: The Lost Book                 ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Archivist Elm: "Before it comes home to the Library it must be
//   CATALOGUED. A record-rune bundles facts so they travel as one:
//   we call the bundle a `struct`. Dignified word."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   The struct is already defined. Build one *value* of it, naming
//   every field:
//
//       Book { title: ..., pages: ..., years_overdue: ... }
//
//   The paperwork:  title "A Field Guide to Polite Dragons",
//   312 pages, 58 years overdue. (A `String` title needs
//   `.to_string()` — string literals are only borrowed `&str`.)
//
//   Replace the `todo!()`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct Book {
    title: String,
    pages: u32,
    years_overdue: u32,
}

fn catalogue_the_book() -> Book {
    todo!("fill in the record: title, pages, years_overdue")
}

fn main() {
    let book = catalogue_the_book();
    println!("Catalogued: '{}', {} pages.", book.title, book.pages);
}

// ─── Elm's four-angle inspection (leave this part alone) ──────────
#[test]
fn the_record_is_complete() {
    let book = catalogue_the_book();
    assert_eq!(book.title, "A Field Guide to Polite Dragons");
    assert_eq!(book.pages, 312);
    assert_eq!(book.years_overdue, 58);
}
