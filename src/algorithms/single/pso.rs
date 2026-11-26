// Real-coded PSO, single objective only
impl<P, E> Optimizer<P> for Pso<E>
where
    P: RealProblem + Problem<Fitness = f64>,
    E: Evaluator,
{
    type Output = SingleResult<Vec<f64>>;
    fn optimize(&mut self, problem: &P) -> Self::Output { todo!() }
}
