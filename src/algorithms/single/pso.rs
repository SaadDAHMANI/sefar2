// Real-coded PSO, single objective only

impl<P, E> Optimizer<P> for Pso<E>
where
    P: RealProblem<Fitness = f64>,
    E: Evaluator,
{
    type Output = SingleResult<Vec<f64>>;

    fn optimize(&mut self, problem: &P) -> Self::Output {
        let dim = problem.dimension();
        let bounds = problem.bounds();
        // init positions within bounds, velocities, personal bests...
        todo!()
    }
}
