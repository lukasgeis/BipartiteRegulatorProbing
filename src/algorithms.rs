use std::collections::BinaryHeap;

use rand::Rng;

use crate::{model::BipartiteRegulatorProbing, Algorithm, GoalFunction, Solution};

#[derive(Debug)]
pub struct Instance<'a> {
    /// Reference to the BPR model
    bpr: &'a mut BipartiteRegulatorProbing,
    /// Realizations of Edges in this Instance
    realizations: Vec<Vec<usize>>,
    /// Probemax-Realizations for faster access
    probemax_realizations: [Option<Vec<usize>>; 2],
    /// Hash-Coding to identify this specific Instance
    coding: String,
}

impl<'a> Instance<'a> {
    /// Create an Instance from a BPR model
    pub fn new(model: &'a mut BipartiteRegulatorProbing) -> Self {
        let mut realizations: Vec<Vec<usize>> = Vec::with_capacity(model.get_na());
        for a in 0..model.get_na() {
            let mut regulator_realizations: Vec<usize> = Vec::with_capacity(model.get_nb());
            for b in 0..model.get_nb() {
                regulator_realizations.push(model.get_edge(a, b).draw_value());
            }
            realizations.push(regulator_realizations);
        }

        Self {
            bpr: model,
            realizations: realizations,
            probemax_realizations: [None, None],
            coding: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        }
    }

    /// Get the BPR-model
    pub fn get_model(&mut self) -> &mut BipartiteRegulatorProbing {
        &mut self.bpr
    }

    /// Get an Edge-Realization
    pub fn get_realization(&self, a: usize, b: usize) -> usize {
        self.realizations[a][b]
    }

    /// Get the ProbeMax-Realization of this Instance or compute it if not computed already
    pub fn get_probemax_realization(&mut self, a: usize, goal: &GoalFunction) -> usize {
        match goal {
            GoalFunction::COV => panic!("There is no ProbeMax-Variant of Coverage"),
            GoalFunction::MAX => {
                if self.probemax_realizations[0].is_none() {
                    self.probemax_realizations[0] = Some(
                        self.realizations
                            .iter()
                            .map(|a| *a.into_iter().max().unwrap_or(&0))
                            .collect(),
                    );
                }
                return self.probemax_realizations[0].as_ref().unwrap()[a];
            }
            GoalFunction::SUM => {
                if self.probemax_realizations[1].is_none() {
                    self.probemax_realizations[1] = Some(
                        self.realizations
                            .iter()
                            .map(|a| a.into_iter().sum::<usize>())
                            .collect(),
                    );
                }
                return self.probemax_realizations[1].as_ref().unwrap()[a];
            }
        }
    }

    /// Get the Hash-Coding of the Instance
    pub fn get_coding(&self) -> &String {
        &self.coding
    }

    /// Run an algorithm on this instance and return the solution
    pub fn run_algorithm(
        &mut self,
        goal: GoalFunction,
        algorithm: Algorithm,
        k: usize,
        l: usize,
    ) -> Vec<Solution> {
        if (l > k && algorithm != Algorithm::OPT) || l > self.bpr.get_na() || k > self.bpr.get_na()
        {
            return vec![];
        }

        match goal {
            GoalFunction::COV => match algorithm {
                Algorithm::OPT => return vec![opt_cov(self, l)],
                Algorithm::AMP => return vec![amp_cov(self, k, l)],
                Algorithm::NAMP => return vec![namp_cov(self, k, l)],
                Algorithm::ALL => return vec![amp_cov(self, k, l), namp_cov(self, k, l)],
            },
            GoalFunction::MAX | GoalFunction::SUM => {
                // Compute ProbeMax Realizations if not done already
                self.get_probemax_realization(0, &goal);
                match algorithm {
                    Algorithm::OPT => return vec![opt_pm(self, goal, l)],
                    Algorithm::AMP => return vec![amp_pm(self, goal, k, l)],
                    Algorithm::NAMP => return vec![namp_pm(self, goal, k, l)],
                    Algorithm::ALL => {
                        return vec![amp_pm(self, goal.clone(), k, l), namp_pm(self, goal, k, l)]
                    }
                }
            }
        };
    }
}

