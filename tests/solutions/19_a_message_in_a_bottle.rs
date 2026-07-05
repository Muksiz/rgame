// Reference solution — Quest 19: one derive line, and the archive-rune can
// read the whole record aloud.

#[derive(Debug)]
struct Letter {
    sender: String,
    pieces: u32,
    year: u32,
}

fn todays_delivery() -> Letter {
    Letter {
        sender: String::from("M."),
        pieces: 2, // the river is enthusiastic about delivery
        year: 892,
    }
}

fn archive_line(letter: &Letter) -> String {
    format!("{letter:?}")
}

fn main() {
    let letter = todays_delivery();
    println!("The archive-rune reads: {}", archive_line(&letter));
    println!("And in the airy form:\n{letter:#?}");
}

#[test]
fn the_record_reads_aloud_whole() {
    assert_eq!(
        archive_line(&todays_delivery()),
        "Letter { sender: \"M.\", pieces: 2, year: 892 }"
    );
}

#[test]
fn the_letter_is_from_an_old_friend() {
    let letter = todays_delivery();
    assert_eq!(letter.sender, "M.");
    assert_eq!(letter.pieces, 2);
}
