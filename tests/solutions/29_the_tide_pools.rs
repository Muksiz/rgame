// Reference solution — Quest 29: if let for the one shape that matters,
// let-else for the pearl that's required.

fn greet(pool: Option<u32>) -> String {
    if let Some(size) = pool {
        format!("oh, a pearl! {size} grains")
    } else {
        String::from("just patience in this one")
    }
}

fn weigh(pool: Option<u32>) -> u32 {
    let Some(size) = pool else {
        return 0;
    };
    size
}

fn main() {
    println!("{}", greet(Some(9)));
    println!("{}", greet(None));
    println!("Total grains: {}", weigh(Some(9)) + weigh(None));
    println!("Grandmother Brine, at last, looks up.");
}

#[test]
fn the_pearl_is_greeted() {
    assert_eq!(greet(Some(12)), "oh, a pearl! 12 grains");
    assert_eq!(greet(None), "just patience in this one");
}

#[test]
fn the_scales_are_honest() {
    assert_eq!(weigh(Some(31)), 31);
    assert_eq!(weigh(None), 0);
}
