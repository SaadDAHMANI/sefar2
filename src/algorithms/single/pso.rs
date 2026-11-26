// Real-coded PSO, single objective only
use crate::core::{
    error::Error, evaluator::Evaluator, optimizer::Optimizer, problem::SingleObjectiveReal,
    result::SingleResult,
};

pub struct Pso<E> {
    pub config: PsoConfig,
    pub evaluator: E,
}

impl<E> Pso<E> {
    pub fn new(config: PsoConfig, evaluator: E) -> Self {
        Self { config, evaluator }
    }

    fn validate_bounds(lower: &[f64], upper: &[f64]) -> Result<(), Error> {
        if lower.is_empty() {
            return Err(Error::InvalidProblem("dimension must be > 0".into()));
        }
        if lower.len() != upper.len() {
            return Err(Error::InvalidProblem(
                "lower and upper bounds differ in length".into(),
            ));
        }
        if lower
            .iter()
            .zip(upper)
            .any(|(l, u)| !l.is_finite() || !u.is_finite() || l > u)
        {
            return Err(Error::InvalidProblem(
                "each bound must be finite with lower <= upper".into(),
            ));
        }
        Ok(())
    }
}

impl<P, E> Optimizer<P> for Pso<E>
where
    P: SingleObjectiveReal,
    E: Evaluator<P>,
{
    type Output = SingleResult<Vec<f64>>;

    fn optimize(&mut self, problem: &mut P) -> Result<Self::Output, Error> {
        //--------------------------------------
        self.config.validate()?;
        // Pso::validate_bounds(&problem.domain().lower, &problem.domain().upper)?;
        let dim = problem.domain().dimension();
        let ub = problem.domain().upper;
        let lb = problem.domain().lower;

        let max_iter = self.config.max_iterations;
        let nop = self.config.swarm_size;
        let mut break_process: bool = false;
        //--------------------------------------
        // Define the PSO's paramters
        let c1: f64 = self.config.c1;
        let c2: f64 = self.config.c2;

        let w_max: f64 = 0.9;
        let w_min: f64 = 0.2;
        //--------------------------------------
        let result = SingleResult {
            best_genome: vec![-1.0; 5],
            best_fitness: -5.0,
            evaluations: 10,
            history: vec![-1.0; 5],
        };

        Ok(result)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PsoConfig {
    pub swarm_size: usize,
    pub max_iterations: usize,
    pub c1: f64,
    pub c2: f64,
}
impl PsoConfig {
    pub fn validate(&self) -> Result<(), Error> {
        if self.c1 < 0.0 {
            return Err(Error::InvalidConfig(
                "C1 parameter (PSO) should be greater than 0!".into(),
            ));
        }

        if self.c2 < 0.0 {
            return Err(Error::InvalidConfig(
                "C2 parameter (PSO) should be greater than 0!".into(),
            ));
        }

        if self.max_iterations == 0 {
            return Err(Error::InvalidConfig(
                "max_iterations = 0! no iteration will be run!".into(),
            ));
        }
        return Ok(());
    }
}
