// ══════════════════════════════════════════════════════════════════
//   Quest 20: The Lost Book                 ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Archivist Elm: "Any dockhand can bundle facts — no offense to
//   dockhands. A LIBRARY record has *abilities*. It answers for
//   itself: is it overdue? What does its catalogue card say?"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Abilities that belong to a type are *methods*, and they live in
//   an `impl` block:
//
//       impl Book {
//           fn is_overdue(&self) -> bool {
//               self.years_overdue > 0
//           }
//       }
//
//   `&self` is a polite borrow of the very record the method is
//   called on; its fields read as `self.title`, `self.pages`.
//   Methods are called with a dot: `book.is_overdue()`.
//
//   Carve BOTH methods into the impl block:
//
//     .is_overdue()      ->  bool: true past zero years
//     .catalogue_card()  ->  String: title, then pages in brackets —
//                            "A Field Guide to Polite Dragons (312 pages)"
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct Book {
    title: String,
    pages: u32,
    years_overdue: u32,
}

impl Book {
    // TODO: fn is_overdue(&self) -> bool

    // TODO: fn catalogue_card(&self) -> String
}

fn the_returned_book() -> Book {
    Book {
        title: String::from("A Field Guide to Polite Dragons"),
        pages: 312,
        years_overdue: 58, // no judgement. some judgement.
    }
}

fn main() {
    let book = the_returned_book();
    println!("{}", book.catalogue_card());
    println!(
        "Overdue? {}",
        if book.is_overdue() { "Fifty-eight years. NO JUDGEMENT." } else { "Not at all." }
    );
}

// ─── Elm's four-angle inspection (leave this part alone) ──────────
#[test]
fn the_record_answers_for_itself() {
    let book = the_returned_book();
    assert!(book.is_overdue());
    assert_eq!(
        book.catalogue_card(),
        "A Field Guide to Polite Dragons (312 pages)"
    );
}

#[test]
fn a_punctual_book_is_not_accused() {
    let punctual = Book {
        title: String::from("On Time: A Memoir"),
        pages: 41,
        years_overdue: 0,
    };
    assert!(!punctual.is_overdue());
}
