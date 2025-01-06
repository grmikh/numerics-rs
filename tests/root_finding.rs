#[cfg(test)]
mod tests {
    use numerics_rs::root_finding::*;
    #[test]
    fn test_newton_raphson() {
        // Define the function and its derivative
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

    #[test]
    fn test_secant() {
        // Define the function and its derivative
        let function = |x: f64| x.powi(3) - x - 2.0; // f(x) = x³ - x - 2

        // Create the builder and configure it for Secant method
        let builder = RootFinderBuilder::new(RootFindingMethod::Secant)
            .function(&function)
            .boundaries(-400.0, 400.0) // Terrible initial guess on purpose
            .tolerance(1e-6) // Convergence tolerance
            .max_iterations(100) // Maximum iterations
            .log_convergence(true); // Enable logging

        // Build the root finder
        let mut root_finder = builder.build().expect("Failed to build RootFinder");

        let res = root_finder.find_root();
        assert!((res.unwrap() - 1.5213797).abs() < 1e-6);
    }
}
