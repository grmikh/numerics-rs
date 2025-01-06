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
type F = dyn Fn(f64) -> f64;
pub struct RootFindingIterationDecorator<'a> {
    pub(super) function: &'a F,           // The target function f(x)
    pub(super) derivative: Option<&'a F>, // The derivative f'(x)
    num_it: usize,
    max_iterations: usize,
    log_convergence: bool,
    root_finder: Box<dyn RootFinder + 'a>,
}

impl<'a> RootFindingIterationDecorator<'a> {
    pub fn new(
        function: &'a F,           // The target function f(x)
        derivative: Option<&'a F>, // The derivative f'(x)
        root_finder: Box<dyn RootFinder + 'a>,
        max_iterations: usize,
        log_convergence: bool,
    ) -> Self {
        Self {
            function,
            derivative,
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
            let fx = args
                .iter()
                .map(|arg| (self.function)(*arg))
                .collect::<Vec<_>>();
            let mut dfx = vec![];
            if self.derivative.is_some() {
                dfx.extend(args.iter().map(|arg| self.derivative.unwrap()(*arg)));
            }
            //TODO: Add time logging as well
            if self.log_convergence {
                println!(
                    "Iteration {}: x = {:?}, fx = {:?}, dfx = {:?}",
                    self.num_it, args, fx, dfx
                );
            }
            let should_stop: Option<Result<f64, String>> = rf.should_stop(&fx, &dfx);
            if let Some(res) = should_stop {
                return res;
            }
            if self.num_it == self.max_iterations {
                return Err("Maximum iterations reached without convergence.".to_string());
            }
            self.num_it += 1;
            args = rf.get_next_args(&fx, &dfx);
        }
    }
}

pub trait RootFinder {
    fn get_init_args(&mut self) -> Box<[f64]>;
    fn get_next_args(&mut self, fx: &[f64], dfx: &[f64]) -> Box<[f64]>;
    fn should_stop(&self, fx: &[f64], dfx: &[f64]) -> Option<Result<f64, String>>;
}
