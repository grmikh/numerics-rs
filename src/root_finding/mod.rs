use std::fmt::Debug;

mod builder;
pub use builder::RootFinderBuilder;

#[derive(Debug)]
pub enum RootFindingMethod {
    Bisection,
    Brent,
    Secant,
    InverseQuadraticInterpolation,
    NewtonRaphson,
}

pub trait RootFinder {
    type TArgs: Debug + Copy;
    type TRes: Debug + Copy;
    /// Executes the root-finding process based on the specified method.
    fn find_root(&mut self) -> Result<f64, String> {
        let mut num_it: usize = 1;
        loop {
            let args = self.get_next_args();
            let vals = self.evaluate();
            //TODO: Add time logging as well
            if self.log_convergence() {
                println!("Iteration {num_it}, args = {:#?}, vals = {:#?}", args, vals);
            }
            let should_stop: Option<Result<f64, String>> = self.should_stop(&num_it);
            if let Some(res) = should_stop {
                return res;
            }
            num_it += 1;
        }
    }
    fn evaluate(&mut self) -> Self::TRes;
    fn get_next_args(&mut self) -> Self::TArgs;
    fn should_stop(&self, num_it: &usize) -> Option<Result<f64, String>>;
    fn log_convergence(&self) -> bool;
}

pub struct NewtonRaphsonRootFinder<'a> {
    function: &'a dyn Fn(f64) -> f64,   // The target function f(x)
    derivative: &'a dyn Fn(f64) -> f64, // The derivative f'(x)
    x0: f64,                            // Initial guess for the root
    tolerance: f64,                     // Tolerance for the convergence
    max_iterations: usize,              // Maximum number of iterations allowed
    log_convergence: bool,              // Whether to log convergence history
    fx: f64,
    dfx: f64,
}
#[allow(clippy::needless_lifetimes)] // Clippy seems to have a bug here
impl<'a> RootFinder for NewtonRaphsonRootFinder<'a> {
    type TArgs = f64;
    type TRes = (f64, f64);

    /// Evaluates the function and its derivative at the given point, adjusted for the target.
    fn evaluate(&mut self) -> (f64, f64) {
        let f = self.function;
        let df = self.derivative;
        self.fx = f(self.x0);
        self.dfx = df(self.x0);
        (self.fx, self.dfx)
    }

    /// Returns the current argument being evaluated.
    /// Normally called as part of the iteration process.
    fn get_next_args(&mut self) -> f64 {
        self.x0 = if self.fx.is_nan() && self.dfx.is_nan() {
            self.x0
        } else {
            self.x0 - self.fx / self.dfx
        };
        self.x0
    }

    /// Stops if we're within tolerance or exceed max iterations.
    fn should_stop(&self, num_it: &usize) -> Option<Result<f64, String>> {
        // If the difference between consecutive arguments is small enough
        let candidate = self.x0 - self.fx / self.dfx;
        if (self.x0 - candidate).abs() < self.tolerance {
            return Some(Ok(candidate)); // Converged to a root
        }
        if self.dfx.abs() < f64::EPSILON {
            // Avoid division by zero or near-zero derivative.
            return Some(Err("Derivative too close to zero.".to_string()));
        }
        // If the number of iterations exceeds the maximum allowed
        if *num_it >= self.max_iterations {
            return Some(Err(
                "Maximum iterations reached without convergence.".to_string()
            ));
        }
        None
    }

    fn log_convergence(&self) -> bool {
        self.log_convergence
    }
}
