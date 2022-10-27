use std::io::{BufRead, Error, ErrorKind};

use crate::{
    distributions::{max_distribution, sum_distribution, Distribution},
    Probability,
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

        Ok(BipartiteRegulatorProbing {
            na: na,
            nb: nb,
            vs: vs,
            name: name,
            data: data,
        })
    }

    /// Get Distribution of Edge
    pub fn get_distribution(&self, a: usize, b: usize) -> &Distribution {
        &self.data[a][b]
    }

    pub fn top_l_probemax(&self, goal: Reduction) -> ToplProbeMax {
        ToplProbeMax {
            n: self.na,
            data: self
                .data
                .clone()
                .into_iter()
                .map(|v| -> Distribution {
                    match goal {
                        Reduction::MAX => return max_distribution(&v),
                        Reduction::SUM => return sum_distribution(&v),
                    };
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct ToplProbeMax {
    n: usize,
    data: Vec<Distribution>,
}

pub enum Reduction {
    MAX,
    SUM,
}
