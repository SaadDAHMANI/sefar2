use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// A configuration value is invalid (swarm_size = 0, c1 < 0, ...).
    InvalidConfig(String),
    /// The problem definition is inconsistent (empty domain, lower > upper, ...).
    InvalidProblem(String),
    /// The objective returned NaN (or another unusable value).
    NonFiniteFitness,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig(m) => write!(f, "invalid configuration: {m}"),
            Error::InvalidProblem(m) => write!(f, "invalid problem: {m}"),
            Error::NonFiniteFitness => write!(f, "objective function returned NaN"),
        }
    }
}

impl std::error::Error for Error {}
