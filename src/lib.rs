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

/// MDP Types
pub type ProbemaxState = (Vec<bool>, Vec<usize>);
pub type ProbingAction = usize;

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
    /// Markov-Decision-Process
    MDP,
    /// Adaptive-Myopic-Policy
    AMP,
    /// Non-Adaptive-Myopic-Policy
    NAMP,
    /// Stochastic-Continous-Greedy
    SCG,
    /// Optimal-Offline-Algorithm
    OPT,
    /// All Algorithms above
    ALL,
    /// All Algorithms above without MDP
    POLY,
    /// All Algorithms above without MDP and SCG
    FAST,
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
            "MDP" => Ok(Algorithm::MDP),
            "AMP" => Ok(Algorithm::AMP),
            "SCG" => Ok(Algorithm::SCG),
            "ALL" => Ok(Algorithm::ALL),
            "OPT" => Ok(Algorithm::OPT),
            "NAMP" => Ok(Algorithm::NAMP),
            "POLY" => Ok(Algorithm::POLY),
            "FAST" => Ok(Algorithm::FAST),
            _ => Err("Could not parse Algorithm!"),
        }
    }
}

/// Are two f64 close enough to be considered the same
pub fn is_close(a: f64, b: f64, tol: Option<f64>) -> bool {
    (b - a).abs() < tol.unwrap_or(1e-09)
}

/// Get Factorial of Number
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// Get all possible combinations of {0,...,v - 1} of length n
pub fn combinations(n: usize, v: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut values: Vec<Vec<usize>> = Vec::new();
    for val in 0..v {
        for vec in combinations(n - 1, v) {
            let mut z: Vec<usize> = Vec::with_capacity(n);
            z.push(val);
            for t in vec {
                z.push(t);
            }
            values.push(z);
        }
    }
    values
        .into_iter()
        .map(|vec| vec.into_iter().rev().collect::<Vec<usize>>())
        .collect()
}

/// Get all possible combinations of {true, false} of length n
pub fn boolean_combinations(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![vec![]];
    }
    combinations(n, 2)
        .into_iter()
        .map(|vec| vec.into_iter().map(|val| val == 1).collect::<Vec<bool>>())
        .collect()
}

/// Convert a boolean array to its coherrent binary number
pub fn boolean_array_to_usize(arr: &Vec<bool>) -> usize {
    let mut val: usize = 0;
    for k in 0..arr.len() {
        if arr[k] {
            val += 2usize.pow(k as u32);
        }
    }
    val
}

/// Convert a usize array to its ciherrent number in base vs
pub fn usize_array_to_usize(arr: &Vec<usize>, vs: usize) -> usize {
    let mut val: usize = 0;
    for k in 0..arr.len() {
        val += arr[k] * vs.pow(k as u32);
    }
    val
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
