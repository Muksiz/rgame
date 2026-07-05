// Reference solution — Quest 23: an associated function makes the page from
// nothing; a &mut self method fills it. The capstone, standing together.

struct CataloguePage {
    number: u32,
    entries: u32,
}

impl CataloguePage {
    fn new(number: u32) -> Self {
        Self { number, entries: 0 }
    }

    fn record_entry(&mut self) {
        self.entries += 1;
    }

    fn is_filled(&self) -> bool {
        self.entries >= 3
    }
}

fn rewrite_the_missing_page() -> CataloguePage {
    let mut page = CataloguePage::new(58);
    page.record_entry(); // A Field Guide to Polite Dragons — returned
    page.record_entry(); // On Time: A Memoir — renewed
    page.record_entry(); // The River Delivers: Collected Letters — new
    page
}

fn main() {
    let page = rewrite_the_missing_page();
    println!(
        "Page {} rewritten: {} entries. {}",
        page.number,
        page.entries,
        if page.is_filled() { "The catalogue is whole." } else { "Still a gap..." }
    );
}

#[test]
fn the_page_is_made_fresh_by_the_type() {
    let page = CataloguePage::new(58);
    assert_eq!(page.number, 58);
    assert_eq!(page.entries, 0, "a fresh page starts empty");
    assert!(!page.is_filled());
}

#[test]
fn the_missing_page_is_made_whole() {
    let page = rewrite_the_missing_page();
    assert_eq!(page.number, 58);
    assert_eq!(page.entries, 3);
    assert!(page.is_filled());
}
