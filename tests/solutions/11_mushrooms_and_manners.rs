// Reference solution — Quest 11: Maren's rules, glow-check first.

fn is_safe_to_eat(spots: u32, glows: bool) -> bool {
    if glows {
        false
    } else if spots > 4 {
        false
    } else {
        true
    }
}

fn main() {
    println!(
        "Plain and three-spotted? {}",
        if is_safe_to_eat(3, false) { "basket!" } else { "leave it." }
    );
}

#[test]
fn plain_mushrooms_are_dinner() {
    assert!(is_safe_to_eat(0, false));
    assert!(is_safe_to_eat(4, false));
}

#[test]
fn spotty_mushrooms_stay_put() {
    assert!(!is_safe_to_eat(5, false));
    assert!(!is_safe_to_eat(11, false));
}

#[test]
fn glowing_mushrooms_are_never_ever_dinner() {
    assert!(!is_safe_to_eat(0, true));
    assert!(!is_safe_to_eat(9, true));
}
