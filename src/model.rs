use std::{
    fs::OpenOptions,
    io::{prelude::Write, BufRead, Error, ErrorKind},
    path::PathBuf,
    time::Instant,
};

use rand::{distributions::Alphanumeric, Rng};

use crate::{
    distributions::{max_distribution, sum_distribution, Distribution},
    model_to_string, solution_to_string, Algorithm, GoalType, Probability, Setting, Solution,
};

/// Base model for BipartiteRegulatorProbing
#[derive(Debug)]
pub struct BipartiteRegulatorProbing {
    /// Number of Regulators
    na: usize,
    /// Number of Positions
    nb: usize,
    /// Size of Support
    vs: usize,
    /// Name of Graph (almost most certainly Random)
    name: String,
    /// Distributions for every Edge
    edges: Vec<Vec<Distribution>>,
    /// Reductions to Top-l-ProbeMax
    probemax: Option<(Vec<Distribution>, Vec<Distribution>)>,
    /// Non-Adaptive Policies
    non_adaptive_algorithms: Vec<Solution>,
}

impl BipartiteRegulatorProbing {
    /// Reads input and returns the parsed instance
    pub fn init<T: BufRead>(reader: T, do_reduction: bool) -> Result<Self, Error> {
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
        let (na, nb, vs, name) = {
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

                let name: String = match fields[0].parse() {
                    Ok(name) => name,
                    Err(_) => return error("Cannot parse name!"),
                };

                (na, nb, vs, name)
            } else {
                return error("Cannot parse Header!");
            }
        };

