use sefar2::benchmarks::real::Sphere;
use sefar2::prelude::Parallel;
use sefar2::prelude::*;

fn main() {
    println!("Hello, Sefar2!");
    pso_sphere_parallel_test1();
}

#[allow(dead_code)]
fn pso_sphere_sequential_test1() {
    let config = PsoConfig {
        c1: 1.0,
        c2: 2.0,
        swarm_size: 10,
        max_iterations: 100,
    };

    let mut problem = Sphere::new(5);

    let mut algo = Pso::new(config, Sequential);

    if let Ok(output) = algo.optimize(&mut problem) {
        println!(
            "best.sol : {:?}, best-fit : {:?}",
            output.best_genome, output.best_fitness
        );
    };
}

#[allow(dead_code)]
fn pso_sphere_parallel_test1() {
    let config = PsoConfig {
        c1: 2.0,
        c2: 2.0,
        swarm_size: 20,
        max_iterations: 100,
    };

    let mut problem = Sphere::new(5);

    let mut algo = Pso::new(config.clone(), Parallel::new(2));

    if let Ok(output) = algo.optimize(&mut problem) {
        println!(
            "best.sol : {:?}, best-fit : {:?} || {:?} threads were used.",
            output.best_genome,
            output.best_fitness,
            algo.evaluator.threads()
        );
    };
}
