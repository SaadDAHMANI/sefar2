// Define the Problem trait.

///
/// This trait define optimization problem.
///
pub trait Problem {
    type Genome: Clone;
    type Fitness;
    /// Description of the search space: `Bounds` for real problems,
    /// a length for binary ones, and so on.
    type Domain;

    fn evaluate(&mut self, genome: &Self::Genome) -> Self::Fitness;
    fn domain(&self) -> Self::Domain;
}

///
/// Define the trait for Continuous Problems.
///
pub trait SingleObjectiveReal: Problem<Genome = Vec<f64>, Fitness = f64, Domain = Bounds> {}

impl<T> SingleObjectiveReal for T where T: Problem<Genome = Vec<f64>, Fitness = f64, Domain = Bounds>
{}

/// Define the trait for Binary Problems
pub trait SingleObjectiveBinary:
    Problem<Genome = Vec<bool>, Fitness = f64, Domain = usize>
{
}
impl<T> SingleObjectiveBinary for T where
    T: Problem<Genome = Vec<bool>, Fitness = f64, Domain = usize>
{
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
    pub fn dimension(&self) -> usize {
        self.lower.len()
    }
}

/// Fitness for multi-objective problems
pub type Objectives = Vec<f64>;
