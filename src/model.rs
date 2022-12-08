use std::io::{BufRead, Error, ErrorKind};

use rand::Rng;

use crate::{
    algorithms::Instance,
    distributions::{max_distributions, sum_distributions, DiscreteDistribution},
    GoalFunction, Probability, Setting, Solution,
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
            coding: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        }
    }

    /// Reads input and returns the parsed instance
    pub fn init<T: BufRead>(reader: T) -> Result<Self, Error> {
        // Custom Error Messages
        let error = |msg| Err(Error::new(ErrorKind::Other, msg));
        // Read all lines and remove Comment-Lines (almost certainly not present)
        let mut lines = reader.lines().filter_map(|x| -> Option<String> {
            if let Ok(line) = x {
                if !line.starts_with("%") {
                    return Some(line);
                }
            }
            None
        });

        // Parse Header
        let (na, nb, vs) = {
            if let Some(header) = lines.next() {
                let fields: Vec<_> = header.split(" ").collect();
                if fields.len() != 4 {
                    return error("Expected exactly 4 header fields!");
                }

                let na: usize = match fields[1].parse() {
                    Ok(na) => na,
                    Err(_) => return error("Cannot parse number of Regulators!"),
                };

                let nb: usize = match fields[2].parse() {
                    Ok(nb) => nb,
                    Err(_) => return error("Cannot parse number of Positions!"),
                };

                let vs: usize = match fields[3].parse() {
                    Ok(vs) => vs,
                    Err(_) => return error("Cannot parse size of Support!"),
                };

                (na, nb, vs)
            } else {
                return error("Cannot parse Header!");
            }
        };

        // Parse Distributions
        let mut edges: Vec<Vec<DiscreteDistribution>> = Vec::with_capacity(na);
        for (number, line) in lines.enumerate() {
            if number % nb == 0 {
                edges.push(Vec::with_capacity(nb));
            }

            let content: Vec<_> = line.split(" ").collect();
            let (a, b) = {
                let edge: Vec<_> = content[0].split("-").collect();
                if edge.len() != 2 {
                    return error("Expected exactly 2 edge nodes!");
                }

                let a: usize = match edge[0].parse() {
                    Ok(a) => a,
                    Err(_) => return error(format!("Cannot parse Regulator {}", edge[0]).as_str()),
                };
                let b: usize = match edge[1].parse() {
                    Ok(b) => b,
                    Err(_) => return error(format!("Cannot parse Position {}", edge[1]).as_str()),
                };

                (a, b)
            };

            if a - 1 != number / nb || b - 1 != number % nb {
                return error(format!("Wrong order of edges at {}-{}", a, b).as_str());
            }

            let mut values: Vec<Probability> = Vec::with_capacity(vs);
            for v in content[1].split(",") {
                if let Ok(fv) = v.parse::<Probability>() {
                    if fv < 0.0 || fv > 1.0 {
                        return error(format!("Impossible probabilities at {}-{}", a, b).as_str());
                    }

                    values.push(fv);
                }
            }

            edges[number / nb].push(DiscreteDistribution::from_list(&values));
        }

        Ok(Self {
            na: na,
            nb: nb,
            vs: vs,
            edges: edges,
            probemax: (None, None),
            non_adaptive_algorithms: Vec::new(),
            coding: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        })
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

    /// Creates an Instance of this BPR model
    pub fn create_instance(&mut self) -> Instance {
        Instance::new(self)
    }
}
