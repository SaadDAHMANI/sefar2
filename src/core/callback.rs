///
/// Controle optimization progression.
///
use std::ops::ControlFlow;

pub struct Progress<'a, G: ?Sized> {
    pub iteration: usize,
    pub evaluations: usize,
    pub best_genome: &'a G,
    pub best_fitness: f64,
}

pub type Callback<'a> = Box<dyn FnMut(&Progress<'_, [f64]>) -> ControlFlow<()> + 'a>;
