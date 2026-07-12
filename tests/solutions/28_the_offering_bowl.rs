// Reference solution — Quest 28: Option<T>, built with Some/None and
// answered with a two-armed match.

fn shell_if_low_tide(low_tide: bool) -> Option<String> {
    if low_tide {
        Some(String::from("a pink shell"))
    } else {
        None
    }
}

fn thanks_for(offering: Option<String>) -> String {
    match offering {
        Some(gift) => format!("the shrine thanks the sea for {gift}"),
        None => String::from("the shrine thanks the sea for the quiet"),
    }
}

fn main() {
    println!("{}", thanks_for(shell_if_low_tide(true)));
    println!("{}", thanks_for(shell_if_low_tide(false)));
    println!("Both answers true. The shrine bell approves.");
}

#[test]
fn the_sea_keeps_its_schedule() {
    assert_eq!(
        shell_if_low_tide(true),
        Some(String::from("a pink shell"))
    );
    assert_eq!(shell_if_low_tide(false), None);
}

#[test]
fn thanks_are_given_either_way() {
    assert_eq!(
        thanks_for(Some(String::from("a green pebble"))),
        "the shrine thanks the sea for a green pebble"
    );
    assert_eq!(
        thanks_for(None),
        "the shrine thanks the sea for the quiet"
    );
}
