pub mod algorithms;
pub mod benchmarks;
pub mod core;

pub mod prelude {
    pub use crate::algorithms::pso::{Pso, PsoConfig};
    pub use crate::core::evaluator::{Evaluator, Parallel, Sequential};
    pub use crate::core::optimizer::Optimizer;
    pub use crate::core::problem::{Bounds, Problem, SingleObjectiveBinary, SingleObjectiveReal};
    pub use crate::core::result::SingleResult;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
