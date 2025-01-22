use std::fmt::Debug;

mod bisection;
mod brent;
mod builder;
mod convergencelog;
mod newton_raphson;
mod secant;

pub use builder::RootFinderBuilder;
pub use convergencelog::ConvergenceLog;

#[derive(Debug)]
pub enum RootFindingMethod {
    Bisection,
    Brent,
    Secant,
    InverseQuadraticInterpolation,
    NewtonRaphson,
}
type F = dyn Fn(f64) -> f64;

pub trait RootFindingIterator<'a> {
    fn find_root(&mut self) -> Result<f64, String>;
    fn get_convergence_log(&self) -> &ConvergenceLog;
}
pub struct RootFindingIterationDecorator<'a> {
    function: &'a F,           // The target function f(x)
    derivative: Option<&'a F>, // The derivative f'(x)
    num_it: usize,
    max_iterations: usize,
    log_convergence: bool,
    root_finder: Box<dyn RootFinder + 'a>,
    convergence_log: ConvergenceLog,
}

impl<'a> RootFindingIterationDecorator<'a> {
    fn new(
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
            convergence_log: ConvergenceLog::new(),
        }
    }
}
impl<'a> RootFindingIterator<'a> for RootFindingIterationDecorator<'a> {
    fn find_root(&mut self) -> Result<f64, String> {
        self.convergence_log.reset();
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
                self.convergence_log
                    .add_entry(self.num_it, args, Box::from(&fx[..]));
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

    fn get_convergence_log(&self) -> &ConvergenceLog {
        &self.convergence_log
    }
}

pub trait RootFinder {
    fn get_init_args(&mut self) -> Box<[f64]>;
    fn get_next_args(&mut self, fx: &[f64], dfx: &[f64]) -> Box<[f64]>;
    fn should_stop(&self, fx: &[f64], dfx: &[f64]) -> Option<Result<f64, String>>;
}
