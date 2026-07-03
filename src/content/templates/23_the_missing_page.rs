// ══════════════════════════════════════════════════════════════════
//   Quest 23: The Missing Page                   ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Scribe Faye: "One last rune before you rest, rune-smith. Some
//   books are missing pages — the Field Guide, mercifully, is not
//   one of them. I need a rune that reads two pages and, if either
//   one doesn't exist, says so plainly instead of guessing."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `find_page` already returns a `Result<u32, String>` — `Ok(page)`
//   if it exists, `Err(reason)` if it doesn't. The `?` operator
//   unwraps an `Ok` for you, or ends the function early by handing
//   the `Err` straight back to *your* caller:
//
//       let page = find_page(total_pages, wanted)?;
//
//   Fix `read_two_pages` so it uses `?` on both lookups, then press
//   `c` in the game. Your journey ends here.
// ──────────────────────────────────────────────────────────────────

fn find_page(total_pages: u32, page: u32) -> Result<u32, String> {
    if page == 0 || page > total_pages {
        return Err(format!("no page {page} in a {total_pages}-page book"));
    }
    Ok(page)
}

fn read_two_pages(total_pages: u32, first: u32, second: u32) -> Result<u32, String> {
    // TODO: use `?` on each find_page call instead of holding the raw Result
    let a = find_page(total_pages, first);
    let b = find_page(total_pages, second);
    Ok(a + b)
}

fn main() {
    println!("{:?}", read_two_pages(312, 58, 100));
}

// ─── Faye checks the catalogue against the shelf (leave alone) ────
#[test]
fn two_real_pages_add_up() {
    assert_eq!(read_two_pages(312, 58, 100), Ok(158));
}

#[test]
fn a_missing_page_stops_the_reading() {
    assert!(read_two_pages(312, 400, 1).is_err());
}
