//! Deterministic helpers for tests. A property test draws its cases from [`SeededRandom`], so a failure
//! can be replayed from the seed printed in its message.
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod seeded_random;

pub use seeded_random::SeededRandom;
