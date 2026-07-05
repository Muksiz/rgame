// Reference solution — Quest 11: the rune borrows with &String, the caller
// lends with & — looking, not keeping.

fn is_dinner(mushroom: &String, spots: u32) -> bool {
    !mushroom.contains("glowing") && spots <= 4
}

fn basket_check() -> String {
    let find = String::from("a plain brown bolete");

    let verdict = is_dinner(&find, 3);

    if verdict {
        format!("{find} — into the basket")
    } else {
        format!("{find} — left for the moths")
    }
}

fn main() {
    println!("{}", basket_check());
}

#[test]
fn the_mushroom_survives_inspection() {
    assert_eq!(basket_check(), "a plain brown bolete — into the basket");
}

#[test]
fn plain_mushrooms_are_dinner() {
    assert!(is_dinner(&String::from("a plain brown bolete"), 0));
    assert!(is_dinner(&String::from("a stout penny-cap"), 4));
}

#[test]
fn spotty_mushrooms_stay_put() {
    assert!(!is_dinner(&String::from("a freckled toadstool"), 5));
    assert!(!is_dinner(&String::from("a freckled toadstool"), 11));
}

#[test]
fn glowing_mushrooms_are_never_ever_dinner() {
    assert!(!is_dinner(&String::from("a glowing violet cap"), 0));
    assert!(!is_dinner(&String::from("a glowing violet cap"), 9));
}
