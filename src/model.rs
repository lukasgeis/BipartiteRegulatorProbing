use rand::Rng;

use crate::{
    algorithms::Instance,
    distributions::{max_distributions, sum_distributions, DiscreteDistribution},
    GoalFunction, ProbingAction, Reward, Setting, Solution, Time,
};

/// Main Model of BPR holding all necessary Information
#[derive(Debug)]
pub struct BipartiteRegulatorProbing {
    /// Number of Regulators
    na: usize,
    /// Number of Positions
    nb: usize,
    /// Size of Support
    vs: usize,
    /// Distributions of Edges
    edges: Vec<Vec<DiscreteDistribution>>,
    /// ProbeMax-Reductions
    probemax: (
        Option<Vec<DiscreteDistribution>>,
        Option<Vec<DiscreteDistribution>>,
    ),
    /// Non-Adaptive-Algorithms
    non_adaptive_algorithms: Vec<Solution>,
    /// Optimal-Adaptive-Algorithm
    optimal_adaptive_table: Vec<(Setting, Vec<Vec<Option<(ProbingAction, Reward)>>>, Time)>,
    /// Name-Coding
    coding: String,
}

impl BipartiteRegulatorProbing {
    /// Creates a new BPR Model
    pub fn new(na: usize, nb: usize, vs: usize, poisson: bool) -> Self {
        let mut edges: Vec<Vec<DiscreteDistribution>> = Vec::with_capacity(na);
        for _ in 0..na {
            let mut regulator: Vec<DiscreteDistribution> = Vec::with_capacity(nb);
            for _ in 0..nb {
                regulator.push(DiscreteDistribution::new(vs, poisson));
            }
            edges.push(regulator);
        }

        Self {
            na: na,
            nb: nb,
            vs: vs,
            edges: edges,
            probemax: (None, None),
            non_adaptive_algorithms: Vec::new(),
            optimal_adaptive_table: Vec::new(),
            coding: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        }
    }

    /// Get Number of Regulators
    pub fn get_na(&self) -> usize {
        self.na
    }

    /// Get Number of Positions
    pub fn get_nb(&self) -> usize {
        self.nb
    }

    /// Get Size of Support
    pub fn get_vs(&self) -> usize {
        self.vs
    }

    /// Get Coding of BPR
    pub fn get_coding(&self) -> &String {
        &self.coding
    }

    /// Get the Distributions of all incident edges of Regulator a
    pub fn get_regulator(&self, a: usize) -> &Vec<DiscreteDistribution> {
        &self.edges[a]
    }

    /// Get the Distribution of edge (a,b)
    pub fn get_edge(&self, a: usize, b: usize) -> &DiscreteDistribution {
        &self.edges[a][b]
    }

    /// Get the ProbeMax-Distributions for a certain goal or compute it if not computed already
    pub fn get_probemax(&mut self, goal: &GoalFunction) -> &Vec<DiscreteDistribution> {
        match goal {
            GoalFunction::COV => panic!("There is no ProbeMax-Variant of Coverage!"),
            GoalFunction::MAX => {
                if self.probemax.0.is_none() {
                    self.probemax.0 =
                        Some(self.edges.iter().map(|a| max_distributions(a)).collect());
                }
                return self.probemax.0.as_ref().unwrap();
            }
            GoalFunction::SUM => {
                if self.probemax.1.is_none() {
                    self.probemax.1 =
                        Some(self.edges.iter().map(|a| sum_distributions(a)).collect());
                }
                return self.probemax.1.as_ref().unwrap();
            }
        }
    }

    /// Get Non-Adaptive Algorithm if found
    pub fn get_algorithm(&self, setting: Setting) -> Option<&Solution> {
        for algo in &self.non_adaptive_algorithms {
            if algo.0 == setting {
                return Some(&algo);
            }
        }
        None
    }

    /// Add Non-Adaptive Algorithm
    pub fn add_non_adaptive_solution(&mut self, solution: Solution) {
        self.non_adaptive_algorithms.push(solution);
    }

    /// Get Optimal Adaptive Policy
    pub fn get_optimal_adaptive_probemax_policy(
        &self,
        setting: Setting,
    ) -> Option<(&Vec<Vec<Option<(ProbingAction, Reward)>>>, Time)> {
        for entry in &self.optimal_adaptive_table {
            if entry.0 == setting {
                return Some((&entry.1, entry.2));
            }
        }
        None
    }

    /// Add Optimal Adaptive Policy
    pub fn add_optimal_adaptive_probemax_policy(
        &mut self,
        setting: Setting,
        policy: Vec<Vec<Option<(ProbingAction, Reward)>>>,
        time: Time,
    ) {
        self.optimal_adaptive_table.push((setting, policy, time));
    }

    /// Creates an Instance of this BPR model
    pub fn create_instance(&mut self) -> Instance {
        Instance::new(self)
    }
}
