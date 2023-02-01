extern crate core;

use std::str::FromStr;

use model::BipartiteRegulatorProbing;

pub mod algorithms;
pub mod distributions;
pub mod model;

/// Custom Types
pub type Probability = f64;
pub type Reward = f64;
pub type Time = f64;

/// Settings/Solutions
pub type Setting = (GoalFunction, Algorithm, usize, usize);
pub type Solution = (Setting, Time, Vec<usize>, usize);

/// Possible GoalFunctions
#[derive(Debug, Clone, PartialEq)]
pub enum GoalFunction {
    /// ProbeMax-Reduction with MAX-Distribution
    MAX,
    /// ProbeMax-Reduction with SUM-Distribution
    SUM,
    /// Maximum-Coverage Variant
    COV,
}

/// Possible Algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    /// Adaptive-Myopic-Policy
    AMP,
    /// Non-Adaptive-Myopic-Policy
    NAMP,
    /// Optimal-Offline-Algorithm
    OPT,
    /// All Algorithms above
    ALL,
}

/// Allow parsing GoalFunction from String
impl FromStr for GoalFunction {
    type Err = &'static str;
    fn from_str(goal: &str) -> Result<Self, Self::Err> {
        match goal {
            "MAX" => Ok(GoalFunction::MAX),
            "SUM" => Ok(GoalFunction::SUM),
            "COV" => Ok(GoalFunction::COV),
            _ => Err("Could not parse GoalFunction!"),
        }
    }
}

/// Allow parsing Algorithm from String
impl FromStr for Algorithm {
    type Err = &'static str;
    fn from_str(algorithm: &str) -> Result<Self, Self::Err> {
        match algorithm {
            "AMP" => Ok(Algorithm::AMP),
            "ALL" => Ok(Algorithm::ALL),
            "OPT" => Ok(Algorithm::OPT),
            "NAMP" => Ok(Algorithm::NAMP),
            _ => Err("Could not parse Algorithm!"),
        }
    }
}

/// Are two f64 close enough to be considered the same
pub fn is_close(a: f64, b: f64) -> bool {
    (b - a).abs() < 1e-09
}

/// Get Factorial of Number
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// Convert a BPR to a suitable String to print
pub fn bpr_to_string(bpr: &BipartiteRegulatorProbing) -> String {
    format!(
        "Name: {:?} -- na: {} -- nb: {} -- vs: {}",
        bpr.get_coding(),
        bpr.get_na(),
        bpr.get_nb(),
        bpr.get_vs()
    )
}

/// Convert a Solution to a suitable String to print
pub fn solution_to_string(solution: &Solution) -> String {
    format!(
        "Goal: {:?} -- Algorithm: {:?} -- k: {} -- l: {} -- Time: {} -- Subset: {:?} -- Value: {}",
        solution.0 .0,
        solution.0 .1,
        solution.0 .2,
        solution.0 .3,
        solution.1,
        solution.2,
        solution.3
    )
}
