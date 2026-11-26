// Define the Problem trait.

///
/// This trait define optimization problem.
///
pub trait Problem: Sync {
    type Genome: Clone + Send + Sync;
    type Fitness: Send;

    fn evaluate(&self, genome: &Self::Genome) -> Self::Fitness;
}
