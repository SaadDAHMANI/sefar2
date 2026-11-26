use super::problem::Problem;

///
///
///
pub trait Evaluator {
    fn evaluate<P: Problem>(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness>;
}

///
/// Sequential optimization.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct Sequential;

impl Evaluator for Sequential {
    fn evaluate<P: Problem>(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        pop.iter().map(|g| problem.evaluate(g)).collect()
    }
}

// --------------------------------------
///
/// Parallel optimization using Rayon crate.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct Parallel;

impl<P> Evaluator<P> for Parallel
where
    P: Problem + Clone + Send + Sync,
    P::Genome: Sync,
    P::Fitness: Send,
{
    fn evaluate(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        use rayon::prelude::*;
        let shared: &P = &*problem; // shared view; each work unit gets its own copy
        pop.par_iter()
            .map_init(|| shared.clone(), |p, g| p.evaluate(g))
            .collect()
    }
}
