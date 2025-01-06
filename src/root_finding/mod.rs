use std::fmt::Debug;

mod builder;
mod newton_raphson;
mod secant;

pub use builder::RootFinderBuilder;

#[derive(Debug)]
pub enum RootFindingMethod {
    Bisection,
    Brent,
    Secant,
    InverseQuadraticInterpolation,
    NewtonRaphson,
}

pub struct RootFindingIterationDecorator<'a> {
    num_it: usize,
    max_iterations: usize,
    log_convergence: bool,
    root_finder: Box<dyn RootFinder + 'a>,
}

impl<'a> RootFindingIterationDecorator<'a> {
    pub fn new(
        root_finder: Box<dyn RootFinder + 'a>,
        max_iterations: usize,
        log_convergence: bool,
    ) -> Self {
        Self {
            num_it: 1,
            max_iterations,
            log_convergence,
            root_finder,
        }
    }

    pub fn find_root(&mut self) -> Result<f64, String> {
        let rf = &mut self.root_finder;
        let mut args = rf.get_init_args();
        loop {
            let vals = rf.evaluate();
            //TODO: Add time logging as well
            if self.log_convergence {
                println!(
                    "Iteration {}: args = {:?}, vals = {:?}",
                    self.num_it, args, vals
                );
            }
            let should_stop: Option<Result<f64, String>> = rf.should_stop();
            if let Some(res) = should_stop {
                return res;
            }
            if self.num_it == self.max_iterations {
                return Err("Maximum iterations reached without convergence.".to_string());
            }
            self.num_it += 1;
            args = rf.get_next_args();
        }
    }
}

pub trait RootFinder {
    fn evaluate(&mut self) -> (f64, f64);
    fn get_init_args(&mut self) -> (f64, f64);
    fn get_next_args(&mut self) -> (f64, f64);
    fn should_stop(&self) -> Option<Result<f64, String>>;
}
