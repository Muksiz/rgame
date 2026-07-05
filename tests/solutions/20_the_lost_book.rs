// Reference solution — Quest 20: two &self methods in the impl block; the
// record answers for itself.

struct Book {
    title: String,
    pages: u32,
    years_overdue: u32,
}

impl Book {
    fn is_overdue(&self) -> bool {
        self.years_overdue > 0
    }

    fn catalogue_card(&self) -> String {
        format!("{} ({} pages)", self.title, self.pages)
    }
}

fn the_returned_book() -> Book {
    Book {
        title: String::from("A Field Guide to Polite Dragons"),
        pages: 312,
        years_overdue: 58, // no judgement. some judgement.
    }
}

fn main() {
    let book = the_returned_book();
    println!("{}", book.catalogue_card());
    println!(
        "Overdue? {}",
        if book.is_overdue() { "Fifty-eight years. NO JUDGEMENT." } else { "Not at all." }
    );
}

#[test]
fn the_record_answers_for_itself() {
    let book = the_returned_book();
    assert!(book.is_overdue());
    assert_eq!(
        book.catalogue_card(),
        "A Field Guide to Polite Dragons (312 pages)"
    );
}

#[test]
fn a_punctual_book_is_not_accused() {
    let punctual = Book {
        title: String::from("On Time: A Memoir"),
        pages: 41,
        years_overdue: 0,
    };
    assert!(!punctual.is_overdue());
}
