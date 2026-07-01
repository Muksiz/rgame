// Reference solution — Quest 1: println! is a macro, so it needs the `!`.

fn main() {
    println!("Let there be light!");
}

#[test]
fn the_lantern_listens() {
    main();
}
