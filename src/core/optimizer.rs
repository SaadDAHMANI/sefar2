use super::problem::Problem;

///
/// Define the Optimizer trait.
///
pub trait Optimizer<P: Problem> {
    type Output;
    fn optimize(&mut self, problem: &mut P) -> Self::Output;
}
