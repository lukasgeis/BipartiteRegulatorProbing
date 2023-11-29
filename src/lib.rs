#![feature(binary_heap_into_iter_sorted)]

use std::str::FromStr;

pub mod algorithms;
pub mod distributions;
pub mod model;

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


pub fn compute_opt_l_values(n: usize) -> [usize; 8] {
    let n16 = n / 16;
    [n16, n16 * 2, n16 * 3, n16 * 4, n16 * 6, n16 * 8, n16 * 9, n16 * 12]
}

pub fn compute_k_l_pairs(n: usize) -> [(usize, usize); 12] {
    let n4 = n / 4;
    let n16 = n / 16;
    [
        (n4, n16), (n4, n16 * 2), (n4, n16 * 3), (n4, n4),
        (n4 * 2, n16 * 2), (n4 * 2, n4), (n4 * 2, n16 * 6), (n4 * 2, n4 * 2),
        (n4 * 3, n16 * 3), (n4 * 3, n16 * 6), (n4 * 3, n16 * 9), (n4 * 3, n4 * 3)
    ]
}
