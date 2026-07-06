//! Words (loosely speaking) from the small lives of the world. The pen's
//! hens answer a friendly `e`: most just cluck, some are unaccountably
//! funny, and one or two have clearly been thinking about things for a
//! very long time.

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

/// The line for this hen at this moment (seed from `hash2`, so it's the
/// same answer if you ask the same hen again straight away).
pub fn chicken(seed: u32) -> &'static str {
    CHICKEN_TALK[seed as usize % CHICKEN_TALK.len()]
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
}
