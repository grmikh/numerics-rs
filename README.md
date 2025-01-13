# numerics-rs

[![Crates.io](https://img.shields.io/crates/v/numerics-rs.svg)](https://crates.io/crates/numerics-rs)
[![Docs.rs](https://docs.rs/numerics-rs/badge.svg)](https://docs.rs/numerics-rs)
[![CI](https://github.com/grmikh/numerics-rs/workflows/CI/badge.svg)](https://github.com/grmikh/numerics-rs/actions)

# About

In order to learn Rust, I decided to create a project inspired by the most common problems I have encountered working in
finanial engineering. It is written in pure Rust and I strive for it to be the fastest general-purpose implementation.
Currently supports:

- Interpolation (Univariate, spline polynomials up to an order of 3)
- Numerical solving (Bisection method, Newton-Raphson method, Secant method, Brent method [WIP])

## Usage

To use `numerics-rs`, first add this to your `Cargo.toml`:

```toml
[dependencies]
numerics-rs = "1.0.2"
```

And add this to your crate:

```rust
use numerics_rs::interp::{InterpolationType, Interpolator, ExtrapolationStrategy};

fn main() {
    // ...
}
```

## Supported modes

### Numerical solving

Currently, supports only Newton-Raphson, Secant and Bisection methods. It uses a Builder pattern to construct the
solver. One of the coolest features is convergence logging - this
allows you to experiment and see how well the algorithm behaves on your data

#### Supported modes

1. **Newton-Raphson method**
   The Newton-Raphson method is a numerical algorithm for finding solutions to functions of the form `f(x) = 0`. It
   iteratively converges to the root of the function using an initial guess. The method requires the derivative `f'(x)`
   of the function. At its core, it uses the formula:
   ```
   x_{n+1} = x_n - f(x_n) / f'(x_n)
   ```
   Each iteration refines the estimate for the root of the function. Newton-Raphson is fast and efficient for
   well-behaved functions but requires you to provide the derivative of the function

   #### Example
   Here's an example of using the Newton-Raphson method to find the root of a quadratic function:

   ```rust
   use numerics_rs::solver::newton_raphson;

   fn main() {
       let function = |x: f64| x.powi(3) - x - 2.0; // f(x) = x³ - x - 2
        let derivative = |x: f64| 3.0 * x.powi(2) - 1.0; // f'(x) = 3x² - 1

        // Create the builder and configure it for Newton-Raphson
        let builder = RootFinderBuilder::new(RootFindingMethod::NewtonRaphson)
            .function(&function)
            .derivative(&derivative)
            .initial_guess(400.0) // Terrible initial guess on purpose
            .tolerance(1e-6) // Convergence tolerance
            .max_iterations(100) // Maximum iterations
            .log_convergence(true); // Enable logging

        // Build the Newton-Raphson root finder
        let mut root_finder = builder.build().expect("Failed to build RootFinder");

        let res = root_finder.find_root();
        assert!((res.unwrap() - 1.5213797).abs() < 1e-6);
   }
   ```

   In the above example:
    - `function` is the target function `f(x)`.
    - `derivative` provides the derivative `f'(x)`.
    - `initial_guess` is the starting point for the iterations.
    - The method will terminate either when the result meets the tolerance (`1e-6`) or after the maximum number of
      iterations (`100`).

2. **Secant method**
   Unlike the Newton-Raphson method, the Secant Method does not require the explicit calculation of the derivative of
   the function, making it suitable for cases where the derivative is difficult or expensive to compute.
   Instead of using the derivative, the Secant Method employs a finite difference approximation based on two initial
   guesses that are close to the root. These two points are used to form a line (secant), and the root of this line is
   used as the next approximation.

3. **Bisection method**
   The Bisection Method is a simple and robust numerical technique for finding a root of a continuous function. It is
   classified as a bracketing method, meaning it works by narrowing down an interval where the root is located. This
   method requires that the function changes sign over the interval, ensuring the presence of a root due to the
   Intermediate Value Theorem.

### Interpolation

#### Interpolation Types

1. **Linear Interpolation**
    - Provides a straight-line transition between two points.

2. **Quadratic Interpolation**
    - Fits a quadratic function to each segment of the data.
    - NB! Please note that quadratic splines are very seldom used in practice and I certainly won't recommend anyone
      using it as it doesn't preserve the shape well, the best explanation I found is
      here: https://math.stackexchange.com/questions/4291501/why-it-is-not-possible-to-use-quadratic-spline

3. **Cubic Interpolation**
    - Fits a cubic function to each segment of the data.
    - Delivers smooth transitions by adjusting for changes in curvature and slopes. It is using a 'Natural end
      condition'.
    - This is the best description I found: https://blog.timodenk.com/cubic-spline-interpolation/

4. **Constant (Stepwise) Interpolation**
    - Maintains a constant value between intervals, resulting in a step-like transition.
    - Supported modes:
        - **Constant Forward**: Uses the value of the next point in the interval.
        - **Constant Backward**: Uses the value of the previous point in the interval.

#### Extrapolation

For inputs outside the range of the provided data, the library supports various extrapolation methods:

1. **Linear Extrapolation**
    - Extends the trend of the data linearly based on the slope of the boundary points.

2. **Constant Extrapolation**
    - Maintains a constant value beyond the known points.
    - Similar to using a specific boundary value for all out-of-range inputs.

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

It is very fast and lightweight, it is using precomputed coefficients and it scales really well if you want to call it
many times using the same set of knots. Benchmarks will be added in future versions

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

