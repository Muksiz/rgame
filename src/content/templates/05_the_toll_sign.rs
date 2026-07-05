// ══════════════════════════════════════════════════════════════════
//   Quest 5: The Toll Sign                        ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Toll-keeper Hobb: "Four coins a wagon, four slots on the board —
//   every day, world without end. I want the numbers CARVED in, not
//   chalked on. And then the board can do the evening sums itself."
//
//   (These `//` lines, by the way, are comments — notes for humans.
//   The compiler reads right past them. The scrolls are full of
//   them, and Hobb approves: carve your reasons next to your work.)
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   1. Carve in the constants. A `const` never changes, is known
//      before the program even runs, and spells out its type:
//
//          const NAME: u32 = value;
//
//      (Capitals by convention — that's how you spot one at a
//      glance.) The board wants TOLL_PRICE (4 coins) and
//      BOARD_SLOTS (4 slots).
//
//   2. Whole-number arithmetic for the evening count:
//      `/` divides and keeps only the whole part (21 / 4 is 5);
//      `%` gives what's left over (21 % 4 is 1).
//
//   Fix the board, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

// TODO: carve the two constants in, right about here.

fn days_takings(wagons: u32) -> u32 {
    wagons * TOLL_PRICE
}

fn coins_per_slot(takings: u32) -> u32 {
    // TODO: split the takings EVENLY across the board's slots —
    // whole coins only, `/` drops the remainder.
    todo!("an even split across BOARD_SLOTS")
}

fn coins_for_the_tea_tin(takings: u32) -> u32 {
    // TODO: whatever the even split leaves behind goes in the tea
    // tin — `%` finds exactly that remainder.
    todo!("the remainder after the slots take their share")
}

fn main() {
    let takings = days_takings(5);
    println!("Five wagons: {takings} coins.");
    println!(
        "{} per slot, {} for the tea tin.",
        coins_per_slot(takings),
        coins_for_the_tea_tin(takings)
    );
}

// ─── Hobb checks the board every morning (leave this part alone) ──
#[test]
fn the_price_is_carved_in() {
    assert_eq!(days_takings(1), 4, "one wagon, four coins — thirty years running");
    assert_eq!(days_takings(5), 20);
}

#[test]
fn the_evening_split_is_even() {
    assert_eq!(coins_per_slot(20), 5, "20 coins across 4 slots");
    assert_eq!(coins_per_slot(21), 5, "whole coins only — no splitting a coin");
}

#[test]
fn the_tea_tin_gets_the_rest() {
    assert_eq!(coins_for_the_tea_tin(20), 0);
    assert_eq!(coins_for_the_tea_tin(21), 1);
    assert_eq!(coins_for_the_tea_tin(23), 3);
}
