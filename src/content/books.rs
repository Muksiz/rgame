//! The Great Library's collection. Every bookshelf tile holds one of these;
//! walking the stacks and pressing `e` reads them in shelf order. Guides,
//! field notes on the language's features, and a proper history — all true
//! things about Rust, told in the Library's own voice.

pub struct Book {
    pub title: &'static str,
    pub pages: &'static [&'static str],
}

pub static BOOKS: [Book; 26] = [
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
    Book {
        title: "The Patterns Almanac",
        pages: &[
            "A pattern is a little picture the compiler tries to match your data against. match, if let, while let, even a plain let — all of them take patterns, and all of them can reach inside a value and give its parts names.",
            "Destructuring, the locals call it: let (a, b) = pair unpacks a tuple, let Point { x, y } = p opens a struct, and Some(n) both tests and unwraps in one gesture. Whole staircases of if-else melt into a single, honest match.",
        ],
    },
    Book {
        title: "Closures: Spells That Remember",
        pages: &[
            "A closure is a small nameless function you can write right where you need it — |x| x + 1 — and unlike a plain function it may capture the world around it, borrowing or taking the variables in scope.",
            "The three trait-siblings Fn, FnMut and FnOnce record how greedily a closure holds what it caught: a gentle reader, a scribbler, or a one-time guest that consumes what it's given. Iterators and threads run on closures the way mills run on water.",
        ],
    },
    Book {
        title: "Boxes, Rc, and the Cell Family",
        pages: &[
            "Sometimes one owner isn't enough, or a value must live on the heap. Box<T> puts a single value in a box with one owner; Rc<T> lets several owners share by counting the hands that hold it, tidying up when the last lets go.",
            "And when a shared thing must still change, RefCell<T> moves the borrow-checking from compile time to run time — you promise to behave, and it politely panics if you don't. Reach for these when the ownership tree wants to be a graph.",
        ],
    },
    Book {
        title: "The Testing Handbook",
        pages: &[
            "Write a function, then write its doubts: a #[test] function that asserts what ought to be true. cargo test finds every one, runs them in parallel, and reports back — green for kept promises, red for broken ones.",
            "Unit tests nestle in a #[cfg(test)] module beside the code; bigger tests live in the tests/ folder, exercising the crate as a stranger would. Even the examples in your documentation are run, so a lying doc-comment cannot hide for long.",
        ],
    },
    Book {
        title: "Slices, Vecs, and the Shape of Data",
        pages: &[
            "A Vec<T> is a growable row of values, owned and heap-kept; an array [T; N] is a fixed row known at compile time. A slice &[T] is a borrowed window onto either — a start and a length, owning nothing.",
            "Because a slice is just a view, the same function can read from a Vec, an array, or part of either, and never needs a copy. &str is simply a slice of text. The librarians consider this tidiness close to a moral virtue.",
        ],
    },
    Book {
        title: "Modules, Crates, and Good Fences",
        pages: &[
            "A crate is one tree of code the compiler builds at once; a module is a room within it, made with mod and opened with use. Everything is private until you say pub — good fences, the saying goes, make good neighbors.",
            "A library crate lends its abilities to others; a binary crate has a main and runs. A workspace gathers many crates under one roof, sharing a lockfile and a target folder, the way a street shares a name.",
        ],
    },
    Book {
        title: "Async, and the Art of Waiting Well",
        pages: &[
            "An async function returns a Future — a promise of a value not ready yet. Nothing happens until something awaits it; then the work proceeds until it must wait, yields the thread to others, and resumes when the world is ready.",
            "This lets a single thread juggle thousands of slow errands — network calls, timers, files — without a thread apiece. A runtime such as tokio or async-std does the juggling. Rust supplies the grammar; you bring the patience.",
        ],
    },
    Book {
        title: "Enums: One of Several Things",
        pages: &[
            "An enum names a value that is exactly one of several shapes: an Direction that is Up or Down or Left or Right, a Shape that is a Circle with a radius or a Rect with two sides. Each variant may carry its own cargo.",
            "Paired with match, an enum is Rust's great honesty engine: the compiler will not rest until every variant is accounted for. Option and Result — the whole language's approach to absence and failure — are, underneath, just two very famous enums.",
        ],
    },
    Book {
        title: "The Iterator's Songbook",
        pages: &[
            "An iterator is anything that can hand you the next thing until there are no more things: one trait, Iterator, one required method, next. Every for loop in the land is quietly asking next, again and again, until it hears None.",
            "The verses are the adaptors — map to transform, filter to choose, zip to walk two lines abreast — each lazy as a summer river, doing nothing until a collect or a sum finally asks for the ending. Chained together they read like the loop you meant, and compile down to one as fast as any you'd write by hand.",
        ],
    },
    Book {
        title: "Strings: An Honest Appraisal",
        pages: &[
            "There are two of them, and the honest news is you need both. String owns its text and can grow; &str is a borrowed view of text living elsewhere. The same split as Vec and slice, wearing letters instead.",
            "Every Rust string is UTF-8, which is why you cannot simply take s[3] and be handed a letter — some letters are one byte, some are four, and the compiler refuses to guess wrong on your behalf. Ask for .chars() or .bytes() and say which counting you mean.",
            "The librarians note, wearily, that more ink has been spilled on 'why can't I index a String' than on any other question in the collection. The answer is on this shelf. It has always been on this shelf.",
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
