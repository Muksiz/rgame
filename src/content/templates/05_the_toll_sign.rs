// ══════════════════════════════════════════════════════════════════
//   Quest 5: The Toll Sign                        ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Toll-keeper Hobb: "Four coins, four slots on the board — every
//   wagon, every day, world without end. Not a number that should
//   ever wobble. I want it carved in, not chalked on."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `let` bindings can change (with `mut`) or be shadowed by a new
//   `let` — but sometimes a value should never, ever move, and the
//   compiler should know that up front. That's `const`:
//
//       const NAME: Type = value;
//
//   (Always uppercase, by convention — it's how you spot one at a
//   glance.) Declare `TOLL_PRICE` and `TOLL_SLOTS` as constants
//   below, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn toll_board() -> [u32; TOLL_SLOTS] {
    [TOLL_PRICE; TOLL_SLOTS]
}

fn main() {
    println!("{:?}", toll_board());
}

// ─── Hobb checks the board every morning (leave this part alone) ──
#[test]
fn four_wagons_four_coins() {
    assert_eq!(toll_board(), [4, 4, 4, 4]);
}
