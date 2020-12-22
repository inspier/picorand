use core::marker::PhantomData;

pub trait PicoRandRNG {
    type Generates;

    fn rand(&mut self) -> Self::Generates;
}

pub struct WyRand {
    seed: u64,
}

impl WyRand {
    const fn new(seed: u64) -> Self {
        WyRand { seed }
    }
}

impl PicoRandRNG for WyRand {
    type Generates = u64;

    fn rand(&mut self) -> Self::Generates {
        self.seed += 0xE7037ED1A0B428DB;
        let x = (self.seed as u128).wrapping_mul((self.seed ^ 0xE7037ED1A0B428DB) as u128);
        ((x >> 64) ^ x) as u64
    }
}

pub struct RNG<R: PicoRandRNG = WyRand, T=u64> {
    rng: R,
    _marker: PhantomData<T>,
}
