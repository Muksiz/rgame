// Reference solution — Quest 9: format! mends borrowed halves into an owned String.

fn mend_message(first_half: &str, second_half: &str) -> String {
    format!("{first_half} {second_half}")
}

fn sign_message(message: String) -> String {
    format!("{message} — with love, M.")
}

fn the_whole_letter() -> String {
    let mended = mend_message("meet me", "where the river sings");
    sign_message(mended)
}

fn main() {
    println!("{}", the_whole_letter());
}

#[test]
fn the_halves_are_mended() {
    assert_eq!(
        mend_message("meet me", "where the river sings"),
        "meet me where the river sings"
    );
}

#[test]
fn the_letter_is_signed() {
    assert!(the_whole_letter().ends_with("— with love, M."));
}
