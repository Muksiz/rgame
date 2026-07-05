// Reference solution — Quest 16: field init shorthand in the issuing, and a
// `mut` binding so the stamp can go on.

struct Token {
    number: u32,
    holder: String,
    stamped: bool,
}

fn issue(number: u32, holder: String) -> Token {
    Token {
        number,
        holder,
        stamped: false,
    }
}

fn crossing_paperwork() -> Token {
    let mut token = issue(7, String::from("the rune-smith"));
    token.stamped = true;
    token
}

fn main() {
    let token = crossing_paperwork();
    println!(
        "Token No. {} — held by {}, {}.",
        token.number,
        token.holder,
        if token.stamped { "stamped and seaworthy" } else { "NOT VALID" }
    );
}

#[test]
fn the_token_is_issued_correctly() {
    let token = issue(7, String::from("the rune-smith"));
    assert_eq!(token.number, 7);
    assert_eq!(token.holder, "the rune-smith");
    assert!(!token.stamped, "fresh tokens are unstamped until inspection");
}

#[test]
fn the_crossing_token_is_stamped() {
    let token = crossing_paperwork();
    assert_eq!(token.number, 7);
    assert!(token.stamped);
}
