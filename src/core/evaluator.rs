///
///
pub trait Evaluator {
    fn evaluate<P: Problem>(&self, problem: &P, pop: &[P::Genome]) -> Vec<P::Fitness>;
}

impl Evaluator for Parallel {
    fn evaluate<P: Problem>(&self, problem: &P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        use rayon::prelude::*;
        pop.par_iter().map(|g| problem.evaluate(g)).collect()
    }
}
