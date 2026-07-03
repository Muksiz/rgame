// Reference solution — Quest 4: a boolean expression, no `if` required.

fn toll_is_waived(age: u32, is_local: bool) -> bool {
    age < 12 || is_local
}

fn main() {
    println!("Waived for an 8-year-old? {}", toll_is_waived(8, false));
}

#[test]
fn children_pass_free() {
    assert!(toll_is_waived(8, false));
}

#[test]
fn locals_pass_free() {
    assert!(toll_is_waived(30, true));
}

#[test]
fn everyone_else_pays() {
    assert!(!toll_is_waived(30, false));
}
