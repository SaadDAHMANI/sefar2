use super::problem::Problem;

///
///
///
pub trait Evaluator {
    fn evaluate<P: Problem>(&self, problem: &P, pop: &[P::Genome]) -> Vec<P::Fitness>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Sequential;

impl Evaluator for Sequential {
    fn evaluate<P: Problem>(&self, problem: &P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        pop.iter().map(|g| problem.evaluate(g)).collect()
    }
}

// --------------------------------------
#[derive(Debug, Clone, Copy, Default)]
pub struct Parallel;

impl Evaluator for Parallel {
    fn evaluate<P: Problem>(&self, problem: &P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        use rayon::prelude::*;
        pop.par_iter().map(|g| problem.evaluate(g)).collect()
    }
}
