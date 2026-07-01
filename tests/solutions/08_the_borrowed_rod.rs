// Reference solution — Quest 8: a mutable borrow, and * to reach through it.

fn sharpen(hook: &mut i32) {
    *hook += 2;
}

fn tune_up_the_rod() -> i32 {
    let mut sharpness = 3;
    sharpen(&mut sharpness);
    sharpen(&mut sharpness);
    sharpness
}

fn main() {
    println!("The rod comes back at sharpness {}.", tune_up_the_rod());
}

#[test]
fn returned_better_than_borrowed() {
    assert_eq!(tune_up_the_rod(), 7, "3, sharpened twice by 2, should be 7");
}
