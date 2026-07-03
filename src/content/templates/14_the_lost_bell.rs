// ══════════════════════════════════════════════════════════════════
//   Quest 14: The Lost Bell                       ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Woodward Sable: "Some evenings I find the old bell. Some
//   evenings I don't. I need a rune honest about that — not a
//   number that pretends to mean something when the answer is just
//   'nothing today'."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `Option<T>` holds either `Some(value)` or `None` — Rust's way of
//   saying "maybe" right in the type, so nobody can forget to check.
//
//   Return `Some(7)` when the bell turns up, `None` when it doesn't.
//   Fix the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn search_for_the_bell(found_today: bool) -> Option<u32> {
    // TODO: Some(7) if found_today, otherwise None
    todo!()
}

fn evening_report(found_today: bool) -> String {
    match search_for_the_bell(found_today) {
        Some(marker) => format!("Found it! Marker {marker}."),
        None => String::from("No luck today. Tomorrow, then."),
    }
}

fn main() {
    println!("{}", evening_report(true));
}

// ─── Sable makes the rounds every dusk (leave this part alone) ────
#[test]
fn some_evenings_it_turns_up() {
    assert_eq!(search_for_the_bell(true), Some(7));
}

#[test]
fn some_evenings_it_does_not() {
    assert_eq!(search_for_the_bell(false), None);
}

#[test]
fn the_report_reads_true() {
    assert_eq!(evening_report(true), "Found it! Marker 7.");
    assert_eq!(evening_report(false), "No luck today. Tomorrow, then.");
}
