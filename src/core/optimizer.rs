///
/// Define the Optimizer trait.
///
pub trait Optimizer<P: Problem> {
    type Output;
    fn optimize(&mut self, problem: &P) -> Self::Output;
}
