# numerics-rs

[![Crates.io](https://img.shields.io/crates/v/numerics-rs.svg)](https://crates.io/crates/numerics-rs)
[![Docs.rs](https://docs.rs/numerics-rs/badge.svg)](https://docs.rs/numerics-rs)
[![CI](https://github.com/grmikh/numerics-rs/workflows/CI/badge.svg)](https://github.com/grmikh/numerics-rs/actions)

# About
In order to learn Rust, I decided to create a project inspired by the most common problems I have encountered working in finanial engineering. It is written in pure Rust and I strive for it to be the fastest existing implementation.
Currently supports: 
- Interpolation (Univariate, spline polynomials up to an order of 3)
- Numerical solving (WIP)
- Convenient matrix API and common matrix operations (WIP)

## Usage

To use, add this to your crate:

```rust
use numerics_rs::interp::{InterpolationType, Interpolator, ExtrapolationStrategy};

fn main() {
    // ...
}
```

## Examples

Linear interpolation:

```rust
        let x_values = vec![0.0, 1.0, 2.0, 3.0];
        let y_values = vec![0.0, 2.0, 4.0, 6.0];

        // Create an interpolator
        let interpolator = Interpolator::new(x_values, y_values, InterpolationType::Linear, ExtrapolationStrategy::None);

        // Test linear interpolation at known points
        assert_eq!(interpolator.interpolate(0.0), 0.0);
        assert_eq!(interpolator.interpolate(1.0), 2.0);
        assert_eq!(interpolator.interpolate(2.0), 4.0);
        assert_eq!(interpolator.interpolate(3.0), 6.0);

        // Test linear interpolation between the points
        assert_eq!(interpolator.interpolate(0.5), 1.0);
        assert_eq!(interpolator.interpolate(1.5), 3.0);
        assert_eq!(interpolator.interpolate(2.5), 5.0);
```

## Dependencies
It comes with 0 external dependencies

## Thread safety
Everything in the API is immutable, thus it is safe to use in a multi-threaded environment

## How fast is it? 
It is very fast and lightweight, it is using precomputed coefficients and it scales really well if you want to call it many times using the same set of knots. Benchmarks will be added in future versions

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install numerics-rs`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
## Feedback
This is my first Rust project therefore I greatly appreciate your feedback, feel free to get in touch

