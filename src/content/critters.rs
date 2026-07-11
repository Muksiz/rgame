//! Words (loosely speaking) from the small lives of the world. Every critter
//! answers a friendly `e`: most just make their noise, some are unaccountably
//! funny, and one or two have clearly been thinking about things for a very
//! long time.

/// What a chicken has to say. Ordinary clucks are deliberately common —
/// profundity is rare, which is what makes it profound.
pub const CHICKEN_TALK: &[&str] = &[
    "Cluck.",
    "Cluck cluck.",
    "Bwok?",
    "Bwok bwok bwok. Bwok.",
    "The hen regards you with one eye, then the other. Whatever she concludes, she keeps it to herself.",
    "\"Cluck,\" she declares, with the gravity of a toll-keeper reading the board aloud.",
    "She stares past you at the horizon, the way one watches a debt that hasn't been repaid.",
    "\"Bwok,\" the hen says — and somehow it lands like 'the egg you seek is already in your hand.'",
    "\"Every dawn gets announced,\" she seems to say, \"whether or not anyone asked for it. That is the job.\"",
    "A long, thoughtful pause. \"The fence,\" she seems to conclude, \"is only a suggestion the world makes.\"",
    "She scratches the dirt twice, considers the result, and abandons the project with visible dignity.",
    "\"Cluuuck,\" she offers, in the exact tone of someone agreeing with a point you haven't made yet.",
];

/// The sheep of the east meadows. Sheep opinions are few but firmly held.
pub const SHEEP_TALK: &[&str] = &[
    "Baa.",
    "Baaaa.",
    "The sheep chews. You wait. The sheep continues to chew. It was never a conversation.",
    "\"Baa,\" she says, in the settled tone of someone who has read the terms and accepts them.",
    "She looks at the grass, then at you, then at the grass — plainly ranking the two of you.",
    "A slow blink. Wherever sheep go when they think, she is there now, and it is peaceful.",
];

/// A frog of the Silverford banks. Mostly heckling.
pub const FROG_TALK: &[&str] = &[
    "Rrrp.",
    "The frog inflates slightly, holds it, and lets it go. A statement was made.",
    "\"Rrrp,\" the frog says — flat, factual, like a stamp coming down on a ledger.",
    "It watches a fly you cannot see with the patience of a compiler watching your semicolons.",
];

/// A moth of the mountain road. The moth is busy.
pub const MOTH_TALK: &[&str] = &[
    "The moth pays you no mind. The moth has a lamp to think about.",
    "It settles on your sleeve for exactly one breath — high praise, in moth.",
    "The moth circles twice and lands where it started. You understand it completely.",
    "It flutters up toward the light like a question toward an answer. Neither will land.",
];

/// The village cat. She was here before the village, ask anyone.
pub const CAT_TALK: &[&str] = &[
    "Mrr.",
    "The cat allows the top of her head to be admired. Touching it is not on offer.",
    "She half-closes her eyes at you: the deepest courtesy a cat extends in public.",
    "\"Mrow,\" she says, which this time clearly means someone here has fish and it isn't you.",
    "The cat inspects your boots and files a report you will never be permitted to read.",
    "She yawns hugely — every tooth accounted for — then looks embarrassed about nothing at all.",
    "The cat is watching a spot on the wall with tremendous purpose. There is nothing on the wall.",
    "A slow tail-curl. You have been catalogued, and against every precedent, approved of.",
];

/// The village dog. The village dog is a very good dog.
pub const DOG_TALK: &[&str] = &[
    "Woof!",
    "The dog's whole back half wags. You are, apparently, the best thing since breakfast.",
    "He brings you a stick. It is, he is confident, the finest stick the village has produced.",
    "The dog sits without being asked and looks enormously proud of both of you.",
    "A low, happy huff. He walked you all the way here, in his opinion, and it went well.",
    "He looks from you to the road east and back, tail going. Adventure smells like more walks.",
    "The dog rolls over. This is not a trick; it's a standing offer, and today you're worthy.",
];

/// A wild boar of the deep woods. Not unfriendly. Not friendly. A boar.
pub const BOAR_TALK: &[&str] = &[
    "Snrf.",
    "The boar keeps rooting. Whatever is under that leaf-litter matters more than you do.",
    "It lifts its head, decides you are neither acorn nor threat, and files you under 'weather'.",
    "\"Hrmf,\" says the boar, in the tone of a hermit who chose the deep woods for good reasons.",
];

/// A duck of the riverbanks. The river's most senior auditor.
pub const DUCK_TALK: &[&str] = &[
    "Quack.",
    "Quack quack.",
    "The duck fixes you with a stare that has already found three problems with your paperwork.",
    "She preens one wing flat, then the other, then regards the river as a job well supervised.",
    "\"Quack,\" she rules — firmly, finally, with no route of appeal. Court is adjourned.",
    "The duck waddles two steps upstream, as if to say the good water is that way, obviously.",
];

/// The pack donkey of the mountain road. Has seen everything twice.
pub const DONKEY_TALK: &[&str] = &[
    "The donkey breathes out slowly through her nose. Mountains, the breath says. Every year, mountains.",
    "She regards the uphill road with the calm of someone who knows exactly how long it really takes.",
    "A soft ear-flick as you pass: acknowledged, traveler. Keep your load balanced and your steps short.",
    "\"Hrrnh,\" she offers — the sound of experience declining, politely, to be hurried.",
    "She looks at your little satchel, then at her own panniers, and forgives you everything.",
];

/// The line for this critter at this moment (seed from `hash2`, so it's the
/// same answer if you ask the same creature again straight away).
pub fn chicken(seed: u32) -> &'static str {
    pick(CHICKEN_TALK, seed)
}

pub fn sheep(seed: u32) -> &'static str {
    pick(SHEEP_TALK, seed)
}

pub fn frog(seed: u32) -> &'static str {
    pick(FROG_TALK, seed)
}

pub fn moth(seed: u32) -> &'static str {
    pick(MOTH_TALK, seed)
}

pub fn cat(seed: u32) -> &'static str {
    pick(CAT_TALK, seed)
}

pub fn dog(seed: u32) -> &'static str {
    pick(DOG_TALK, seed)
}

pub fn boar(seed: u32) -> &'static str {
    pick(BOAR_TALK, seed)
}

pub fn duck(seed: u32) -> &'static str {
    pick(DUCK_TALK, seed)
}

pub fn donkey(seed: u32) -> &'static str {
    pick(DONKEY_TALK, seed)
}

fn pick(pool: &'static [&'static str], seed: u32) -> &'static str {
    pool[seed as usize % pool.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_seed_draws_a_cluck() {
        for seed in 0..64 {
            assert!(!chicken(seed).is_empty());
        }
        assert_ne!(chicken(0), chicken(4));
    }

    #[test]
    fn every_critter_has_words_for_every_seed() {
        for pool in [
            CHICKEN_TALK,
            SHEEP_TALK,
            FROG_TALK,
            MOTH_TALK,
            CAT_TALK,
            DOG_TALK,
            BOAR_TALK,
            DUCK_TALK,
            DONKEY_TALK,
        ] {
            assert!(pool.len() >= 3, "every critter deserves some range");
            for seed in 0..64 {
                assert!(!pick(pool, seed).is_empty());
            }
        }
    }
}
