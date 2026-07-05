// ══════════════════════════════════════════════════════════════════
//   Quest 15: The Dock Ledger                   ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Dockhand Fenn: "Every crate that lands gets a record: what it
//   is, what it weighs, whether the seal's unbroken. Three facts,
//   ONE crate — and my ledger keeps them on three different pages.
//   One under a teacup, usually."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A *struct* bundles named facts into one shape:
//
//       struct Manifest {
//           label: String,
//           weight: u32,
//           sealed: bool,
//       }
//
//   1. Define `Manifest` with exactly those three fields.
//   2. Build one in `receive_crate`: label "Crate 14, dry goods",
//      weight 12, sealed true. (A String field wants
//      `String::from(...)` — a bare literal is only borrowed.)
//
//   The ledger-line below already reads fields with a dot —
//   `entry.label`, `entry.weight` — that part works the moment the
//   bundle exists. Press `c` in the game when it does.
// ──────────────────────────────────────────────────────────────────

// TODO: define the `Manifest` struct here.

fn receive_crate() -> Manifest {
    // TODO: build the record for the crate that just landed.
    todo!("Crate 14, dry goods / 12 stone / seal unbroken")
}

fn ledger_line(entry: &Manifest) -> String {
    format!(
        "{} — {} stone, seal {}",
        entry.label,
        entry.weight,
        if entry.sealed { "unbroken" } else { "BROKEN" }
    )
}

fn main() {
    let crate_record = receive_crate();
    println!("{}", ledger_line(&crate_record));
}

// ─── Fenn checks the ledger at every tide (leave this part alone) ─
#[test]
fn the_record_holds_all_three_facts() {
    let entry = receive_crate();
    assert_eq!(entry.label, "Crate 14, dry goods");
    assert_eq!(entry.weight, 12);
    assert!(entry.sealed);
}

#[test]
fn the_ledger_line_reads_true() {
    assert_eq!(
        ledger_line(&receive_crate()),
        "Crate 14, dry goods — 12 stone, seal unbroken"
    );
}
