// Define the Problem trait.

///
/// This trait define optimization problem.
///
pub trait Problem: Sync {
    type Genome: Clone + Send + Sync;
    type Fitness: Send;

    fn evaluate(&self, genome: &Self::Genome) -> Self::Fitness;
}

///
/// Define the trait for Continuous Problems.
///
pub trait RealProblem: Problem<Genome = Vec<f64>> {
    fn dimension(&self) -> usize;
    fn bounds(&self) -> Bounds;
}

/// Define the trait for Binary Problems
pub trait BinaryProblem: Problem<Genome = Vec<bool>> {
    fn length(&self) -> usize;
}

///
///  Lower and Upper bound for real problems.
///
pub struct Bounds {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

/// Fitness for multi-objective problems
pub type Objectives = Vec<f64>;