fn opt_pm(ins: &mut Instance, goal: GoalFunction, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Get Realizations
    let mut sorted_ordering: Vec<(usize, usize)> = (0..ins.get_model().get_na())
        .into_iter()
        .map(|i| (i, ins.get_probemax_realization(i, &goal)))
        .collect();

    // Sort Realizations in descending order and pick l highest ones
    sorted_ordering.sort_by(|(_, a), (_, b)| b.cmp(a));
    sorted_ordering.truncate(l);

    // Get Value
    let value: usize = sorted_ordering.iter().map(|(_, v)| v).sum();

    // Return Solution
    (
        (goal, Algorithm::OPT, 0, l),
        time.elapsed().as_secs_f64(),
        sorted_ordering.into_iter().map(|(i, _)| i).collect(),
        value,
    )
}

fn amp_pm(ins: &mut Instance, goal: GoalFunction, k: usize, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Get Expected Values
    let mut first_order: Vec<(usize, f64)> = ins
        .get_model()
        .get_probemax(&goal)
        .into_iter()
        .enumerate()
        .map(|(i, d)| (i, d.expected_value()))
        .collect();

    // Sort Expected Values in descending order and pick l highest ones
    first_order.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    first_order.truncate(l);

    // Probe first l Regulators
    let (mut probed_subset, values_unsorted): (Vec<usize>, Vec<usize>) = first_order
        .iter()
        .map(|(i, _)| (*i, ins.get_probemax_realization(*i, &goal)))
        .unzip();

    // Create Heap from Values for faster access of l highest element
    let mut values_heap = BinaryHeap::from(values_unsorted);

    // Last (k - l) probes
    for _ in 0..(k - l) {
        // Get l highest value
        let lval: usize = values_heap.clone().into_sorted_vec()[probed_subset.len() - l];

        // Compute next probe
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !probed_subset.contains(i))
            .map(|i| {
                (
                    i,
                    ins.get_model().get_probemax(&goal)[i].expected_greater(lval),
                )
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        // Probe argmax
        probed_subset.push(argmax);
        values_heap.push(ins.get_probemax_realization(argmax, &goal));
    }

    // Compute final values
    let mut values: Vec<usize> = values_heap.into_sorted_vec().into_iter().rev().collect();
    values.truncate(l);

    // Return Solution
    (
        (goal, Algorithm::AMP, k, l),
        time.elapsed().as_secs_f64(),
        probed_subset,
        values.into_iter().sum(),
    )
}

fn namp_pm(ins: &mut Instance, goal: GoalFunction, k: usize, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Get Expected Values
    let mut probe_order: Vec<(usize, f64)> = ins
        .get_model()
        .get_probemax(&goal)
        .into_iter()
        .enumerate()
        .map(|(i, d)| (i, d.expected_value()))
        .collect();

    // Sort Expected Values in descending order and pick k highest ones
    probe_order.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    probe_order.truncate(k);

    // Probe Regulators
    let (subset, mut values): (Vec<usize>, Vec<usize>) = probe_order
        .into_iter()
        .map(|(i, _)| (i, ins.get_probemax_realization(i, &goal)))
        .unzip();

    // Compute final values
    values.sort_by(|a, b| b.cmp(a));
    values.truncate(l);

    // Return Solution
    (
        (goal, Algorithm::NAMP, k, l),
        time.elapsed().as_secs_f64(),
        subset,
        values.into_iter().sum(),
    )
}

fn opt_cov(ins: &mut Instance, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Current States
    let mut current_values: Vec<usize> = vec![0; ins.get_model().get_nb()];
    let mut optimal_subset: Vec<usize> = Vec::with_capacity(l);

    // Greedy Procedure
    for _ in 0..l {
        // Compute next Regulator to pick
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !optimal_subset.contains(i))
            .map(|a| -> (usize, usize) {
                // Increase in Value when chosing a
                let mut possible_reward: usize = 0;
                for b in 0..ins.get_model().get_nb() {
                    if ins.get_realization(a, b) > current_values[b] {
                        possible_reward += ins.get_realization(a, b) - current_values[b];
                    }
                }

                // Return (Regulator, Value) pair
                (a, possible_reward)
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;

        // Pick argmax and update Values
        optimal_subset.push(argmax);
        for b in 0..ins.get_model().get_nb() {
            if ins.get_realization(argmax, b) > current_values[b] {
                current_values[b] = ins.get_realization(argmax, b);
            }
        }
    }

    // Return Solution
    (
        (GoalFunction::COV, Algorithm::OPT, 0, l),
        time.elapsed().as_secs_f64(),
        optimal_subset,
        current_values.into_iter().sum(),
    )
}

fn amp_cov(ins: &mut Instance, k: usize, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Current States
    let mut current_value: Vec<usize> = vec![0; ins.get_model().get_nb()];
    let mut probed_subset: Vec<usize> = Vec::with_capacity(k);

    // First l probes
    for _ in 0..l {
        // Compute next Regulator to probe
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !probed_subset.contains(i))
            .map(|a| -> (usize, f64) {
                // Compute expected reward when probing a
                let mut expected_reward: f64 = 0.0;
                for b in 0..ins.get_model().get_nb() {
                    expected_reward += ins
                        .get_model()
                        .get_edge(a, b)
                        .expected_greater(current_value[b] + 1);
                }

                // Return (Regulator, Reward) pair
                (a, expected_reward)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        // Probe argmax and update Values
        probed_subset.push(argmax);
        for b in 0..ins.get_model().get_nb() {
            if ins.get_realization(argmax, b) > current_value[b] {
                current_value[b] = ins.get_realization(argmax, b);
            }
        }
    }

    // If k = l, then stop and return
    if l == k {
        return (
            (GoalFunction::COV, Algorithm::AMP, k, l),
            time.elapsed().as_secs_f64(),
            probed_subset,
            current_value.into_iter().sum(),
        );
    }

    // Final (k - l) probes
    for _ in 0..(k - l) {
        // Compute next Regulator to probe
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !probed_subset.contains(i))
            .map(|a| -> (usize, f64) {
                // Possible Regulators
                let mut ext_subset: Vec<usize> = probed_subset.clone();
                ext_subset.push(a);

                // Greedy algorithm to evaluate a
                let mut temp_subset: Vec<usize> = Vec::with_capacity(l);
                let mut temp_values: Vec<f64> = vec![0.0; ins.get_model().get_nb()];

                // Run Greedy Algorithm
                for _ in 0..l {
                    // Compute next Temp-Regulator to pick
                    let temp_argmax: usize = ext_subset
                        .iter()
                        .filter(|i| !temp_subset.contains(i))
                        .map(|temp_a| -> (usize, f64) {
                            // Compute expected reward
                            let mut expected_reward: f64 = 0.0;
                            for b in 0..ins.get_model().get_nb() {
                                match *temp_a == a {
                                    // If we have not probed temp_a yet
                                    true => {
                                        if ins.get_model().get_edge(*temp_a, b).expected_value()
                                            > temp_values[b]
                                        {
                                            expected_reward += ins
                                                .get_model()
                                                .get_edge(*temp_a, b)
                                                .expected_value()
                                                - temp_values[b];
                                        }
                                    }
                                    // If we already probed temp_a
                                    false => {
                                        if ins.get_realization(*temp_a, b) as f64 > temp_values[b] {
                                            expected_reward += ins.get_realization(*temp_a, b)
                                                as f64
                                                - temp_values[b];
                                        }
                                    }
                                }
                            }

                            // Return (Regulator, Reward) pair
                            (*temp_a, expected_reward)
                        })
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .unwrap()
                        .0;

                    // Pick argmax and update Values
                    temp_subset.push(temp_argmax);
                    for b in 0..ins.get_model().get_nb() {
                        match temp_argmax == a {
                            // If we have not probed temp_argmax yet
                            true => {
                                if ins.get_model().get_edge(temp_argmax, b).expected_value()
                                    > temp_values[b]
                                {
                                    temp_values[b] =
                                        ins.get_model().get_edge(temp_argmax, b).expected_value();
                                }
                            }
                            // If we already probed temp_argmax
                            false => {
                                if ins.get_realization(temp_argmax, b) as f64 > temp_values[b] {
                                    temp_values[b] = ins.get_realization(temp_argmax, b) as f64;
                                }
                            }
                        };
                    }
                }

                // Return (Regulator, Reward) pair
                (a, temp_values.into_iter().sum())
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        // Probe argmax
        probed_subset.push(argmax);
    }

    // Final Greedy Algorithm
    let mut final_values: Vec<usize> = vec![0; ins.get_model().get_nb()];
    let mut final_subset: Vec<usize> = Vec::with_capacity(l);

    // Run Greedy Algorithm
    for _ in 0..l {
        // Compute next Regulator to choose
        let argmax: usize = probed_subset
            .iter()
            .filter(|i| !final_subset.contains(i))
            .map(|a| -> (usize, usize) {
                // Compute possible reward
                let mut possible_reward: usize = 0;
                for b in 0..ins.get_model().get_nb() {
                    if ins.get_realization(*a, b) > final_values[b] {
                        possible_reward += ins.get_realization(*a, b) - final_values[b];
                    }
                }

                // Return (Regulator, Reward) pair
                (*a, possible_reward)
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;

        // Probe argmax and update Values
        final_subset.push(argmax);
        for b in 0..ins.get_model().get_nb() {
            if ins.get_realization(argmax, b) > final_values[b] {
                final_values[b] = ins.get_realization(argmax, b);
            }
        }
    }

    // Return Solution
    (
        (GoalFunction::COV, Algorithm::AMP, k, l),
        time.elapsed().as_secs_f64(),
        probed_subset,
        final_values.into_iter().sum(),
    )
}

fn namp_cov(ins: &mut Instance, k: usize, l: usize) -> Solution {
    // Init Time
    let time = std::time::Instant::now();

    // Current states
    let mut current_value: Vec<f64> = vec![0.0; ins.get_model().get_nb()];
    let mut probed_subset: Vec<usize> = Vec::with_capacity(k);

    // First l probes
    for _ in 0..l {
        // Compute next Regulator to probe
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !probed_subset.contains(i))
            .map(|a| -> (usize, f64) {
                // Computed expected reward for probing a
                let mut expected_reward: f64 = 0.0;
                for b in 0..ins.get_model().get_nb() {
                    expected_reward += ins
                        .get_model()
                        .get_edge(a, b)
                        .expected_greater(current_value[b].floor() as usize + 1);
                }
                (a, expected_reward)
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;

        // Add argmax to probed Regulators
        probed_subset.push(argmax);
        for b in 0..ins.get_model().get_nb() {
            if ins.get_model().get_edge(argmax, b).expected_value() > current_value[b] {
                current_value[b] = ins.get_model().get_edge(argmax, b).expected_value();
            }
        }
    }

    // If l = k, stop and return
    if l == k {
        // Probe all Regulators to obtain value
        let mut value_obtained: usize = 0;
        for b in 0..ins.get_model().get_nb() {
            let mut max_value: usize = 0;
            for a in &probed_subset {
                if ins.get_realization(*a, b) > max_value {
                    max_value = ins.get_realization(*a, b);
                }
            }
            value_obtained += max_value;
        }

        // Return Solution
        return (
            (GoalFunction::COV, Algorithm::NAMP, k, l),
            time.elapsed().as_secs_f64(),
            probed_subset.clone(),
            value_obtained,
        );
    }

    // Final (k - l) probes
    for _ in l..k {
        // Compute next Regulator to probe
        let argmax: usize = (0..ins.get_model().get_na())
            .into_iter()
            .filter(|i| !probed_subset.contains(i))
            .map(|a| -> (usize, f64) {
                // Return (Regulator, Expected Coverage) pair
                (
                    a,
                    (0..ins.get_model().get_nb())
                        .into_iter()
                        .map(|b| ins.get_model().get_edge(a, b).expected_value())
                        .sum(),
                )
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        // Probe argmax
        probed_subset.push(argmax);
    }

    // Final Greedy Algorithm States
    let mut final_values: Vec<usize> = vec![0; ins.get_model().get_nb()];
    let mut final_subset: Vec<usize> = Vec::with_capacity(l);

    // Run Greedy Algorithm
    for _ in 0..l {
        // Compute next Regulator to pick
        let argmax: usize = probed_subset
            .iter()
            .filter(|i| !final_subset.contains(i))
            .map(|a| -> (usize, usize) {
                // Compute Possible Reward
                let mut possible_reward: usize = 0;
                for b in 0..ins.get_model().get_nb() {
                    if ins.get_realization(*a, b) > final_values[b] {
                        possible_reward += ins.get_realization(*a, b) - final_values[b];
                    }
                }

                // Return (Regulator, Reward) pair
                (*a, possible_reward)
            })
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;

        // Pick argmax and update Values
        final_subset.push(argmax);
        for b in 0..ins.get_model().get_nb() {
            if ins.get_realization(argmax, b) > final_values[b] {
                final_values[b] = ins.get_realization(argmax, b);
            }
        }
    }

    // Return Solution
    (
        (GoalFunction::COV, Algorithm::NAMP, k, l),
        time.elapsed().as_secs_f64(),
        probed_subset,
        final_values.into_iter().sum(),
    )
}
