use crate::core::problem::{Bounds, Problem, RealProblem};

/// Sphere problem benchmark
///
#[allow(dead_code)]
#[derive(Clone)]
pub struct Sphere {
    pub dim: usize,
}

impl Sphere {
    pub fn new(dim: usize) -> Self {
        Self { dim }
    }
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
