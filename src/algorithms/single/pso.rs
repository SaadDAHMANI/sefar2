// use std::ops::Bound;

// Real-coded PSO, single objective only
use crate::core::{
    error::Error, evaluator::Evaluator, optimizer::Optimizer, problem::SingleObjectiveReal,
    result::SingleResult,
};
use crate::operators::init::real_uniform;

/// For disply optimization progression.
pub type Callback = Box<dyn FnMut(usize, &[f64], f64, &mut bool)>;

pub struct Pso<E> {
    pub config: PsoConfig,
    pub evaluator: E,
    callback: Option<Callback>,
}

impl<E> Pso<E> {
    pub fn new(config: PsoConfig, evaluator: E) -> Self {
        Self {
            config,
            evaluator,
            callback: None,
        }
    }
    pub fn with_callback<F>(mut self, f: F) -> Self
    where
        F: FnMut(usize, &[f64], f64, &mut bool) + 'static,
    {
        self.callback = Some(Box::new(f));
        self
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
        let bounds = problem.domain();
        let dim = bounds.dimension();
        let ub = &bounds.upper;
        let lb = &bounds.lower;

        let max_iter = self.config.max_iterations;
        let nop = self.config.swarm_size;
        let mut gbest_fit: f64 = f64::MAX;
        let mut gbest_x = vec![0.0; dim];
        let mut iter: usize = 0;

        //--------------------------------------
        // Define the PSO's paramters
        let _c1: f64 = self.config.c1;
        let _c2: f64 = self.config.c2;

        let w_max: f64 = 0.9;
        let w_min: f64 = 0.2;

        let mut v_max: Vec<f64> = Vec::with_capacity(dim);
        let mut v_min: Vec<f64> = Vec::with_capacity(dim);

        for i in 0..dim {
            v_max.push((ub[i] - lb[i]) * 0.2f64);
            v_min.push(-1.0 * v_max[i]);
        }

        let mut _cgcurve = vec![0.0f64; max_iter];

        // Velocities initialization
        let mut _v = vec![vec![0.0f64; dim]; nop];
        let mut _rand1 = vec![0.0f64; dim];
        let mut _rand2 = vec![0.0f64; dim];

        //------------------------------------
        // PSO algorithm
        // Particles initialization

        let mut rng = rand::rng();
        let mut particles = real_uniform(nop, &bounds, &mut rng);
        //initialize pbest_x population with (fitness = f64::MAX)
        let mut pbest_x = real_uniform(nop, &bounds, &mut rng);
        let mut fitness_pbest_x = self.evaluator.evaluate(problem, &pbest_x);

        // ------------------------------------
        // Main PSO loop

        for iter in 0..max_iter {
            //Objective function computation
            // Evaluate search agent using objective function
            let fitness_particles = self.evaluator.evaluate(problem, &particles);

            //Update the pbest
            for k in 0..nop {
                if fitness_particles[k] < fitness_pbest_x[k] {
                    pbest_x[k][..dim].clone_from_slice(&particles[k][..dim]);
                }

                //Update the gbest
                if fitness_particles[k] < gbest_fit {
                    gbest_fit = fitness_particles[k];
                    gbest_x[..dim].clone_from_slice(&particles[k][..dim]);
                }
            }
            //Update the x and v
            let tf64 = iter as f64;
            let max_iterf64 = max_iter as f64;
            let w = w_max - ((tf64 * (w_max - w_min)) / max_iterf64);

            //---------- Call the callback -----------------------------
            if let Some(cb) = self.callback.as_mut() {
                let mut stop = false;
                cb(iter + 1, &gbest_x, gbest_fit, &mut stop);
                if stop {
                    break;
                }
            }
        }

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
