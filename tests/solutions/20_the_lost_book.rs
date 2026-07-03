// Reference solution — Quest 20: build the struct value, naming every field.

struct Book {
    title: String,
    pages: u32,
    years_overdue: u32,
}

fn catalogue_the_book() -> Book {
    Book {
        title: "A Field Guide to Polite Dragons".to_string(),
        pages: 312,
        years_overdue: 58,
    }
}

fn main() {
    let book = catalogue_the_book();
    println!("Catalogued: '{}', {} pages.", book.title, book.pages);
}

#[test]
fn the_record_is_complete() {
    let book = catalogue_the_book();
    assert_eq!(book.title, "A Field Guide to Polite Dragons");
    assert_eq!(book.pages, 312);
    assert_eq!(book.years_overdue, 58);
}
