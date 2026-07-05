// Reference solution — Quest 17: struct update syntax — one field named, the
// rest carried over with ..rod.

struct Rod {
    owner: String,
    reach: u32,
    sharpness: u32,
}

fn sharpened(rod: Rod) -> Rod {
    Rod {
        sharpness: rod.sharpness + 2,
        ..rod
    }
}

fn lend_and_return() -> Rod {
    let spare = Rod {
        owner: String::from("Juniper"),
        reach: 9,
        sharpness: 3,
    };
    // Two borrowers this week — Juniper's rule, applied twice.
    sharpened(sharpened(spare))
}

fn main() {
    let rod = lend_and_return();
    println!(
        "The spare comes home: reach {}, sharpness {}.",
        rod.reach, rod.sharpness
    );
}

#[test]
fn returned_better_than_borrowed() {
    let rod = lend_and_return();
    assert_eq!(rod.sharpness, 7, "3, sharpened twice by 2, should be 7");
}

#[test]
fn nothing_else_about_the_rod_changes() {
    let rod = lend_and_return();
    assert_eq!(rod.reach, 9, "NINE paces. Not two. Juniper is watching");
    assert_eq!(rod.owner, "Juniper");
}
