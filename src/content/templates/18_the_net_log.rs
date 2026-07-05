// ══════════════════════════════════════════════════════════════════
//   Quest 18: The Net Log                       ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Net-mender Sil: "The tide chart, for my purposes, is three
//   numbers: Monday, Tuesday, Wednesday. I could write a struct
//   with three solemn field names, but the POSITIONS already say
//   it. There's a terser shape for that."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A *tuple struct* has a name but no field names — order is the
//   meaning:
//
//       struct EarlyWeek(u32, u32, u32);     // note the semicolon
//
//   Build one like a function call — `EarlyWeek(2, 4, 6)` — and
//   read the pieces by position, like a tuple's: `.0`, `.1`, `.2`.
//   The name still counts for typing: an EarlyWeek can never be
//   mistaken for any other three numbers standing together.
//
//   1. Define the `EarlyWeek` tuple struct (three u32 counts).
//   2. Read this week's chart into one: 2, 4, 6.
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

// TODO: define the `EarlyWeek` tuple struct here.

fn read_chart() -> EarlyWeek {
    // TODO: Monday 2, Tuesday 4, Wednesday 6.
    todo!()
}

fn early_total(days: EarlyWeek) -> u32 {
    days.0 + days.1 + days.2
}

fn main() {
    let days = read_chart();
    println!("Tuesday's count: {}", days.1);
    println!("Early-week total: {}", early_total(read_chart()));
}

// ─── Sil checks the chart against the log (leave this part alone) ─
#[test]
fn the_early_week_is_bundled() {
    let days = read_chart();
    assert_eq!(days.0, 2);
    assert_eq!(days.1, 4);
    assert_eq!(days.2, 6);
}

#[test]
fn the_tally_reads_twelve() {
    assert_eq!(early_total(read_chart()), 12);
}
