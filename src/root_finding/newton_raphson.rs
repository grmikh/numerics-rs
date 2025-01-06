use crate::root_finding::RootFinder;

pub(super) struct NewtonRaphsonRootFinder {
    pub(super) x0: f64,        // Initial guess for the root
    pub(super) tolerance: f64, // Tolerance for the convergence
}
#[allow(clippy::needless_lifetimes)] // Clippy seems to have a bug here
impl RootFinder for NewtonRaphsonRootFinder {
    fn get_init_args(&mut self) -> Box<[f64]> {
        Box::from([self.x0])
    }

    /// Returns the current argument being evaluated.
    /// Normally called as part of the iteration process.
    fn get_next_args(&mut self, fx: &[f64], dfx: &[f64]) -> Box<[f64]> {
        let fx = fx[0];
        let dfx = dfx[0];
        self.x0 -= fx / dfx;
        Box::from([self.x0])
    }

    /// Stops if we're within tolerance
    fn should_stop(&self, fx: &[f64], dfx: &[f64]) -> Option<Result<f64, String>> {
        // If the difference between consecutive arguments is small enough
        let fx = fx[0];
        let dfx = dfx[0];
        let candidate = self.x0 - fx / dfx;
        if (self.x0 - candidate).abs() < self.tolerance {
            return Some(Ok(candidate)); // Converged to a root
        }
        if dfx.abs() < f64::EPSILON {
            // Avoid division by zero or near-zero derivative.
            return Some(Err("Derivative too close to zero.".to_string()));
        }
        None
    }
}
