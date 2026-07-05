// ══════════════════════════════════════════════════════════════════
//   Quest 17: The Borrowed Rod              ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Fisher Juniper: "One rule on the spare rod: return it BETTER
//   than you got it — one sharpening's worth. My returning-rune
//   copies every field across by hand, and last week it turned my
//   nine-pace rod into a two-pace rod. TWO PACES."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   *Struct update syntax* builds a new instance that is mostly an
//   old one: name only the fields that change, then `..old` — "and
//   the rest, from this one" — which must come last:
//
//       Rod { sharpness: rod.sharpness + 2, ..rod }
//
//   Every unnamed field (reach, owner) carries over exactly. No
//   hand-copying, no two-pace rods.
//
//   Fix `sharpened`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct Rod {
    owner: String,
    reach: u32,
    sharpness: u32,
}

fn sharpened(rod: Rod) -> Rod {
    // TODO: the SAME rod, sharpness two better, nothing else touched —
    // struct update syntax is the tidy spell for it.
    todo!()
}

fn lend_and_return() -> Rod {
    let spare = Rod {
        owner: String::from("Juniper"),
        reach: 9,
        sharpness: 3,
    };
    // Two borrowers this week — Juniper's rule, applied twice.
    sharpened(sharpened(spare))
}

fn main() {
    let rod = lend_and_return();
    println!(
        "The spare comes home: reach {}, sharpness {}.",
        rod.reach, rod.sharpness
    );
}

// ─── Juniper's one rule (leave this part alone) ───────────────────
#[test]
fn returned_better_than_borrowed() {
    let rod = lend_and_return();
    assert_eq!(rod.sharpness, 7, "3, sharpened twice by 2, should be 7");
}

#[test]
fn nothing_else_about_the_rod_changes() {
    let rod = lend_and_return();
    assert_eq!(rod.reach, 9, "NINE paces. Not two. Juniper is watching");
    assert_eq!(rod.owner, "Juniper");
}
