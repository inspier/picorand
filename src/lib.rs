#![no_std]

use core::marker::PhantomData;

pub trait PicoRandRNG {
    type Input;
    type Output;

    fn new(seed: Self::Input) -> Self;
    fn rand(&mut self) -> Self::Output;
    fn rand_range(&mut self, min: usize, max: usize) -> Self::Output;
}

pub trait PicoRandGenerate<R: PicoRandRNG, T> {
    fn generate(&mut self) -> R::Output;
}

pub struct WyRand {
    seed: u64,
}

impl PicoRandRNG for WyRand {
    type Input = u64;
    type Output = u64;

    fn new(seed: Self::Input) -> Self {
        WyRand { seed }
    }

    fn rand(&mut self) -> Self::Output {
        self.seed = self.seed.wrapping_add(0xE7037ED1A0B428DB);
        let x = (self.seed as u128).wrapping_mul((self.seed ^ 0xE7037ED1A0B428DB) as u128);
        ((x >> 64) ^ x) as u64
    }

    // Adapted from https://github.com/lemire/FastShuffleExperiments
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

pub struct RNG<R: PicoRandRNG = WyRand, T = u64> {
    rng: R,
    _marker: PhantomData<T>,
}

impl<R: PicoRandRNG, T> RNG<R, T> {
    pub fn new(seed: R::Input) -> Self {
        RNG::<R, T> {
            rng: R::new(seed),
            _marker: PhantomData,
        }
    }

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
