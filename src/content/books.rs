//! The Great Library's collection. Every bookshelf tile holds one of these;
//! walking the stacks and pressing `e` reads them in shelf order. Guides,
//! field notes on the language's features, and a proper history — all true
//! things about Rust, told in the Library's own voice.

pub struct Book {
    pub title: &'static str,
    pub pages: &'static [&'static str],
}

pub static BOOKS: [Book; 16] = [
    Book {
        title: "The Rust Programming Language",
        pages: &[
            "The one everyone calls, simply, 'the Book'. Written by Steve Klabnik and Carol Nichols, with the whole Rust community peering helpfully over their shoulders.",
            "It is free to read, kept faithfully up to date, and begins — as all good journeys do — with a small program that says hello to the world. A copy lives at doc.rust-lang.org/book, which is a strange address for a library, but the mist doesn't judge.",
        ],
    },
    Book {
        title: "A History of Rust, Vol. I: A Personal Project",
        pages: &[
            "In 2006, a programmer at Mozilla named Graydon Hoare began a language as a personal side project. He named it after rust fungi — organisms he admired for being distributed, resilient, and thoroughly over-engineered for survival.",
            "The name carried a second joke, of course: rust is what forms on old iron, and this was a language for systems programming — the old iron of computing — that refused to crumble.",
            "Mozilla saw promise and began sponsoring the work in 2009. In 2010 the project was announced to the world, and the world, mostly, said 'a new systems language? good luck.' The fungus, patient as ever, kept growing.",
        ],
    },
    Book {
        title: "A History of Rust, Vol. II: The Road to 1.0",
        pages: &[
            "The first Rust compiler was written in OCaml. In 2011 the compiler learned to compile itself — written in Rust, standing on LLVM's shoulders — and the language began the long, humbling work of deciding what it was.",
            "Early Rust had many things modern Rust does not: garbage-collected pointers with @ sigils, green threads, a typestate system. One by one they were carried out of the house. What remained was the core bargain: memory safety without a garbage collector, enforced by ownership and borrowing.",
            "On the fifteenth of May, 2015, Rust 1.0 shipped with a promise called 'stability without stagnation': code that compiles today keeps compiling, while a new release train leaves every six weeks, forever, like a very punctual ferry.",
        ],
    },
    Book {
        title: "A History of Rust, Vol. III: Out in the World",
        pages: &[
            "Rust's first great proving ground was Servo, an experimental browser engine begun at Mozilla. Pieces of it — most famously the Stylo style engine — sailed into Firefox Quantum in 2017, making a very large, very real program measurably faster and safer.",
            "Developers kept voting it their 'most loved' language in the Stack Overflow survey, year after year after year — 2016 through 2023 and beyond, an almost embarrassing streak.",
            "In February 2021 the Rust Foundation was founded — AWS, Google, Huawei, Microsoft and Mozilla among the founders — giving the language a home of its own. In late 2022, Rust became the second language ever welcomed into the Linux kernel, with version 6.1. Android and Windows carry Rust in their bones now, too.",
            "Not bad, the librarians agree, for a fungus.",
        ],
    },
    Book {
        title: "The Book of Ferris",
        pages: &[
            "Rust's unofficial mascot is a small, cheerful crab named Ferris, drawn by Karen Rustad Tölva. The name is a pun on 'ferrous' — of or containing iron — which is the sort of joke this library shelves under 'excellent'.",
            "Rust programmers call themselves Rustaceans in Ferris's honor. Some carry a Ferris of their own in their satchel, who naps between compile errors and claims, unverifiably, to be a distant cousin of the original.",
        ],
    },
    Book {
        title: "The Little Book of Ownership",
        pages: &[
            "Every value in Rust has exactly one owner. When the owner goes out of scope, the value is tidied away — deterministically, immediately, no garbage collector wandering through at odd hours with a broom.",
            "Hand a value to someone else and it *moves*: the old name becomes an empty glove. This feels strict on the first day, and like a super-power on the thirtieth, when whole categories of bugs simply cannot be written.",
        ],
    },
    Book {
        title: "Borrowing, Politely",
        pages: &[
            "You may lend a value out instead of giving it away: &thing lends a look, &mut thing lends the pen. The rule of the reading room: any number of readers, or exactly one writer — never both at once.",
            "The borrow checker enforces this at compile time. Newcomers describe it as arguing with a librarian; the librarian would like it noted that she has never once lost a book.",
        ],
    },
    Book {
        title: "The Lifetimes Atlas",
        pages: &[
            "Every reference lives precisely as long as the compiler can prove it valid — no longer. The little 'a marks you sometimes see are not decoration; they are map annotations, naming how long a borrow may safely wander.",
            "Thanks to them, a Rust program cannot hold a reference to something that has already been tidied away. Dangling pointers are, in this country, a myth told to frighten C programmers.",
        ],
    },
    Book {
        title: "Traits: A Field Guide to Shared Behavior",
        pages: &[
            "A trait names something types can *do*: Display, Clone, Iterator. Any type may learn a trait by providing an impl, and generic functions may ask only for the doing, not the being — 'anything that can fly', not 'specifically a moth'.",
            "The compiler weaves it all together at build time, so the abstraction costs nothing at run time. The locals call this 'zero-cost', with the smugness of people who have measured.",
        ],
    },
    Book {
        title: "Fearless Concurrency",
        pages: &[
            "Two marker traits, Send and Sync, record which types may cross between threads and which may be shared. They are checked at compile time, like tickets at the ferry.",
            "The consequence is remarkable: data races — the flickering, unreproducible night-gremlins of concurrent programming — are simply not among Rust's possible bugs. Threads still require thought. They no longer require luck.",
        ],
    },
    Book {
        title: "Cargo: A Shipwright's Manual",
        pages: &[
            "Cargo builds your project, runs your tests, fetches your dependencies and never once sighs about it. You describe the vessel in a small manifest called Cargo.toml, and cargo does the shipwrighting.",
            "The crates — Rust's word for packages — come from crates.io, a public harbor of shared libraries, versioned by semver so upgrades arrive as guests rather than burglars.",
        ],
    },
    Book {
        title: "The Edition Almanac",
        pages: &[
            "Every few years Rust publishes an *edition* — 2015, 2018, 2021, 2024 — a bundle of refinements a crate may opt into whenever it pleases.",
            "The old promise holds through all of them: editions interoperate, and code from the first day of 1.0 still compiles. Change arrives like the seasons here — regularly, and without burning the barn down.",
        ],
    },
    Book {
        title: "Errs & Absences",
        pages: &[
            "Rust has no exceptions leaping from towers, and no null lurking in cellars. A fallible function returns Result — Ok or Err, say which — and a missing thing is an Option: Some(treasure) or None.",
            "The ? mark passes errors politely up the road, and match makes you look every possibility in the eye. Failure, handled kindly, stops being frightening. (Local rune-smiths call unexpected failures 'fizzles'. No harm done.)",
        ],
    },
    Book {
        title: "The Rustonomicon",
        pages: &[
            "The dark arts: this book concerns the unsafe keyword, by which a programmer may tell the compiler 'trust me' and take raw pointers into their own hands.",
            "It exists because the borrow checker's rules, though wise, are not omniscient — someone must build the safe foundations the rest stand on. The librarians keep it on a high shelf. Not hidden — Rust hides nothing — merely high, so that reaching for it is always a deliberate act.",
        ],
    },
    Book {
        title: "Clippy & the Art of Tidiness",
        pages: &[
            "Clippy is a collection of several hundred lints — small, opinionated observations about your code, ranging from 'this could be simpler' to 'this is technically fine and morally questionable'.",
            "Its colleague rustfmt lays every line out in the community's shared style, ending all arguments about indentation by the simple method of having the argument once, for everyone, forever. This library has strong feelings about tidiness. Both tools are shelved as 'friends'.",
        ],
    },
    Book {
        title: "Macros, and Other Polite Explosions",
        pages: &[
            "Anything ending in ! — println!, vec!, format! — is a macro: code that writes code before the compiler reads it. The mark is a courtesy, so you always know when the language is about to do something clever on your behalf.",
            "With macro_rules! you may write your own, and with #[derive(...)] a struct can be handed whole abilities — Clone, Debug, and friends — the way a coat is handed to a guest. Used sparingly, they are delightful. Used everywhere, they are a haunted house. This book teaches sparingly.",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_book_is_readable() {
        let mut titles = std::collections::HashSet::new();
        for book in &BOOKS {
            assert!(!book.title.is_empty());
            assert!(titles.insert(book.title), "duplicate title: {}", book.title);
            assert!(!book.pages.is_empty(), "{} has no pages", book.title);
            for page in book.pages {
                assert!(
                    (20..=420).contains(&page.chars().count()),
                    "a page of {} won't fit the dialogue box comfortably",
                    book.title
                );
            }
        }
    }
}
