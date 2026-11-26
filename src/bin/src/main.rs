use sefar2::algorithms::pso::{Pso, PsoConfig};
use sefar2::benchmarks::real::Sphere;
use sefar2::core::evaluator::Sequential;

use sefar2::core::optimizer::Optimizer;

fn main() {
    println!("Hello, Sefar2!");
    pso_test1();
}

#[allow(dead_code)]
fn pso_test1() {
    let config = PsoConfig {
        c1: 1.0,
        c2: 2.0,
        swarm_size: 10,
        max_iterations: 100,
    };

    let mut problem = Sphere::new(5);

    let mut algo = Pso::new(config, Sequential);

    let output = algo.optimize(&mut problem);

    println!(
        "best.sol : {:?}, best-fit : {:?}",
        output.best_genome, output.best_fitness
    );
}
