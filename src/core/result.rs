use super::problem::Objectives;

///
/// Optimization result of single objective.
///
pub struct SingleResult<G> {
    pub best_genome: G,
    pub best_fitness: f64,
    pub evaluations: usize,
    pub history: Vec<f64>,
}

///
/// Optimization result of multi-objectives.
///
pub struct ParetoResult<G> {
    pub front: Vec<(G, Objectives)>,
    pub evaluations: usize,
    pub history: Vec<f64>, // e.g. hypervolume per generation
}
