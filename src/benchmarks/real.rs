use crate::core::problem::{Bounds, Problem};

///
/// Sphere problem benchmark
///
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
    type Domain = Bounds;

    fn domain(&self) -> Bounds {
        Bounds::uniform(-10.0, 10.0, self.dim)
    }

    fn evaluate(&mut self, x: &Vec<f64>) -> f64 {
        x.iter().map(|v| v * v).sum()
    }
}
