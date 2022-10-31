use std::str::FromStr;

use model::BipartiteRegulatorProbing;

extern crate core;

pub mod distributions;
pub mod mdp;
pub mod model;

pub type Probability = f64;
pub type Reward = f64;
pub type Time = f64;

pub type Setting = (GoalType, Algorithm, usize, usize);
pub type Solution = (Setting, Time, Vec<usize>, Option<usize>);

#[derive(Debug, PartialEq, Clone)]
/// Type of Goal function
pub enum GoalType {
    /// Max-Edge-Variant
    MAX,
    /// Sum-Edge-Variant
    SUM,
    /// Coverage-Variant
    COV,
}

#[derive(Debug, PartialEq, Clone)]
/// Algorithm
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
    /// All Algorithms above expect MDP
    POLY,
}

/// Allow parsing Algorithm from Structopt
impl FromStr for Algorithm {
    type Err = &'static str;
    fn from_str(algo: &str) -> Result<Self, Self::Err> {
        match algo {
            "MDP" => Ok(Algorithm::MDP),
            "AMP" => Ok(Algorithm::AMP),
            "SCG" => Ok(Algorithm::SCG),
            "OPT" => Ok(Algorithm::OPT),
            "ALL" => Ok(Algorithm::ALL),
            "NAMP" => Ok(Algorithm::NAMP),
            "POLY" => Ok(Algorithm::POLY),
            _ => Err("Could not parse Algorithm!"),
        }
    }
}

/// Checks if two numbers of f64 are close enough to let their difference be considered a numerical error
pub fn is_close(a: f64, b: f64, tol: Option<f64>) -> bool {
    (b - a).abs() < tol.unwrap_or(1e-09)
}

/// Generates all possible combination of length n and values 0..v
pub fn combinations(n: usize, values: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut val: Vec<Vec<usize>> = Vec::new();
    for v in 0..values {
        for vector in combinations(n - 1, values) {
            let mut x: Vec<usize> = Vec::with_capacity(n);
            x.push(v);
            for z in vector {
                x.push(z);
            }
            val.push(x);
        }
    }

    val.into_iter()
        .map(|v| v.into_iter().rev().collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>()
}

/// Get all possible combinations of true/false of length n (for subset calculations)
pub fn boolean_combination(n: usize) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![];
    }
    combinations(n, 2)
        .into_iter()
        .map(|v| v.into_iter().map(|u| u == 1).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}

/// Converts a vector of boolen (binary reprensentation) to its equivalent number
pub fn boolean_array_to_usize(arr: &Vec<bool>) -> usize {
    let mut val: usize = 0;
    for k in 0..arr.len() {
        if arr[k] {
            val += 2_usize.pow(k as u32);
        }
    }

    val
}

/// Convert a BPR-model into a suitable output format
pub fn model_to_string(bpr: &BipartiteRegulatorProbing) -> String {
    format!(
        "Name: {} -- na: {} -- nb: {} -- vs: {}",
        bpr.get_name(),
        bpr.get_na(),
        bpr.get_nb(),
        bpr.get_vs()
    )
}

/// COnvert a Solution into a suitable output format
pub fn solution_to_string(solution: &Solution) -> String {
    format!(
        "Goal: {:?} -- Algorithm: {:?} -- k: {} -- l: {} -- Time: {} -- Subset: {:?} -- Value: {}",
        solution.0 .0,
        solution.0 .1,
        solution.0 .2,
        solution.0 .3,
        solution.1,
        solution.2,
        solution.3.unwrap_or(0)
    )
}
