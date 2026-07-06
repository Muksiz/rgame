// Reference solution — Quest 4: comparisons make the bool, `if` is an
// expression, `else if` carries the elders' half toll.

fn toll_is_waived(age: u32, is_local: bool) -> bool {
    age < 12 || is_local
}

fn toll_owed(age: u32, is_local: bool) -> u32 {
    let coins = if toll_is_waived(age, is_local) {
        0
    } else if age >= 70 {
        2
    } else {
        4
    };
    coins
}

fn main() {
    println!("An 8-year-old owes {} coins.", toll_owed(8, false));
    println!("A 30-year-old trader owes {} coins.", toll_owed(30, false));
}

#[test]
fn children_pass_free() {
    assert!(toll_is_waived(8, false));
    assert_eq!(toll_owed(8, false), 0);
}

#[test]
fn locals_pass_free() {
    assert!(toll_is_waived(30, true));
    assert_eq!(toll_owed(75, true), 0, "a local elder is waived, not halved");
}

#[test]
fn elders_pay_half() {
    assert_eq!(toll_owed(70, false), 2);
    assert_eq!(toll_owed(91, false), 2);
}

#[test]
fn everyone_else_pays_in_full() {
    assert!(!toll_is_waived(30, false));
    assert_eq!(toll_owed(30, false), 4);
}
