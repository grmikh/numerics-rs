use crate::root_finding::RootFinder;

pub(super) struct BisectionRootFinder {
    pub(super) x0: f64,        // Initial guess for the root
    pub(super) x1: f64,        // Initial guess for the root
    pub(super) tolerance: f64, // Tolerance for the convergence
    pub(super) search_left: bool,
}

// This implementation is a bit complex to accommodate for the common iterator interface
impl RootFinder for BisectionRootFinder {
    fn get_init_args(&mut self) -> Box<[f64]> {
        let mid = (self.x0 + self.x1) / 2.0;
        self.x1 = mid;
        Box::from([self.x0, self.x1])
    }
    fn get_next_args(&mut self, fx: &[f64], _dfx: &[f64]) -> Box<[f64]> {
        let [fx0, fx1]: [_; 2] = fx.try_into().unwrap();
        if fx0 * fx1 < 0.0 {
            let mid = (self.x0 + self.x1) / 2.0;
            self.x1 = mid;
            self.search_left = true;
        } else {
            self.x1 = self.x1 * 2.0 - self.x0;
            let mid = (self.x0 + self.x1) / 2.0;
            self.x0 = mid;
            self.search_left = false;
        }
        Box::from([self.x0, self.x1])
    }

    fn should_stop(&self, fx: &[f64], _dfx: &[f64]) -> Option<Result<f64, String>> {
        let [fx0, fx1]: [_; 2] = fx.try_into().unwrap();
        let fxmid = if self.search_left { &fx1 } else { &fx0 };
        let mid = (self.x0 + self.x1) / 2.0;
        if fxmid.abs() < self.tolerance || (self.x1 - self.x0) < self.tolerance {
            return Some(Ok(mid)); // Converged to a root
        }
        None
    }
}
