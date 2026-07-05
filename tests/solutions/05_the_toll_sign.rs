// Reference solution — Quest 5: two constants carved in, then whole-number
// division for the slots and `%` for the tea tin.

const TOLL_PRICE: u32 = 4;
const BOARD_SLOTS: u32 = 4;

fn days_takings(wagons: u32) -> u32 {
    wagons * TOLL_PRICE
}

fn coins_per_slot(takings: u32) -> u32 {
    takings / BOARD_SLOTS
}

fn coins_for_the_tea_tin(takings: u32) -> u32 {
    takings % BOARD_SLOTS
}

fn main() {
    let takings = days_takings(5);
    println!("Five wagons: {takings} coins.");
    println!(
        "{} per slot, {} for the tea tin.",
        coins_per_slot(takings),
        coins_for_the_tea_tin(takings)
    );
}

#[test]
fn the_price_is_carved_in() {
    assert_eq!(days_takings(1), 4, "one wagon, four coins — thirty years running");
    assert_eq!(days_takings(5), 20);
}

#[test]
fn the_evening_split_is_even() {
    assert_eq!(coins_per_slot(20), 5, "20 coins across 4 slots");
    assert_eq!(coins_per_slot(21), 5, "whole coins only — no splitting a coin");
}

#[test]
fn the_tea_tin_gets_the_rest() {
    assert_eq!(coins_for_the_tea_tin(20), 0);
    assert_eq!(coins_for_the_tea_tin(21), 1);
    assert_eq!(coins_for_the_tea_tin(23), 3);
}
