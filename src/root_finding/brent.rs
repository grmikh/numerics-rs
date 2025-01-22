use crate::root_finding::{ConvergenceLog, RootFindingIterator, F};

// Brent search isn't using the common iterator class due to the fact that it has a very tricky iteration that switches methods
pub(super) struct BrentRootFinder<'a> {
    pub(super) x0: f64,        // Initial guess for the root
    pub(super) x1: f64,        // Initial guess for the root
    pub(super) tolerance: f64, // Tolerance for the convergence

    pub(super) function: &'a F, // The target function f(x)
    pub(super) max_iterations: usize,
    pub(super) log_convergence: bool,
    pub(super) convergence_log: ConvergenceLog,
}

impl<'a> RootFindingIterator<'a> for BrentRootFinder<'a> {
    /// Finds a root for a given function `f` in the interval [x0, x1] using Brent's method.
    fn find_root(&mut self) -> Result<f64, String>
    where
        F: FnMut(f64) -> f64,
    {
        self.convergence_log.reset();
        let mut a = self.x0;
        let mut b = self.x1;
        let mut fa = (self.function)(a);
        let mut fb = (self.function)(b);

        if self.log_convergence {
            self.convergence_log
                .add_entry(0, Box::from(vec![a, b]), Box::from(vec![fa, fb]));
        }

        if fa * fb > 0.0 {
            // If the signs of function values at `a` and `b` are the same, a root is not guaranteed.
            return Err(String::from("F(a) and F(b) must be of opposite signs"));
        }

        // Swap a and b if needed to ensure b is the best guess.
        if fa.abs() < fb.abs() {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut fa, &mut fb);
        }

        let mut c = a;
        let mut fc = fa;
        #[allow(unused_assignments)]
        let mut s = b;

        let mut d = b - a;
        let mut e = d;

        for i in 1..self.max_iterations {
            // Update the root estimate using inverse quadratic interpolation or secant method.
            if fa != fc && fb != fc {
                // Inverse quadratic interpolation
                s = a * fb * fc / ((fa - fb) * (fa - fc))
                    + b * fa * fc / ((fb - fa) * (fb - fc))
                    + c * fa * fb / ((fc - fa) * (fc - fb));
            } else {
                // Secant method
                s = b - fb * (b - a) / (fb - fa);
            }

            // Conditions to accept the new approximation.
            if !((3.0 * a + b) / 4.0 < s && s < b)
                || (e.abs() < self.tolerance || (s - b).abs() < self.tolerance)
            {
                // Fall back to bisection method.
                s = (a + b) / 2.0;
                e = d;
                d = b - a;
            } else {
                d = e;
                e = b - a;
            }

            // Update values.
            a = b;
            fa = fb;
            if (s - b).abs() < self.tolerance || fb.abs() < self.tolerance {
                return Ok(s);
            }

            b = s;
            fb = (self.function)(s);
            if self.log_convergence {
                self.convergence_log
                    .add_entry(i, Box::from(vec![s]), Box::from(vec![fb]));
            }
            if fa * fb < 0.0 {
                c = a;
                fc = fa;
            }

            if fa.abs() < fb.abs() {
                std::mem::swap(&mut a, &mut b);
                std::mem::swap(&mut fa, &mut fb);
            }
        }

        Err(String::from("Failed to converge")) // Return None if the method did not converge within the maximum iterations.
    }

    fn get_convergence_log(&self) -> &ConvergenceLog {
        &self.convergence_log
    }
}
