use crate::core::problem::{Bounds, Problem, RealProblem};

/// Sphere problem benchmark
///
#[derive(Clone)]
struct Sphere {
    dim: usize,
}

impl Problem for Sphere {
    type Genome = Vec<f64>;
    type Fitness = f64;

    fn evaluate(&mut self, x: &Vec<f64>) -> f64 {
        x.iter().map(|v| v * v).sum()
    }
}

impl RealProblem for Sphere {
    fn dimension(&self) -> usize {
        self.dim
    }
    fn bounds(&self) -> Bounds {
        Bounds {
            lower: vec![-5.12; self.dim],
            upper: vec![5.12; self.dim],
        }
    }
}
