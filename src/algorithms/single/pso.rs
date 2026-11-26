// Real-coded PSO, single objective only
use crate::core::{
    evaluator::Evaluator, optimizer::Optimizer, problem::RealProblem, result::SingleResult,
};

pub struct Pso<E> {
    pub config: PsoConfig,
    pub evaluator: E,
}

impl<E> Pso<E> {
    pub fn new(config: PsoConfig, evaluator: E) -> Self {
        Self { config, evaluator }
    }
}

impl<P, E> Optimizer<P> for Pso<E>
where
    P: RealProblem<Fitness = f64>,
    E: Evaluator<P>,
{
    type Output = SingleResult<Vec<f64>>;

    fn optimize(&mut self, problem: &mut P) -> Self::Output {
        // ... identical to before; these calls now go through &mut:
        //     let dim = problem.dimension();
        //     let fitness = self.evaluator.evaluate(problem, &positions);
        todo!()
    }
}

pub struct PsoConfig {
    pub c1: f64,
    pub c2: f64,
}
