
// NSGA-II, multi-objective only
impl<P, E> Optimizer<P> for Nsga2<E>
where
    P: RealProblem + Problem<Fitness = Objectives>,
    E: Evaluator,
{
    type Output = ParetoResult<Vec<f64>>;
    fn optimize(&mut self, problem: &P) -> Self::Output { todo!() }
}
