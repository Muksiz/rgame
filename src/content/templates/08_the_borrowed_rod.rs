// ══════════════════════════════════════════════════════════════════
//   Quest 8: The Borrowed Rod              ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Fisher Juniper: "My sharpening-rune runs beautifully — on a
//   COPY. Some ghost-rod in rune-land gets nice and sharp and my
//   actual rod stays dull as a Tuesday."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   To let a function change the caller's value, it must borrow
//   it *mutably*:
//
//       fn sharpen(hook: &mut i32) { *hook += 2; }   // reach through with *
//       sharpen(&mut sharpness);                     // lend it mutably
//
//   Fix the rune and its call sites, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn sharpen(hook: i32) {
    // This only ever sharpens a private copy — the change is lost.
    let _ = hook + 2;
}

fn tune_up_the_rod() -> i32 {
    let mut sharpness = 3;
    sharpen(sharpness);
    sharpen(sharpness);
    sharpness // Juniper expects a 7 after two sharpenings
}

fn main() {
    println!("The rod comes back at sharpness {}.", tune_up_the_rod());
}

// ─── Juniper's one rule (leave this part alone) ───────────────────
#[test]
fn returned_better_than_borrowed() {
    assert_eq!(tune_up_the_rod(), 7, "3, sharpened twice by 2, should be 7");
}
