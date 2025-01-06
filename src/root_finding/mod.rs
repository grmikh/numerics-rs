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

pub trait RootFinder {
    /// Executes the root-finding process based on the specified method.
    fn find_root(&mut self) -> Result<f64, String> {
        let mut num_it: usize = 1;
        let mut args = self.get_init_args();
        loop {
            let vals = self.evaluate();
            //TODO: Add time logging as well
            if self.log_convergence() {
                println!("Iteration {num_it}, args = {:#?}, vals = {:#?}", args, vals);
            }
            let should_stop: Option<Result<f64, String>> = self.should_stop(&num_it);
            if let Some(res) = should_stop {
                return res;
            }
            num_it += 1;
            args = self.get_next_args();
        }
    }
    fn evaluate(&mut self) -> (f64, f64);
    fn get_init_args(&mut self) -> (f64, f64);
    fn get_next_args(&mut self) -> (f64, f64);
    fn should_stop(&self, num_it: &usize) -> Option<Result<f64, String>>;
    fn log_convergence(&self) -> bool;
}
