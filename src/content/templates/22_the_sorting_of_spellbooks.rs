// ══════════════════════════════════════════════════════════════════
//   Quest 22: The Sorting of Spellbooks     ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Sage Alderly: "Fifty years of homecomings pile up on the
//   returns cart, and the cart keeps its own count — books waiting,
//   books shelved. It can already ANSWER (that's `all_shelved`,
//   below, a looking-method). What it cannot yet do is SHELVE."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A method that CHANGES the very struct it's called on borrows it
//   with the pen — `&mut self`:
//
//       fn shelve_one(&mut self) {
//           // one off the waiting pile, one onto the shelved count
//       }
//
//   Compare with `all_shelved(&self)`, which only looks. Same
//   pen-rules as the echo cave: the caller's binding must be `mut`
//   (the sorting below already is), and the pen is only held for
//   the length of the call.
//
//   Carve in `shelve_one`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct ReturnsCart {
    waiting: u32,
    shelved: u32,
}

impl ReturnsCart {
    fn all_shelved(&self) -> bool {
        self.waiting == 0
    }

    // TODO: fn shelve_one(&mut self) — waiting down one, shelved up one
}

fn the_evening_sorting() -> ReturnsCart {
    let mut cart = ReturnsCart { waiting: 3, shelved: 0 };
    cart.shelve_one();
    cart.shelve_one();
    cart.shelve_one();
    cart
}

fn main() {
    let cart = the_evening_sorting();
    println!(
        "{} shelved, {} waiting. {}",
        cart.shelved,
        cart.waiting,
        if cart.all_shelved() { "The books are humming." } else { "The cart sulks on." }
    );
}

// ─── The Library's final count (leave this part alone) ────────────
#[test]
fn every_homecoming_is_counted() {
    let cart = the_evening_sorting();
    assert_eq!(cart.shelved, 3);
    assert_eq!(cart.waiting, 0);
    assert!(cart.all_shelved());
}

#[test]
fn a_loaded_cart_knows_it() {
    let cart = ReturnsCart { waiting: 2, shelved: 0 };
    assert!(!cart.all_shelved());
}

#[test]
fn one_shelving_at_a_time() {
    let mut cart = ReturnsCart { waiting: 2, shelved: 5 };
    cart.shelve_one();
    assert_eq!(cart.waiting, 1);
    assert_eq!(cart.shelved, 6);
}
