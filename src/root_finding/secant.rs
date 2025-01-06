use crate::root_finding::RootFinder;

pub(super) struct SecantRootFinder<'a> {
    pub(super) function: &'a dyn Fn(f64) -> f64, // The target function f(x)
    pub(super) x0: f64,                          // Initial guess for the root
    pub(super) x1: f64,                          // Initial guess for the root
    pub(super) x2: f64,                          // Candidate for the next x1
    pub(super) tolerance: f64,                   // Tolerance for the convergence
    pub(super) max_iterations: usize,            // Maximum number of iterations allowed
    pub(super) log_convergence: bool,            // Whether to log convergence history
    pub(super) fx0: f64,
    pub(super) fx1: f64,
}

impl<'a> RootFinder for SecantRootFinder<'a> {
    fn evaluate(&mut self) -> (f64, f64) {
        let f = self.function;
        self.fx0 = f(self.x0);
        self.fx1 = f(self.x1);
        (self.fx0, self.fx1)
    }

    fn get_init_args(&mut self) -> (f64, f64) {
        (self.x0, self.x1)
    }
    fn get_next_args(&mut self) -> (f64, f64) {
        self.x2 = self.x1 - self.fx1 * (self.x1 - self.x0) / (self.fx1 - self.fx0);
        self.x0 = self.x1;
        self.x1 = self.x2;
        (self.x0, self.x1)
    }

    fn should_stop(&self, num_it: &usize) -> Option<Result<f64, String>> {
        if (self.x0 - self.x1).abs() < self.tolerance {
            return Some(Ok(self.x2)); // Converged to a root
        }
        if (self.fx0 - self.fx1).abs() < f64::EPSILON {
            // Avoid division by zero or near-zero
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
