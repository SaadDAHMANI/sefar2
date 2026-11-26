// Define the Problem trait.

///
/// This trait define optimization problem.
///
pub trait Problem {
    type Genome: Clone;
    type Fitness;

    fn evaluate(&mut self, genome: &Self::Genome) -> Self::Fitness;
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

impl Bounds {
    pub fn uniform(lower: f64, upper: f64, dim: usize) -> Self {
        Bounds {
            lower: vec![lower; dim],
            upper: vec![upper; dim],
        }
    }
}

/// Fitness for multi-objective problems
pub type Objectives = Vec<f64>;
