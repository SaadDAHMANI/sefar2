// use std::ops::Bound;

// Real-coded PSO, single objective only
use crate::core::{
    callback::{Callback, Progress},
    error::Error,
    evaluator::Evaluator,
    optimizer::Optimizer,
    problem::SingleObjectiveReal,
    result::SingleResult,
};

use std::ops::ControlFlow;

use crate::operators::init::{fill_by_random_uniform, real_uniform};

pub struct Pso<'a, E> {
    pub config: PsoConfig,
    pub evaluator: E,
    callback: Option<Callback<'a>>,
}

impl<'a, E> Pso<'a, E> {
    pub fn new(config: PsoConfig, evaluator: E) -> Self {
        Self {
            config,
            evaluator,
            callback: None,
        }
    }

    pub fn with_callback<F>(mut self, f: F) -> Self
    where
        F: FnMut(&Progress<'_, [f64]>) -> ControlFlow<()> + 'a,
    {
        self.callback = Some(Box::new(f));
        self
    }
}

impl<'a, P, E> Optimizer<P> for Pso<'a, E>
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
        let mut gbest_x_fit: f64 = f64::MAX;
        let mut gbest_x = vec![0.0; dim];

        //--------------------------------------
        // Define the PSO's paramters
        let c1: f64 = self.config.c1;
        let c2: f64 = self.config.c2;

        let w_max: f64 = 0.9;
        let w_min: f64 = 0.2;

        let mut v_max: Vec<f64> = Vec::with_capacity(dim);
        let mut v_min: Vec<f64> = Vec::with_capacity(dim);

        for i in 0..dim {
            v_max.push((ub[i] - lb[i]) * 0.2f64);
            v_min.push(-1.0 * v_max[i]);
        }

        let mut cgcurve = vec![0.0f64; max_iter];

        // Velocities initialization
        let mut v = vec![vec![0.0f64; dim]; nop];
        let mut rand1 = vec![0.0f64; dim];
        let mut rand2 = vec![0.0f64; dim];

        //------------------------------------
        // PSO algorithm
        // Particles initialization

        let mut rng = rand::rng();
        let mut particles = real_uniform(nop, &bounds, &mut rng);
        //initialize pbest_x population with (fitness = f64::MAX)
        let mut pbest_x = real_uniform(nop, &bounds, &mut rng);
        let fitness_pbest_x = self.evaluator.evaluate(problem, &pbest_x);

        // ------------------------------------
        // Main PSO loop
        let mut iter_counter: usize = 0;
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
                if fitness_particles[k] < gbest_x_fit {
                    gbest_x_fit = fitness_particles[k];
                    gbest_x[..dim].clone_from_slice(&particles[k][..dim]);
                }
            }
            //Update the x and v
            let tf64 = iter as f64;
            let max_iterf64 = max_iter as f64;
            let w = w_max - ((tf64 * (w_max - w_min)) / max_iterf64);

            for k in 0..nop {
                fill_by_random_uniform(&mut rand1, &mut rng, 0.0, 1.0);
                fill_by_random_uniform(&mut rand2, &mut rng, 0.0, 1.0);

                for j in 0..dim {
                    v[k][j] = (w * v[k][j])
                        + (c1 * rand1[j] * (pbest_x[k][j] - particles[k][j]))
                        + (c2 * rand2[j] * (gbest_x[j] - particles[k][j]));
                }
                // -----------------------------------
                for j in 0..dim {
                    if v[k][j] > v_max[j] {
                        //index1.push(j);
                        v[k][j] = v_max[j];
                    }

                    if v[k][j] < v_min[j] {
                        //index2.push(j);
                        v[k][j] = v_min[j];
                    }
                }
                //------------------------------------
                // Update particles positions
                for j in 0..dim {
                    particles[k][j] = particles[k][j] + v[k][j];
                }
                //------------------------------------
                for j in 0..dim {
                    if particles[k][j] > ub[j] {
                        particles[k][j] = ub[j];
                    }

                    if particles[k][j] < lb[j] {
                        particles[k][j] = lb[j];
                    }
                }
            }
            cgcurve[iter] = gbest_x_fit;
            iter_counter = iter + 1;
            //---------- Call the callback -----------------------------
            if let Some(cb) = self.callback.as_mut() {
                let info = Progress {
                    iteration: iter + 1,
                    evaluations: iter_counter * nop,
                    best_genome: gbest_x.as_slice(),
                    best_fitness: gbest_x_fit,
                };
                if cb(&info).is_break() {
                    break;
                }
            }
        }

        //--------------------------------------
        let result = SingleResult {
            best_genome: gbest_x,
            best_fitness: gbest_x_fit,
            evaluations: iter_counter,
            history: cgcurve,
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
