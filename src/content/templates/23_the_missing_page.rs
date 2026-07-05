// ══════════════════════════════════════════════════════════════════
//   Quest 23: The Missing Page                   ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Scribe Faye: "The great catalogue is missing page fifty-eight,
//   torn out who-knows-when. I mean to rewrite it fresh tonight.
//   One last rune, rune-smith, and it's one ability short of ready."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A fresh page isn't made by asking an existing page — there's no
//   `self` to ask. It's made by the TYPE, with an *associated
//   function*: no `self` parameter at all, called with `::` on the
//   type's name, exactly like `String::from`:
//
//       let page = CataloguePage::new(58);
//
//   Inside the impl block, `Self` means the type itself:
//
//       fn new(number: u32) -> Self {
//           Self { number, entries: 0 }     // shorthand welcome
//       }
//
//   Write BOTH halves of the rune — this is the capstone, and it's
//   only pieces you already know, standing together:
//
//     CataloguePage::new(number)  ->  a fresh page, zero entries
//     .record_entry()             ->  &mut self, one more entry
//
//   Then press `c` in the game. The catalogue — and your journey
//   down this road — will be whole.
// ──────────────────────────────────────────────────────────────────

struct CataloguePage {
    number: u32,
    entries: u32,
}

impl CataloguePage {
    // TODO: fn new(number: u32) -> Self — a fresh page, zero entries

    // TODO: fn record_entry(&mut self) — one more entry on the page

    fn is_filled(&self) -> bool {
        self.entries >= 3
    }
}

fn rewrite_the_missing_page() -> CataloguePage {
    let mut page = CataloguePage::new(58);
    page.record_entry(); // A Field Guide to Polite Dragons — returned
    page.record_entry(); // On Time: A Memoir — renewed
    page.record_entry(); // The River Delivers: Collected Letters — new
    page
}

fn main() {
    let page = rewrite_the_missing_page();
    println!(
        "Page {} rewritten: {} entries. {}",
        page.number,
        page.entries,
        if page.is_filled() { "The catalogue is whole." } else { "Still a gap..." }
    );
}

// ─── Faye checks the catalogue against the shelf (leave alone) ────
#[test]
fn the_page_is_made_fresh_by_the_type() {
    let page = CataloguePage::new(58);
    assert_eq!(page.number, 58);
    assert_eq!(page.entries, 0, "a fresh page starts empty");
    assert!(!page.is_filled());
}

#[test]
fn the_missing_page_is_made_whole() {
    let page = rewrite_the_missing_page();
    assert_eq!(page.number, 58);
    assert_eq!(page.entries, 3);
    assert!(page.is_filled());
}
