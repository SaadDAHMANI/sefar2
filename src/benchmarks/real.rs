/// Sphere problem benchmark
///
struct Sphere { dim: usize }

impl Problem for Sphere {
    type Genome = Vec<f64>;
    type Fitness = f64;

    fn evaluate(&self, x: &Vec<f64>) -> f64 {
        x.iter().map(|v| v * v).sum()
    }
}

impl RealProblem for Sphere {
    fn dimension(&self) -> usize { self.dim }
    fn bounds(&self) -> Bounds {
        Bounds { lower: vec![-5.12; self.dim], upper: vec![5.12; self.dim] }
    }
}
