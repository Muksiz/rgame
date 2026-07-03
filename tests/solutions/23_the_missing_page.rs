// Reference solution — Quest 23: `?` unwraps Ok or returns Err early.

fn find_page(total_pages: u32, page: u32) -> Result<u32, String> {
    if page == 0 || page > total_pages {
        return Err(format!("no page {page} in a {total_pages}-page book"));
    }
    Ok(page)
}

fn read_two_pages(total_pages: u32, first: u32, second: u32) -> Result<u32, String> {
    let a = find_page(total_pages, first)?;
    let b = find_page(total_pages, second)?;
    Ok(a + b)
}

fn main() {
    println!("{:?}", read_two_pages(312, 58, 100));
}

#[test]
fn two_real_pages_add_up() {
    assert_eq!(read_two_pages(312, 58, 100), Ok(158));
}

#[test]
fn a_missing_page_stops_the_reading() {
    assert!(read_two_pages(312, 400, 1).is_err());
}
