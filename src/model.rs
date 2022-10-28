use std::io::{BufRead, Error, ErrorKind};

use crate::{
    algorithms::namp_for_probemax,
    distributions::{max_distribution, sum_distribution, Distribution},
    Algorithm, GoalType, Probability,
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
    data: Vec<Vec<Distribution>>,
    /// Reductions to Top-l-ProbeMax
    reductions: Option<(Vec<Distribution>, Vec<Distribution>)>,
}

impl BipartiteRegulatorProbing {
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

        let mut model: BipartiteRegulatorProbing = BipartiteRegulatorProbing {
            na: na,
            nb: nb,
            vs: vs,
            name: name,
            data: data,
            reductions: None,
        };
        model.create_reductions();

        Ok(model)
    }

    /// Get Distribution of Edge
    pub fn get_distribution(&self, a: usize, b: usize) -> &Distribution {
        &self.data[a][b]
    }

    /// Create Reductions to Top-l-ProbeMax
    fn create_reductions(&mut self) {
        if self.reductions.is_none() {
            let mut new_reductions: (Vec<Distribution>, Vec<Distribution>) =
                (Vec::with_capacity(self.na), Vec::with_capacity(self.na));

            for a in &self.data {
                new_reductions.0.push(max_distribution(a));
                new_reductions.1.push(sum_distribution(a));
            }

            self.reductions = Some(new_reductions);
        }
    }

    /// Get the distribution of the reduction
    pub fn get_reduction(&self, goal: &GoalType) -> &Vec<Distribution> {
        match goal {
            GoalType::MAX => return &self.reductions.as_ref().unwrap().0,
            GoalType::SUM => return &self.reductions.as_ref().unwrap().1,
            GoalType::COV => panic!("There is no Coverage-Reduction!"),
        }
    }

    /// Creates an instance of the problem
    pub fn create_instance(&self) -> Instance {
        Instance {
            model: &self,
            realization: self
                .data
                .clone()
                .into_iter()
                .map(|a| -> Vec<usize> {
                    a.into_iter().map(|b| -> usize { b.draw_value() }).collect()
                })
                .collect(),
            results: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Instance<'a> {
    model: &'a BipartiteRegulatorProbing,
    realization: Vec<Vec<usize>>,
    results: Vec<(GoalType, Algorithm, Vec<usize>, usize)>,
}

impl Instance<'_> {
    pub fn get_realizations(&self) -> &Vec<Vec<usize>> {
        &self.realization
    }
    
    pub fn optimal_solution(&mut self, goal: GoalType, l: usize) {
        let mut regulators: Vec<(usize, usize)> = match goal {
            GoalType::MAX => self
                .realization
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, a)| (i, a.into_iter().max().unwrap_or(0)))
                .collect(),
            GoalType::SUM => self
                .realization
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, a)| (i, a.into_iter().sum()))
                .collect(),
            GoalType::COV => panic!("Not implemented yet!"),
        };
        regulators.sort_by(|(_, a), (_, b)| b.cmp(a));
        regulators.truncate(l);

        let (subset, values): (Vec<usize>, Vec<usize>) = regulators.into_iter().unzip();
        self.results
            .push((goal, Algorithm::OPT, subset, values.into_iter().sum()));
    }

    pub fn namp(&mut self, goal: GoalType, k: usize, l: usize) {
        let mut result: Vec<(usize, usize)> = namp_for_probemax(self.model.get_reduction(&goal), k)
            .into_iter()
            .map(|a| -> (usize, usize) {
                (
                    a,
                    match &goal {
                        GoalType::MAX => self.realization[a].clone().into_iter().max().unwrap_or(0),
                        GoalType::SUM => self.realization[a].clone().into_iter().sum(),
                        GoalType::COV => panic!("Not implemented yet!"),
                    },
                )
            })
            .collect();

        result.sort_by(|(_, a), (_, b)| b.cmp(a));

        let (subset, mut values): (Vec<usize>, Vec<usize>) = result.into_iter().unzip();
        values.truncate(l);

        self.results
            .push((goal, Algorithm::NAMP, subset, values.into_iter().sum()));
    }

    pub fn get_results(&self) -> &Vec<(GoalType, Algorithm, Vec<usize>, usize)> {
        &self.results
    }
}
