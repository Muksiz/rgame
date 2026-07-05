// Reference solution — Quest 7: a tuple, a destructuring, a `for` over an
// array, and a `while` with the survey chain.

fn well_pin() -> (i32, i32) {
    (12, 4)
}

fn pin_code(pin: (i32, i32)) -> i32 {
    let (col, row) = pin;
    col * 100 + row
}

fn road_length(legs: [i32; 4]) -> i32 {
    let mut total = 0;
    for leg in legs {
        total += leg;
    }
    total
}

fn chain_throws(distance: i32) -> i32 {
    let mut covered = 0;
    let mut throws = 0;
    while covered < distance {
        covered += 5;
        throws += 1;
    }
    throws
}

fn main() {
    println!("The well is filed under {}.", pin_code(well_pin()));
    println!("The east road runs {} paces.", road_length([3, 7, 2, 5]));
    println!("Survey says: {} throws of the chain.", chain_throws(23));
}

#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_pin_files_correctly() {
    assert_eq!(pin_code((12, 4)), 1204);
    assert_eq!(pin_code((3, 9)), 309);
}

#[test]
fn the_road_sums_true() {
    assert_eq!(road_length([3, 7, 2, 5]), 17);
    assert_eq!(road_length([1, 1, 1, 1]), 4);
}

#[test]
fn the_chain_count_lands_exact() {
    assert_eq!(chain_throws(23), 5, "four throws cover 20; a fifth finishes 23");
    assert_eq!(chain_throws(20), 4, "20 paces is exactly four throws");
    assert_eq!(chain_throws(0), 0, "no distance, no throwing");
}
