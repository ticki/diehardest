//! A new approach to randomness testing.
//!
//! Diehardest is a small library providing strong tools to rate quality of pseudorandom streams.
//! It works with two components:
//!
//! 1. A number of transformations which will weaken weak RNGs.
//! 2. A collection of analytical tools, which rates the transformed streams.
//!
//! In contrast to many other randomness tests, diehardest is stream-aware, making it able to
//! detect many positional patterns that other tests cannot.

mod transform;
pub mod analysis;

/// A random number generator.
pub trait Random {
    /// Get a random number.
    fn get_random(&mut self) -> u64;
}

/// Crush this random number generator.
///
/// This rates it based on analysis of itself and transformations of it.
pub fn crush<R: Random + Clone>(rand: R) -> u32 {
    analysis::Report::new(rand.clone()).get_score().total() as u32
        + analysis::Report::new(transform::SkipOne(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::SkipTwo(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Concatenate32(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Xor(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Add(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Multiply(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::LastBit(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::MultiplyByThree(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::ModularDivideByThree(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Hamming(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::ParitySkip(rand.clone())).get_score().total() as u32
        + analysis::Report::new(transform::Rol7(rand.clone())).get_score().total() as u32
}
