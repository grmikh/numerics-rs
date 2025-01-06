use super::*;
/// Builder pattern for RootFinder configuration.
pub struct RootFinderBuilder<'a> {
    method: RootFindingMethod,
    initial_guess: Option<f64>,
    boundaries: Option<(f64, f64)>,
    tolerance: Option<f64>,
    max_iterations: Option<usize>,
    log_convergence: Option<bool>,
    function: Option<&'a dyn Fn(f64) -> f64>, // Target function
    derivative: Option<&'a dyn Fn(f64) -> f64>, // Derivative of the target function
}

impl<'a> RootFinderBuilder<'a> {
    /// Creates a new instance of `RootFinderBuilder`.
    pub fn new(method: RootFindingMethod) -> Self {
        Self {
            method,
            initial_guess: None,
            boundaries: None,
            tolerance: None,
            max_iterations: None,
            log_convergence: None,
            function: None,
            derivative: None,
        }
    }

    /// Sets the initial guess for methods that require one (e.g., Newton-Raphson).
    pub fn initial_guess(mut self, guess: f64) -> Self {
        self.initial_guess = Some(guess);
        self
    }

    /// Sets the boundaries for methods that require bounded intervals (e.g., Bisection).
    pub fn boundaries(mut self, x0: f64, x1: f64) -> Self {
        self.boundaries = Some((x0, x1));
        self
    }

    /// Sets the tolerance for the root-finding process.
    pub fn tolerance(mut self, tol: f64) -> Self {
        self.tolerance = Some(tol);
        self
    }

    /// Sets the maximum number of iterations.
    pub fn max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = Some(max);
        self
    }

    /// Enables or disables logging of convergence steps.
    pub fn log_convergence(mut self, log: bool) -> Self {
        self.log_convergence = Some(log);
        self
    }

    /// Sets the target function to be used by the root finder.
    pub fn function(mut self, function: &'a dyn Fn(f64) -> f64) -> Self {
        self.function = Some(function);
        self
    }

    /// Sets the derivative of the target function (required for Newton-Raphson).
    pub fn derivative(mut self, derivative: &'a dyn Fn(f64) -> f64) -> Self {
        self.derivative = Some(derivative);
        self
    }

    /// Builds and returns the `RootFinder` instance.
    pub fn build(self) -> Result<RootFindingIterationDecorator<'a>, String> {
        let function = self.function.ok_or("Function must be specified")?;
        let tolerance = self.tolerance.ok_or("Tolerance must be specified.")?;
        let max_iterations = self
            .max_iterations
            .ok_or("Max iterations must be specified.")?;
        let log_convergence = self.log_convergence.unwrap_or(false);
        // Validate the build configuration based on the selected method
        let rf: Result<Box<dyn RootFinder + 'a>, String> = match self.method {
            RootFindingMethod::NewtonRaphson => {
                let derivative = self.derivative.ok_or("Derivative must be specified")?;
                let initial_guess = self
                    .initial_guess
                    .ok_or("Initial guess must be specified")?;

                Ok(Box::new(newton_raphson::NewtonRaphsonRootFinder {
                    function,
                    derivative,
                    x0: initial_guess,
                    tolerance,
                    fx: f64::NAN,
                    dfx: f64::NAN,
                }))
            }
            RootFindingMethod::Secant => {
                let boundaries = self
                    .boundaries
                    .ok_or("Derivative must be specified for Secant method.")?;

                Ok(Box::new(secant::SecantRootFinder {
                    function,
                    x0: boundaries.0,
                    x1: boundaries.1,
                    x2: f64::NAN,
                    tolerance,
                    fx0: f64::NAN,
                    fx1: f64::NAN,
                }))
            }
            // Handle other methods if needed
            _ => Err("Unsupported method in this example.".to_string()),
        };
        Ok(RootFindingIterationDecorator::new(
            rf?,
            max_iterations,
            log_convergence,
        ))
    }
}
