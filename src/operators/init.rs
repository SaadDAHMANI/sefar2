// init file

use crate::core::problem::Bounds;
use rand::{Rng, RngExt};

pub fn real_uniform(n: usize, bounds: &Bounds, rng: &mut impl Rng) -> Vec<Vec<f64>> {
    (0..n)
        .map(|_| {
            bounds
                .lower
                .iter()
                .zip(&bounds.upper)
                .map(|(&l, &u)| rng.random_range(l..=u))
                .collect()
        })
        .collect()
}

pub fn binary_uniform(n: usize, length: usize, rng: &mut impl Rng) -> Vec<Vec<bool>> {
    (0..n)
        .map(|_| (0..length).map(|_| rng.random_bool(0.5)).collect())
        .collect()
}
