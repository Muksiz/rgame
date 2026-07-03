// ══════════════════════════════════════════════════════════════════
//   Quest 15: The Dock Ledger                   ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Dockhand Fenn: "Every crate gets logged twice — once coming in,
//   once going out. Simple enough, except my logging-rune only lets
//   me write the manifest down ONCE. Second time, it's just... gone.
//   'Moved', it says. Moved WHERE, is what I'd like to know."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   In Rust every value has exactly one owner, and handing a
//   `String` to a function moves it there for good — the caller
//   can't use it again afterward. The plainest fix: give the
//   function a copy of its own with `.clone()`, so the original
//   manifest survives for the second log.
//
//   Fix it, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn log_manifest(label: String) -> String {
    format!("Logged: {label}")
}

fn check_cargo() -> (String, String) {
    let manifest = String::from("Crate 14, dry goods");

    let checked_in = log_manifest(manifest);
    // TODO: `manifest` was moved into the line above and is gone now —
    // give this second logging its own copy instead.
    let checked_out = log_manifest(manifest);

    (checked_in, checked_out)
}

fn main() {
    let (inn, out) = check_cargo();
    println!("{inn}\n{out}");
}

// ─── Fenn checks both ends of the ledger (leave this part alone) ──
#[test]
fn both_logs_are_written() {
    let (checked_in, checked_out) = check_cargo();
    assert_eq!(checked_in, "Logged: Crate 14, dry goods");
    assert_eq!(checked_out, "Logged: Crate 14, dry goods");
}
