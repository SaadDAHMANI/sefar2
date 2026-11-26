use super::problem::Problem;
use crate::core::error::Error;
///
/// Define the Optimizer trait.
///
pub trait Optimizer<P: Problem> {
    type Output;
    fn optimize(&mut self, problem: &mut P) -> Result<Self::Output, Error>;
}
