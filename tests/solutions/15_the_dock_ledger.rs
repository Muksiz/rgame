// Reference solution — Quest 15: .clone() the manifest so the original
// survives its own move.

fn log_manifest(label: String) -> String {
    format!("Logged: {label}")
}

fn check_cargo() -> (String, String) {
    let manifest = String::from("Crate 14, dry goods");

    let checked_in = log_manifest(manifest.clone());
    let checked_out = log_manifest(manifest);

    (checked_in, checked_out)
}

fn main() {
    let (inn, out) = check_cargo();
    println!("{inn}\n{out}");
}

#[test]
fn both_logs_are_written() {
    let (checked_in, checked_out) = check_cargo();
    assert_eq!(checked_in, "Logged: Crate 14, dry goods");
    assert_eq!(checked_out, "Logged: Crate 14, dry goods");
}
