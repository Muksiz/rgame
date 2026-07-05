// ══════════════════════════════════════════════════════════════════
//   Quest 21: Waking the Golem              ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   The brass plate reads: 'ADMISSIONS. All returns through the
//   slot. THE SLOT DECIDES. — The Management.' And beneath, in
//   smaller letters: 'a slot is a package-shaped hole.'
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Methods can take parameters after `self`. The judging-rune
//   compares TWO instances — the parcel it's called on, and the
//   slot it's offered:
//
//       fn fits_through(&self, slot: &Package) -> bool
//
//   Borrow the second instance (judging shouldn't swallow the
//   slot), and be strict: STRICTLY narrower and STRICTLY shorter,
//   `<` not `<=`, or the slot stays shut.
//
//   Carve the method into the impl block, then press `c`.
// ──────────────────────────────────────────────────────────────────

struct Package {
    width: u32,
    height: u32,
}

impl Package {
    // TODO: fn fits_through(&self, slot: &Package) -> bool
}

fn main() {
    let slot = Package { width: 30, height: 40 };
    let field_guide = Package { width: 22, height: 28 };
    println!(
        "The Field Guide {}.",
        if field_guide.fits_through(&slot) {
            "slides through — the golem grinds awake: 'ADMISSIONS. WELCOME.'"
        } else {
            "does not fit. The golem snores on"
        }
    );
}

// ─── The Management's acceptance test (leave this part alone) ─────
#[test]
fn the_field_guide_fits() {
    let slot = Package { width: 30, height: 40 };
    let field_guide = Package { width: 22, height: 28 };
    assert!(field_guide.fits_through(&slot));
}

#[test]
fn the_grand_atlas_does_not() {
    let slot = Package { width: 30, height: 40 };
    let atlas = Package { width: 45, height: 60 };
    assert!(!atlas.fits_through(&slot));
}

#[test]
fn a_slot_sized_package_stays_out() {
    // Strictly smaller — a package EXACTLY the slot's size jams. The
    // Management has opinions about jams.
    let slot = Package { width: 30, height: 40 };
    let exact = Package { width: 30, height: 40 };
    assert!(!exact.fits_through(&slot));
}

#[test]
fn width_alone_is_not_enough() {
    let slot = Package { width: 30, height: 40 };
    let tall_and_thin = Package { width: 10, height: 55 };
    assert!(!tall_and_thin.fits_through(&slot));
}
