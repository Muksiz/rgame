// Reference solution — Quest 12: the echo-rune borrows mutably at both ends.

fn add_echo(call: &mut String) {
    call.push_str(" baa");
}

fn settle_the_sheep() -> String {
    let mut call = String::from("baa");
    add_echo(&mut call);
    add_echo(&mut call);
    call
}

fn main() {
    println!("The cave answers: {}", settle_the_sheep());
}

#[test]
fn a_sleepy_triple() {
    assert_eq!(settle_the_sheep(), "baa baa baa");
}
