/// Represents a log to keep track of the convergence of a root-finding algorithm.
pub struct ConvergenceLog {
    iterations: Vec<IterationEntry>,
}

/// Represents a single iteration's data in the convergence log.
#[derive(Debug, Clone)]
pub struct IterationEntry {
    pub iteration: usize, // The iteration number
    pub x: Box<[f64]>,    // Vector of points considered in this iteration
    pub fx: Box<[f64]>,   // Corresponding function values at the points in `x`
}

impl ConvergenceLog {
    /// Creates a new, empty convergence log.
    pub fn new() -> Self {
        Self {
            iterations: Vec::new(),
        }
    }

    /// Adds an iteration entry to the convergence log, supporting multiple x and f(x).
    pub fn add_entry(&mut self, iteration: usize, x: Box<[f64]>, fx: Box<[f64]>) {
        // Ensure x and fx have the same length
        assert_eq!(
            x.len(),
            fx.len(),
            "x and fx vectors must have the same length"
        );

        self.iterations.push(IterationEntry { iteration, x, fx });
    }

    /// Retrieves all logged iterations.
    pub fn get_entries(&self) -> &Vec<IterationEntry> {
        &self.iterations
    }

    /// Displays the convergence log in a readable format.
    pub fn display_log(&self) {
        println!(
            "{:<10} {:<30} {:<30} {:<15}",
            "Iteration", "x", "f(x)", "Error"
        );
        println!("{:<10} {:<30} {:<30} {:<15}", "-", "-", "-", "-");

        for entry in &self.iterations {
            // Join x and f(x) values into comma-separated strings for display
            let x_str = entry
                .x
                .iter()
                .map(|val| format!("{:.6}", val))
                .collect::<Vec<_>>()
                .join(", ");
            let fx_str = entry
                .fx
                .iter()
                .map(|val| format!("{:.6}", val))
                .collect::<Vec<_>>()
                .join(", ");

            println!("{:<10} {:<30} {:<30}", entry.iteration, x_str, fx_str);
        }
    }

    /// Clears the log for reuse.
    pub fn reset(&mut self) {
        self.iterations.clear();
    }
}

impl Default for ConvergenceLog {
    fn default() -> Self {
        ConvergenceLog::new()
    }
}
