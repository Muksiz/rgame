// Reference solution — Quest 15: one struct definition, one instance, three
// facts traveling as one.

struct Manifest {
    label: String,
    weight: u32,
    sealed: bool,
}

fn receive_crate() -> Manifest {
    Manifest {
        label: String::from("Crate 14, dry goods"),
        weight: 12,
        sealed: true,
    }
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
