// ══════════════════════════════════════════════════════════════════
//   Quest 7: The Map Pins                         ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Cartographer Reed: "Every landmark gets a pin, and a pin is
//   just two numbers traveling together — column, then row. Never
//   split them up, never mix up the order. The well is column 12,
//   row 4. Pin it, and then read it back to me."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A tuple bundles a fixed handful of values into one: `(12, 4)`
//   is a pin. Pull the pieces back out either by position —
//   `pin.0`, `pin.1` — or by destructuring: `let (x, y) = pin;`
//
//   Fix the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn well_pin() -> (i32, i32) {
    // TODO: the well sits at column 12, row 4 — build that pin
    (0, 0)
}

fn pin_label(pin: (i32, i32)) -> String {
    // TODO: pull the column and row out of `pin` and label them
    todo!()
}

fn main() {
    println!("{}", pin_label(well_pin()));
}

// ─── Reed checks every pin against the map (leave this part alone) ─
#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_pin_reads_clearly() {
    assert_eq!(pin_label((12, 4)), "column 12, row 4");
}
