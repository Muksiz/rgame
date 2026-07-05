// Reference solution — Quest 21: a method with a parameter, comparing two
// instances. Strictly smaller, or the slot stays shut.

struct Package {
    width: u32,
    height: u32,
}

impl Package {
    fn fits_through(&self, slot: &Package) -> bool {
        self.width < slot.width && self.height < slot.height
    }
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
