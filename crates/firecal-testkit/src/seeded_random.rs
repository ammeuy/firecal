//! A tiny xorshift64* generator: fast, dependency free, and the same on every platform.

/// A deterministic pseudo-random number generator. Not for anything but tests.
#[derive(Debug, Clone)]
pub struct SeededRandom {
    state: u64,
}

/// Used in place of a zero seed, which would make xorshift return zero forever.
const ZERO_SEED_REPLACEMENT: u64 = 0x9E37_79B9_7F4A_7C15;

impl SeededRandom {
    /// Starts a sequence from `seed`. The same seed always gives the same numbers.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                ZERO_SEED_REPLACEMENT
            } else {
                seed
            },
        }
    }

    /// The next 64 random bits.
    pub const fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number in `0..bound`. A bound of zero is treated as one, so the result is always zero.
    /// The tiny modulo bias is irrelevant for tests.
    pub const fn next_below(&mut self, bound: u64) -> u64 {
        self.next_u64() % if bound == 0 { 1 } else { bound }
    }
}

#[cfg(test)]
mod tests;
