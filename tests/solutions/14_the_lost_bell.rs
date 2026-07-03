// Reference solution — Quest 14: Some when found, None when not.

fn search_for_the_bell(found_today: bool) -> Option<u32> {
    if found_today { Some(7) } else { None }
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
