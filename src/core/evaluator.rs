use super::problem::Problem;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::{Mutex, OnceLock};
// ------------------------------------------------------------------------------------------
///
///
pub trait Evaluator<P: Problem> {
    fn evaluate(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness>;

    /// Evaluate one genome.
    fn evaluate_one(&self, problem: &mut P, genome: &P::Genome) -> P::Fitness {
        problem.evaluate(genome)
    }
}

///
/// Sequential optimization.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct Sequential;

impl<P: Problem> Evaluator<P> for Sequential {
    fn evaluate(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        // No clone: the problem is mutated in place.
        pop.iter().map(|g| problem.evaluate(g)).collect()
    }
}

// ------------------------------------------------------------------------------------------
// Parallel evaluator with one problem copy per worker thread.
///
/// The copies are cloned from the first problem passed to `evaluate` and
/// reused on every later call, so use one `Parallel` per problem.
pub struct Parallel<P> {
    pool: ThreadPool,
    copies: OnceLock<Vec<Mutex<P>>>,
}

impl<P> Parallel<P> {
    pub fn new(threads: usize) -> Self {
        let pool = ThreadPoolBuilder::new()
            .num_threads(
                threads.max(1).min(
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(1),
                ),
            )
            .build()
            .expect("failed to build rayon thread pool");
        Self {
            pool,
            copies: OnceLock::new(),
        }
    }

    pub fn with_all_cores() -> Self {
        Self::new(std::thread::available_parallelism().map_or(1, |n| n.get()))
    }

    pub fn threads(&self) -> usize {
        self.pool.current_num_threads()
    }
}

impl<P> Evaluator<P> for Parallel<P>
where
    P: Problem + Clone + Send,
    P::Genome: Sync,
    P::Fitness: Send,
{
    fn evaluate(&self, problem: &mut P, pop: &[P::Genome]) -> Vec<P::Fitness> {
        let copies = self.copies.get_or_init(|| {
            (0..self.threads())
                .map(|_| Mutex::new(problem.clone()))
                .collect()
        });

        self.pool.install(|| {
            pop.par_iter()
                .map(|g| {
                    let i = rayon::current_thread_index().expect("not in pool");
                    copies[i].lock().unwrap().evaluate(g)
                })
                .collect()
        })
    }
}
// ------------------------------------------------------------------------------------------
