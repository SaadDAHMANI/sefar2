// Binary benchmarks
use crate::core::problem::Problem;

#[derive(Clone)]
pub struct OneMax {
    pub n: usize,
}

impl Problem for OneMax {
    type Genome = Vec<bool>;
    type Fitness = f64;
    type Domain = usize; // genome length

    fn domain(&self) -> usize {
        self.n
    }

    fn evaluate(&mut self, x: &Vec<bool>) -> f64 {
        // minimization convention: fewer ones = better
        -(x.iter().filter(|&&b| b).count() as f64)
    }
}
