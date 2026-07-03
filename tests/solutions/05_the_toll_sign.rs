// Reference solution — Quest 5: two constants, carved in rather than chalked.

const TOLL_PRICE: u32 = 4;
const TOLL_SLOTS: usize = 4;

fn toll_board() -> [u32; TOLL_SLOTS] {
    [TOLL_PRICE; TOLL_SLOTS]
}

fn main() {
    println!("{:?}", toll_board());
}

#[test]
fn four_wagons_four_coins() {
    assert_eq!(toll_board(), [4, 4, 4, 4]);
}
