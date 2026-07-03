// Reference solution — Quest 22: a match arm for every school, no catch-all needed.

enum School {
    Ember,
    Tide,
    Gale,
    Stone,
}

fn shelf_for(school: School) -> u32 {
    match school {
        School::Ember => 1,
        School::Tide => 2,
        School::Gale => 3,
        School::Stone => 4,
    }
}

fn motto_for(school: School) -> &'static str {
    match school {
        School::Ember => "warm hands, warm heart",
        School::Tide => "everything flows",
        School::Gale => "lightly now",
        School::Stone => "patience, patience",
    }
}

fn main() {
    println!(
        "Shelf {} hums: '{}'",
        shelf_for(School::Ember),
        motto_for(School::Ember)
    );
}

#[test]
fn every_shelf_is_known() {
    assert_eq!(shelf_for(School::Ember), 1);
    assert_eq!(shelf_for(School::Tide), 2);
    assert_eq!(shelf_for(School::Gale), 3);
    assert_eq!(shelf_for(School::Stone), 4);
}

#[test]
fn every_motto_is_remembered() {
    assert_eq!(motto_for(School::Ember), "warm hands, warm heart");
    assert_eq!(motto_for(School::Tide), "everything flows");
    assert_eq!(motto_for(School::Gale), "lightly now");
    assert_eq!(motto_for(School::Stone), "patience, patience");
}
