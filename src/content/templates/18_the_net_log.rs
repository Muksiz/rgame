// ══════════════════════════════════════════════════════════════════
//   Quest 18: The Net Log                       ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Net-mender Sil: "The tide chart only cares about the first three
//   days of the week — the rest is noise for my purposes. I don't
//   need my OWN copy of the whole week, mind, just a look at the
//   start of it."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A slice, `&[T]`, borrows a stretch of a list without owning any
//   of it. `&week[..3]` borrows just the first three entries;
//   `&week[..]` borrows the whole thing.
//
//   Narrow the slice to the first three days, then press `c`.
// ──────────────────────────────────────────────────────────────────

fn early_week_total(week: &[u32]) -> u32 {
    let early = &week[..]; // TODO: only the first THREE days count
    let mut total = 0;
    for count in early {
        total += count;
    }
    total
}

fn main() {
    let week = [2, 4, 6, 100, 100, 100, 100];
    println!("Early-week total: {}", early_week_total(&week));
}

// ─── Sil checks the chart against the log (leave this part alone) ─
#[test]
fn only_the_early_days_count() {
    assert_eq!(early_week_total(&[2, 4, 6, 100, 100, 100, 100]), 12);
}

#[test]
fn a_short_week_still_works() {
    assert_eq!(early_week_total(&[1, 1, 1]), 3);
}
