use crate::root_finding::RootFinder;

pub(super) struct NewtonRaphsonRootFinder<'a> {
    pub(super) function: &'a dyn Fn(f64) -> f64, // The target function f(x)
    pub(super) derivative: &'a dyn Fn(f64) -> f64, // The derivative f'(x)
    pub(super) x0: f64,                          // Initial guess for the root
    pub(super) tolerance: f64,                   // Tolerance for the convergence
    pub(super) max_iterations: usize,            // Maximum number of iterations allowed
    pub(super) log_convergence: bool,            // Whether to log convergence history
    pub(super) fx: f64,
    pub(super) dfx: f64,
}
#[allow(clippy::needless_lifetimes)] // Clippy seems to have a bug here
impl<'a> RootFinder for NewtonRaphsonRootFinder<'a> {
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
    fn get_next_args(&mut self) -> (f64, f64) {
        self.x0 = self.x0 - self.fx / self.dfx;
        (self.x0, self.x0)
    }

    fn get_init_args(&mut self) -> (f64, f64) {
        (self.x0, self.x0)
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