        // Parse Distributions
        let mut data: Vec<Vec<Distribution>> = Vec::with_capacity(na);
        for (number, line) in lines.enumerate() {
            if number % nb == 0 {
                data.push(Vec::with_capacity(nb));
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

            data[number / nb].push(Distribution::from_list(&values));
        }

        Ok(BipartiteRegulatorProbing {
            na: na,
            nb: nb,
            vs: vs,
            name: name,
            probemax: match do_reduction {
                false => None,
                true => Some({
                    let mut reductions: (Vec<Distribution>, Vec<Distribution>) =
                        (Vec::with_capacity(na), Vec::with_capacity(na));
                    for a in &data {
                        reductions.0.push(max_distribution(&a));
                        reductions.1.push(sum_distribution(&a));
                    }

                    reductions
                }),
            },
            edges: data,
            non_adaptive_algorithms: Vec::new(),
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

    /// Get Name of Graph (almost most certainly "Random")
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get Distributions of a Regulator
    pub fn get_regulator(&self, a: usize) -> &Vec<Distribution> {
        &self.edges[a]
    }

    /// Get Distribution from an edge (a,b)
    pub fn get_edge(&self, a: usize, b: usize) -> &Distribution {
        &self.edges[a][b]
    }

    /// Get ProbeMax-Distribution as Option
    pub fn get_probemax(&self, goal: GoalType) -> Option<&Vec<Distribution>> {
        if self.probemax.is_none() {
            return None;
        }
        match goal {
            GoalType::COV => panic!("There is no ProbeMax-Reduction for COV!"),
            GoalType::MAX => return Some(&self.probemax.as_ref().unwrap().0),
            GoalType::SUM => return Some(&self.probemax.as_ref().unwrap().1),
        };
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

    /// Create an Instance of this Model
    pub fn create_instance(&mut self) -> Instance {
        Instance::from_model(self, self.probemax.is_some())
    }
}

/// Instance of an Model with drawn values
#[derive(Debug)]
pub struct Instance<'a> {
    /// Reference to Model
    bpr: &'a mut BipartiteRegulatorProbing,
    /// Edge-Realizations
    realizations: Vec<Vec<usize>>,
    /// For faster access: ProbeMax-Realizations
    probemax_realizations: Option<(Vec<usize>, Vec<usize>)>,
    ///  Computed Results
    results: Vec<Solution>,
    /// Hash-Code of Instance (random -> generated to allow better comparison of algorithms after logging)
    coding: String,
}

impl<'a> Instance<'a> {
    /// Create an Instance from a Model
    pub fn from_model(model: &'a mut BipartiteRegulatorProbing, probemax: bool) -> Self {
        let mut realization: Vec<Vec<usize>> = Vec::with_capacity(model.get_na());
        for a in 0..model.get_na() {
            let mut regulator_realization: Vec<usize> = Vec::with_capacity(model.get_nb());
            for b in 0..model.get_nb() {
                regulator_realization.push(model.get_edge(a, b).draw_value());
            }
            realization.push(regulator_realization);
        }

        let pm: Option<(Vec<usize>, Vec<usize>)> = match probemax {
            false => None,
            true => {
                let mut pm_realization: (Vec<usize>, Vec<usize>) = (
                    Vec::with_capacity(model.get_na()),
                    Vec::with_capacity(model.get_na()),
                );
                for a in 0..model.get_na() {
                    let mut max_val: usize = 0;
                    let mut sum_val: usize = 0;
                    for b in 0..model.get_nb() {
                        if realization[a][b] > max_val {
                            max_val = realization[a][b];
                        }
                        sum_val += realization[a][b]
                    }
                    pm_realization.0.push(max_val);
                    pm_realization.1.push(sum_val);
                }
                Some(pm_realization)
            }
        };

        Instance {
            bpr: model,
            realizations: realization,
            probemax_realizations: pm,
            results: Vec::new(),
            coding: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        }
    }

    /// Get the BPR-Model
    pub fn get_model(&self) -> &BipartiteRegulatorProbing {
        &self.bpr
    }

    /// Get a specific Realization
    pub fn get_realization(&self, a: usize, b: usize) -> usize {
        self.realizations[a][b]
    }

    /// Get a realization of ProbeMax
    pub fn get_probemax_realization(&self, goal: GoalType, a: usize) -> usize {
        if self.probemax_realizations.is_none() {
            return 0;
        }
        match goal {
            GoalType::COV => panic!("There is no ProbeMax-Reduction for COV!"),
            GoalType::MAX => return self.probemax_realizations.as_ref().unwrap().0[a],
            GoalType::SUM => return self.probemax_realizations.as_ref().unwrap().1[a],
        };
    }

    /// Get a Result if existing
    pub fn get_result(&self, setting: Setting) -> Option<&Solution> {
        for algo in &self.results {
            if algo.0 == setting {
                return Some(&algo);
            }
        }
        None
    }

    /// Get Hashcode of Instance
    pub fn get_coding(&self) -> &String {
        &self.coding
    }

    /// Log the currently computed results (or a specific result) to a logfile
    pub fn log_results(&self, logfile: Option<PathBuf>, index: Option<usize>) {
        let mut outfile = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(logfile.unwrap())
            .unwrap();

        match index {
            Some(i) => {
                if i >= self.results.len() {
                    return;
                }
                if let Err(e) = writeln!(
                    outfile,
                    "{} --- {} --- Code: {}",
                    model_to_string(&self.bpr),
                    solution_to_string(&self.results[i]),
                    self.coding
                ) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
            None => {
                for sol in &self.results {
                    if let Err(e) = writeln!(
                        outfile,
                        "{} -- {} -- Code: {}",
                        model_to_string(&self.bpr),
                        solution_to_string(sol),
                        self.coding
                    ) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
        };
    }

    /// Run an algorithm on this instance
    pub fn run_algorithm(&mut self, goal: GoalType, algo: Algorithm, k: usize, l: usize) {
        if self
            .get_result((goal.clone(), algo.clone(), k, l))
            .is_some()
            || l > k
        {
            return;
        }

        match algo {
            Algorithm::OPT => {
                if goal == GoalType::COV {
                    panic!("Not implemented yet!");
                }

                let opt_time = Instant::now();

                let mut regulators: Vec<(usize, usize)> = match goal {
                    GoalType::COV => panic!("There is no way you can reach this part of the code!"),
                    GoalType::MAX => self
                        .probemax_realizations
                        .as_ref()
                        .unwrap()
                        .0
                        .clone()
                        .into_iter()
                        .enumerate()
                        .collect(),
                    GoalType::SUM => self
                        .probemax_realizations
                        .as_ref()
                        .unwrap()
                        .1
                        .clone()
                        .into_iter()
                        .enumerate()
                        .collect(),
                };

                regulators.sort_by(|(_, a), (_, b)| b.cmp(a));
                regulators.truncate(l);

                let (subset, values): (Vec<usize>, Vec<usize>) = regulators.into_iter().unzip();
                self.results.push((
                    (goal, Algorithm::OPT, l, l),
                    opt_time.elapsed().as_secs_f64(),
                    subset,
                    Some(values.into_iter().sum()),
                ));
            }
            Algorithm::ALL => {
                self.run_algorithm(goal.clone(), Algorithm::MDP, k, l);
                self.run_algorithm(goal.clone(), Algorithm::POLY, k, l);
            }
            Algorithm::POLY => {
                self.run_algorithm(goal.clone(), Algorithm::OPT, k, l);
                self.run_algorithm(goal.clone(), Algorithm::AMP, k, l);
                self.run_algorithm(goal.clone(), Algorithm::NAMP, k, l);
                self.run_algorithm(goal.clone(), Algorithm::SCG, k, l);
            }
            Algorithm::AMP => {
                let amp_time = Instant::now();
                if goal == GoalType::COV {
                    panic!("Not implemented yet!");
                }

                let mut ordering: Vec<(usize, f64)> = self
                    .bpr
                    .get_probemax(goal.clone())
                    .unwrap()
                    .into_iter()
                    .enumerate()
                    .map(|(i, d)| (i, d.expected_value()))
                    .collect();
                ordering.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                ordering.truncate(l);

                let mut probed_subset: Vec<(usize, usize)> = ordering
                    .into_iter()
                    .map(|(a, _)| (a, self.get_probemax_realization(goal.clone(), a)))
                    .collect();

                for _ in 0..(k - l) {
                    let (subset, mut values): (Vec<usize>, Vec<usize>) =
                        probed_subset.clone().into_iter().unzip();
                    values.sort_by(|a, b| b.cmp(a));

                    let argmax: usize = (0..self.bpr.get_na())
                        .into_iter()
                        .filter(|i| !subset.contains(i))
                        .map(|i| -> (usize, f64) {
                            let distributions: &Vec<Distribution> =
                                self.bpr.get_probemax(goal.clone()).unwrap();
                            (
                                i,
                                distributions[i].greater(values[l - 1] + 1)
                                    * distributions[i].expected_greater(values[l - 1] + 1),
                            )
                        })
                        .max_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
                        .unwrap()
                        .0;

                    probed_subset
                        .push((argmax, self.get_probemax_realization(goal.clone(), argmax)));
                }

                let (subset, mut values): (Vec<usize>, Vec<usize>) =
                    probed_subset.into_iter().unzip();
                values.truncate(l);
                self.results.push((
                    (goal, Algorithm::AMP, k, l),
                    amp_time.elapsed().as_secs_f64(),
                    subset,
                    Some(values.into_iter().sum()),
                ));
            }
            Algorithm::NAMP => {
                let namp_time = Instant::now();
                if goal == GoalType::COV {
                    panic!("Not implemented yet!");
                }

                if let Some(sol) = self
                    .bpr
                    .get_algorithm((goal.clone(), Algorithm::NAMP, 0, 0))
                {
                    let mut subset: Vec<usize> = sol.2.clone();
                    subset.truncate(k);
                    let mut value: usize = 0;
                    for i in 0..l {
                        if goal == GoalType::MAX {
                            value += self.probemax_realizations.as_ref().unwrap().0[subset[i]];
                        } else {
                            value += self.probemax_realizations.as_ref().unwrap().1[subset[i]];
                        }
                    }
                    self.results
                        .push(((goal, Algorithm::NAMP, k, l), 0.0, subset, Some(value)))
                } else {
                    let mut ordering: Vec<(usize, f64)> = self
                        .bpr
                        .get_probemax(goal.clone())
                        .unwrap()
                        .into_iter()
                        .enumerate()
                        .map(|(i, d)| (i, d.expected_value()))
                        .collect();
                    ordering.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                    let mut namp_order: Vec<usize> = ordering.into_iter().map(|(i, _)| i).collect();
                    self.bpr.add_non_adaptive_solution((
                        (goal.clone(), Algorithm::NAMP, 0, 0),
                        0.0,
                        namp_order.clone(),
                        None,
                    ));
                    namp_order.truncate(k);
                    let mut value: usize = 0;
                    for i in 0..l {
                        if goal == GoalType::MAX {
                            value += self.probemax_realizations.as_ref().unwrap().0[namp_order[i]];
                        } else {
                            value += self.probemax_realizations.as_ref().unwrap().1[namp_order[i]];
                        }
                    }
                    self.results.push((
                        (goal, Algorithm::NAMP, k, l),
                        namp_time.elapsed().as_secs_f64(),
                        namp_order,
                        Some(value),
                    ))
                }
            }
            Algorithm::SCG => panic!("Not implemented yet!"),
            Algorithm::MDP => panic!("Not implemented yet!"),
        };
    }
}
