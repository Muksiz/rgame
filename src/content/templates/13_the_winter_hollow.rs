// ══════════════════════════════════════════════════════════════════
//   Quest 13: The Winter Hollow                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Hollow-keeper Yew: "An array wants to know its size before it's
//   born. My hoard doesn't work that way — some years three good
//   gathering days, some years thirty. I need something that grows
//   as the acorns come in."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `Vec<T>` is a growable list. Start one empty and add to it as
//   you go:
//
//       let mut hoard = Vec::new();
//       hoard.push(3);
//
//   Fix the rune so each day's count gets pushed in, then press `c`.
// ──────────────────────────────────────────────────────────────────

fn gather_acorns(day1: u32, day2: u32, day3: u32) -> Vec<u32> {
    // TODO: start an empty Vec (`Vec::new()`) and `.push()` each day's count
    todo!()
}

fn main() {
    println!("Yew's hoard: {:?}", gather_acorns(3, 5, 2));
}

// ─── Yew checks the hollow every evening (leave this part alone) ──
#[test]
fn every_day_is_kept_in_order() {
    assert_eq!(gather_acorns(3, 5, 2), vec![3, 5, 2]);
}

#[test]
fn the_hoard_has_one_entry_per_day() {
    assert_eq!(gather_acorns(0, 0, 0).len(), 3);
}
