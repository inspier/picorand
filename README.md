[![Current Crates.io Version](https://img.shields.io/crates/v/picorand.svg)](https://crates.io/crates/picorand)
[![docs-rs](https://docs.rs/picorand/badge.svg)](https://docs.rs/picorand)

# picorand
A zero-dependency, no_std-compatible, easily extendable library intended for fast random number generation using the [WyRand](https://github.com/wangyi-fudan/wyhash) PRNG with a pico-sized footprint.

To add to your Cargo.toml:
```toml
picorand = "0.1.0"
```

## Example
```rust
use picorand::{PicoRandGenerate, WyRand, RNG};

fn main() {
    let mut rng = RNG::<WyRand, u16>::new(0xDEADBEEF);

    // Generate in implicit range
    let mut generated = rng.generate();
    assert!(generated >= u16::MIN || generated <= u16::MAX);

    // Generate in explicit range
    generated = rng.generate_range(0xC0, 0xDE);
    assert!(generated >= 0xC0 || generated <= 0xDE);
}
```
