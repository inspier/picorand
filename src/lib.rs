#![no_std]

use core::marker::PhantomData;

/// Requirements for compatible PRNG.
pub trait PicoRandRNG {
    /// Input type for the PRNG.
    type Input;
    /// Output type for the PRNG.
    type Output;

    /// Create a new PRNG instance using a specific seed.
    fn new(seed: Self::Input) -> Self;
    /// Generate a new number using the PRNG.
    fn rand(&mut self) -> Self::Output;
    /// Constrain a randomly generated number to a fixed range.
    fn rand_range(&mut self, min: usize, max: usize) -> Self::Output;
}

/// Requirement for implicitly bounded RNG.
pub trait PicoRandGenerate<R: PicoRandRNG, T> {
    /// Generate a new implicitly bound number using the PRNG.
    fn generate(&mut self) -> R::Output;
}

/// A WyRand PRNG instance. Note: This PRNG is NOT cryptographically secure.
pub struct WyRand {
    seed: u64,
}

impl PicoRandRNG for WyRand {
    /// Input type for the PRNG.
    type Input = u64;
    /// Output type for the PRNG.
    type Output = u64;

    /// Create a new [`WyRand`] instance using a specific seed.
    fn new(seed: Self::Input) -> Self {
        WyRand { seed }
    }

    /// Generate a new number using the [`WyRand`] PRNG.
    fn rand(&mut self) -> Self::Output {
        self.seed = self.seed.wrapping_add(0xE7037ED1A0B428DB);
        let x = (self.seed as u128).wrapping_mul((self.seed ^ 0xE7037ED1A0B428DB) as u128);
        ((x >> 64) ^ x) as u64
    }

    // Adapted from https://github.com/lemire/FastShuffleExperiments
    /// Constrain a randomly generated number to a fixed range.
    fn rand_range(&mut self, min: usize, max: usize) -> Self::Output {
        let t = ((-(max as i64)) % (max as i64)) as u64;
        let (mut x, mut m, mut l);

        while {
            x = self.rand();
            m = (x as u128).wrapping_mul(max as u128);
            l = m as u64;

            l < t
        } {}

        ((m >> 64) as u64).max(min as u64)
    }
}

/// An abstraction over a PRNG with a specific seed.
pub struct RNG<R: PicoRandRNG = WyRand, T = u64> {
    rng: R,
    _marker: PhantomData<T>,
}

impl<R: PicoRandRNG, T> RNG<R, T> {
    /// Create a new [`RNG`] instance using a specific PRNG and a specific seed.
    pub fn new(seed: R::Input) -> Self {
        RNG::<R, T> {
            rng: R::new(seed),
            _marker: PhantomData,
        }
    }

    /// Generate a number in the specified range.
    ///
    /// # Example
    ///
    /// ```
    /// use picorand::{RNG, WyRand};
    /// let mut rng = RNG::<WyRand, u8>::new(0xDEADBEEF);
    /// let generated = rng.generate_range(0xC0, 0xDE);
    /// assert!(generated >= 0xC0 || generated <= 0xDE);
    /// ```
    pub fn generate_range(&mut self, min: usize, max: usize) -> R::Output {
        self.rng.rand_range(min, max)
    }
}

macro_rules! ImplPicoRandCommon {
    (for $($type:tt),+) => {
        $(ImplPicoRandCommon!($type);)*
    };

    ($type:ident) => {
        impl<R: PicoRandRNG> PicoRandGenerate<R, $type> for RNG<R, $type> {
            fn generate(&mut self) -> R::Output {
                self.rng.rand_range($type::MIN as usize, $type::MAX as usize)
            }
        }
    };
}

ImplPicoRandCommon!(for u8, u16, u32, u64);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    macro_rules! ImplPicoRandTest {
        (for $($type:tt),+) => {
            $(ImplPicoRandTest!($type);)*
        };

        ($type:ident) => {
        paste! {
            #[test]
            fn [<test_picorand_generate_ $type>]() {
                let mut rng = RNG::<WyRand, $type>::new(0xDEADBEEF);
                let mut generated: $type;
                for _ in 1..100 {
                generated = rng.generate() as _;
                assert!(generated >= $type::MIN || generated <= $type::MAX);
                }
            }

            #[test]
            fn [<test_picorand_generate_range_ $type>]() {
                let mut rng = RNG::<WyRand, $type>::new(0xDEADBEEF);
                let mut generated: $type;
                for _ in 1..100 {
                generated = rng.generate_range(0xC0, 0xDE) as _;
                assert!(generated >= 0xC0 || generated <= 0xDE);
                }
            }
        }
        };
    }

    ImplPicoRandTest!(for u8, u16, u32, u64);
}
