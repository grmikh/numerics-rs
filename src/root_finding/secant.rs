use crate::root_finding::RootFinder;

pub(super) struct SecantRootFinder {
    pub(super) x0: f64,        // Initial guess for the root
    pub(super) x1: f64,        // Initial guess for the root
    pub(super) x2: f64,        // Candidate for the next x1
    pub(super) tolerance: f64, // Tolerance for the convergence
}

impl RootFinder for SecantRootFinder {
    fn get_init_args(&mut self) -> Box<[f64]> {
        Box::from([self.x0, self.x1])
    }
    fn get_next_args(&mut self, fx: &[f64], _dfx: &[f64]) -> Box<[f64]> {
        let [fx0, fx1]: [_; 2] = fx.try_into().unwrap();
        self.x2 = self.x1 - fx1 * (self.x1 - self.x0) / (fx1 - fx0);
        self.x0 = self.x1;
        self.x1 = self.x2;
        Box::from([self.x0, self.x1])
    }

    fn should_stop(&self, fx: &[f64], _dfx: &[f64]) -> Option<Result<f64, String>> {
        let [fx0, fx1]: [_; 2] = fx.try_into().unwrap();
        if (self.x0 - self.x1).abs() < self.tolerance {
            return Some(Ok(self.x2)); // Converged to a root
        }
        if (fx0 - fx1).abs() < f64::EPSILON {
            // Avoid division by zero or near-zero
            return Some(Err("Derivative too close to zero.".to_string()));
        }
        None
    }
}
