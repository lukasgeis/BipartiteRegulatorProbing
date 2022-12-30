use std::collections::BinaryHeap;

use rand::Rng;

use crate::{
    distributions::DiscreteDistribution, model::BipartiteRegulatorProbing, Algorithm, GoalFunction,
    Solution, Time,
};

#[derive(Debug)]
pub struct Instance<'a> {
    /// Reference to the BPR model
    bpr: &'a mut BipartiteRegulatorProbing,
    /// Realizations of Edges in this Instance
    realizations: Vec<Vec<usize>>,
    /// Probemax-Realizations for faster access
    probemax_realizations: [Option<Vec<usize>>; 2],
    /// Optimal Probemax-Ordering = sorted probemax_realizations
    opt_probemax: [Option<(Vec<(usize, usize)>, Time)>; 2],
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
            opt_probemax: [None, None],
            coding: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(5)
                .map(char::from)
                .collect(),
        }
    }

    /// Get the BPR-model
    pub fn get_model(&self) -> &BipartiteRegulatorProbing {
        &self.bpr
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
        let na: usize = self.bpr.get_na();

        match goal {
            GoalFunction::COV => {
                match algorithm {
                    Algorithm::OPT => {
                        let time = std::time::Instant::now();

                        let mut current_value: Vec<usize> = vec![0; self.bpr.get_nb()];
                        let mut optimal_subset: Vec<usize> = Vec::with_capacity(l);

                        for _ in 0..l {
                            let argmax: usize = (0..self.bpr.get_na())
                                .into_iter()
                                .filter(|i| !optimal_subset.contains(i))
                                .map(|a| -> (usize, usize) {
                                    let mut expected_reward: usize = 0;
                                    for b in 0..self.bpr.get_nb() {
                                        if self.get_realization(a, b) > current_value[b] {
                                            expected_reward +=
                                                self.get_realization(a, b) - current_value[b];
                                        }
                                    }
                                    (a, expected_reward)
                                })
                                .max_by(|(_, a), (_, b)| a.cmp(b))
                                .unwrap()
                                .0;
                            for b in 0..self.bpr.get_nb() {
                                if self.get_realization(argmax, b) > current_value[b] {
                                    current_value[b] = self.get_realization(argmax, b);
                                }
                            }
                            optimal_subset.push(argmax);
                        }
                        return vec![(
                            (GoalFunction::COV, Algorithm::OPT, k, l),
                            time.elapsed().as_secs_f64(),
                            optimal_subset,
                            current_value.into_iter().sum(),
                        )];
                    }
                    Algorithm::ALL => {
                        let mut solutions: Vec<Solution> =
                            self.run_algorithm(GoalFunction::COV, Algorithm::NAMP, k, l);
                        solutions.append(&mut self.run_algorithm(
                            GoalFunction::COV,
                            Algorithm::AMP,
                            k,
                            l,
                        ));
                        return solutions;
                    }
                    Algorithm::AMP => {
                        let time = std::time::Instant::now();

                        let mut current_value: Vec<usize> = vec![0; self.bpr.get_nb()];
                        let mut probed_subset: Vec<usize> = Vec::with_capacity(k);

                        for _ in 0..l {
                            let argmax: usize = (0..self.bpr.get_na())
                                .into_iter()
                                .filter(|i| !probed_subset.contains(i))
                                .map(|a| -> (usize, f64) {
                                    let mut expected_reward: f64 = 0.0;
                                    for b in 0..self.bpr.get_nb() {
                                        expected_reward += self
                                            .bpr
                                            .get_edge(a, b)
                                            .expected_greater(current_value[b] + 1);
                                    }
                                    (a, expected_reward)
                                })
                                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .0;
                            for b in 0..self.bpr.get_nb() {
                                if self.get_realization(argmax, b) > current_value[b] {
                                    current_value[b] = self.get_realization(argmax, b);
                                }
                            }
                            probed_subset.push(argmax);
                        }

                        if l == k {
                            return vec![(
                                (GoalFunction::COV, Algorithm::AMP, k, l),
                                time.elapsed().as_secs_f64(),
                                probed_subset,
                                current_value.into_iter().sum(),
                            )];
                        }

                        for _ in l..k {
                            let argmax: usize = (0..self.bpr.get_na())
                                .into_iter()
                                .filter(|i| !probed_subset.contains(i))
                                .map(|a| -> (usize, f64) {
                                    // Run Greedy Algorithm to evaluate Expected Value
                                    let mut temp_subset: Vec<usize> = probed_subset.clone();
                                    temp_subset.push(a);
                                    let mut test_subset: Vec<usize> = Vec::with_capacity(l);
                                    let mut test_values: Vec<f64> = vec![0.0; self.bpr.get_nb()];
                                    for _ in 0..l {
                                        let test_argmax: usize = temp_subset
                                            .iter()
                                            .filter(|i| !test_subset.contains(i))
                                            .map(|test_a| -> (usize, f64) {
                                                let mut test_reward: f64 = 0.0;
                                                for b in 0..self.bpr.get_nb() {
                                                    if *test_a != a {
                                                        if self.get_realization(*test_a, b) as f64
                                                            > test_values[b]
                                                        {
                                                            test_reward += (self
                                                                .get_realization(*test_a, b)
                                                                as f64
                                                                - test_values[b])
                                                                as f64;
                                                        }
                                                    } else {
                                                        if self
                                                            .bpr
                                                            .get_edge(*test_a, b)
                                                            .expected_value()
                                                            > test_values[b]
                                                        {
                                                            test_reward += self
                                                                .bpr
                                                                .get_edge(*test_a, b)
                                                                .expected_value()
                                                                - test_values[b];
                                                        }
                                                    }
                                                }
                                                (*test_a, test_reward)
                                            })
                                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                            .unwrap()
                                            .0;
                                        for b in 0..self.bpr.get_nb() {
                                            if test_argmax != a {
                                                if self.get_realization(test_argmax, b) as f64
                                                    > test_values[b]
                                                {
                                                    test_values[b] =
                                                        self.get_realization(test_argmax, b) as f64;
                                                }
                                            } else {
                                                if self
                                                    .bpr
                                                    .get_edge(test_argmax, b)
                                                    .expected_value()
                                                    > test_values[b]
                                                {
                                                    test_values[b] = self
                                                        .bpr
                                                        .get_edge(test_argmax, b)
                                                        .expected_value();
                                                }
                                            }
                                        }
                                        test_subset.push(test_argmax);
                                    }

                                    (a, test_values.into_iter().sum())
                                })
                                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .0;
                            probed_subset.push(argmax);
                        }

                        let mut final_values: Vec<usize> = vec![0; self.bpr.get_nb()];
                        let mut final_subset: Vec<usize> = Vec::with_capacity(l);
                        for _ in 0..l {
                            let argmax: usize = probed_subset
                                .iter()
                                .filter(|i| !final_subset.contains(i))
                                .map(|a| -> (usize, usize) {
                                    let mut expected_reward: usize = 0;
                                    for b in 0..self.bpr.get_nb() {
                                        if self.get_realization(*a, b) > final_values[b] {
                                            expected_reward +=
                                                self.get_realization(*a, b) - final_values[b];
                                        }
                                    }
                                    (*a, expected_reward)
                                })
                                .max_by(|(_, a), (_, b)| a.cmp(b))
                                .unwrap()
                                .0;
                            for b in 0..self.bpr.get_nb() {
                                if self.get_realization(argmax, b) > final_values[b] {
                                    final_values[b] = self.get_realization(argmax, b);
                                }
                            }
                            final_subset.push(argmax);
                        }

                        return vec![(
                            (GoalFunction::COV, Algorithm::AMP, k, l),
                            time.elapsed().as_secs_f64(),
                            probed_subset,
                            final_values.into_iter().sum(),
                        )];
                    }
                    Algorithm::NAMP => {
                        if let Some(sol) =
                            self.bpr
                                .get_algorithm((GoalFunction::COV, Algorithm::NAMP, k, l))
                        {
                            let time = std::time::Instant::now();
                            let mut final_values: Vec<usize> = vec![0; self.bpr.get_nb()];
                            let mut final_subset: Vec<usize> = Vec::with_capacity(l);
                            for _ in 0..l {
                                let argmax: usize = sol
                                    .2
                                    .iter()
                                    .filter(|i| !final_subset.contains(i))
                                    .map(|a| -> (usize, usize) {
                                        let mut expected_reward: usize = 0;
                                        for b in 0..self.bpr.get_nb() {
                                            if self.get_realization(*a, b) > final_values[b] {
                                                expected_reward +=
                                                    self.get_realization(*a, b) - final_values[b];
                                            }
                                        }
                                        (*a, expected_reward)
                                    })
                                    .max_by(|(_, a), (_, b)| a.cmp(b))
                                    .unwrap()
                                    .0;
                                for b in 0..self.bpr.get_nb() {
                                    if self.get_realization(argmax, b) > final_values[b] {
                                        final_values[b] = self.get_realization(argmax, b);
                                    }
                                }
                                final_subset.push(argmax);
                            }

                            return vec![(
                                (GoalFunction::COV, Algorithm::NAMP, k, l),
                                time.elapsed().as_secs_f64(),
                                sol.2.clone(),
                                final_values.into_iter().sum(),
                            )];
                        }

                        let time = std::time::Instant::now();

                        let mut current_value: Vec<f64> = vec![0.0; self.bpr.get_nb()];
                        let mut probed_subset: Vec<usize> = Vec::with_capacity(k);

                        for _ in 0..l {
                            let argmax: usize = (0..self.bpr.get_na())
                                .into_iter()
                                .filter(|i| !probed_subset.contains(i))
                                .map(|a| -> (usize, f64) {
                                    let mut expected_reward: f64 = 0.0;
                                    for b in 0..self.bpr.get_nb() {
                                        expected_reward +=
                                            self.bpr.get_edge(a, b).expected_greater(
                                                current_value[b].floor() as usize + 1,
                                            );
                                    }
                                    (a, expected_reward)
                                })
                                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .0;
                            for b in 0..self.bpr.get_nb() {
                                if self.bpr.get_edge(argmax, b).expected_value() > current_value[b]
                                {
                                    current_value[b] =
                                        self.bpr.get_edge(argmax, b).expected_value();
                                }
                            }
                            probed_subset.push(argmax);
                        }

                        if l == k {
                            self.bpr.add_non_adaptive_solution((
                                (GoalFunction::COV, Algorithm::NAMP, k, l),
                                time.elapsed().as_secs_f64(),
                                probed_subset.clone(),
                                0,
                            ));
                            let mut value_obtained: usize = 0;
                            for b in 0..self.bpr.get_nb() {
                                let mut max_value: usize = 0;
                                for a in &probed_subset {
                                    if self.get_realization(*a, b) > max_value {
                                        max_value = self.get_realization(*a, b);
                                    }
                                }
                                value_obtained += max_value;
                            }

                            return vec![(
                                (GoalFunction::COV, Algorithm::NAMP, k, l),
                                time.elapsed().as_secs_f64(),
                                probed_subset.clone(),
                                value_obtained,
                            )];
                        }

                        for _ in l..k {
                            let argmax: usize = (0..self.bpr.get_na())
                                .into_iter()
                                .filter(|i| !probed_subset.contains(i))
                                .map(|a| -> (usize, f64) {
                                    (
                                        a,
                                        (0..self.bpr.get_nb())
                                            .into_iter()
                                            .map(|b| self.bpr.get_edge(a, b).expected_value())
                                            .sum(),
                                    )
                                })
                                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .0;
                            probed_subset.push(argmax);
                        }

                        self.bpr.add_non_adaptive_solution((
                            (GoalFunction::COV, Algorithm::NAMP, k, l),
                            time.elapsed().as_secs_f64(),
                            probed_subset.clone(),
                            0,
                        ));

                        let mut final_values: Vec<usize> = vec![0; self.bpr.get_nb()];
                        let mut final_subset: Vec<usize> = Vec::with_capacity(l);
                        for _ in 0..l {
                            let argmax: usize = probed_subset
                                .iter()
                                .filter(|i| !final_subset.contains(i))
                                .map(|a| -> (usize, usize) {
                                    let mut expected_reward: usize = 0;
                                    for b in 0..self.bpr.get_nb() {
                                        if self.get_realization(*a, b) > final_values[b] {
                                            expected_reward +=
                                                self.get_realization(*a, b) - final_values[b];
                                        }
                                    }
                                    (*a, expected_reward)
                                })
                                .max_by(|(_, a), (_, b)| a.cmp(b))
                                .unwrap()
                                .0;
                            for b in 0..self.bpr.get_nb() {
                                if self.get_realization(argmax, b) > final_values[b] {
                                    final_values[b] = self.get_realization(argmax, b);
                                }
                            }
                            final_subset.push(argmax);
                        }

                        return vec![(
                            (GoalFunction::COV, Algorithm::NAMP, k, l),
                            time.elapsed().as_secs_f64(),
                            probed_subset,
                            final_values.into_iter().sum(),
                        )];
                    }
                }
            }
            GoalFunction::MAX | GoalFunction::SUM => {
                let index: usize = (goal == GoalFunction::SUM) as usize;
                match algorithm {
                    Algorithm::ALL => {
                        let mut solutions: Vec<Solution> =
                            self.run_algorithm(goal.clone(), Algorithm::NAMP, k, l);
                        solutions.append(&mut self.run_algorithm(goal, Algorithm::AMP, k, l));
                        return solutions;
                    }
                    Algorithm::OPT => {
                        self.get_probemax_realization(0, &goal);
                        if self.opt_probemax[index].is_none() {
                            let time = std::time::Instant::now();
                            let mut sorted_ordering: Vec<(usize, usize)> = self
                                .probemax_realizations[index]
                                .clone()
                                .unwrap()
                                .into_iter()
                                .enumerate()
                                .collect();
                            sorted_ordering.sort_by(|(_, a), (_, b)| b.cmp(a));
                            self.opt_probemax[index] =
                                Some((sorted_ordering, time.elapsed().as_secs_f64()));
                        }
                        let time = std::time::Instant::now();
                        let mut subset: Vec<usize> = Vec::with_capacity(l);
                        let mut value: usize = 0;
                        for i in 0..l {
                            let entry: (usize, usize) =
                                self.opt_probemax[index].as_ref().unwrap().0[i];
                            subset.push(entry.0);
                            value += entry.1;
                        }
                        return vec![(
                            (goal, Algorithm::OPT, k, l),
                            time.elapsed().as_secs_f64()
                                + self.opt_probemax[index].as_ref().unwrap().1,
                            subset,
                            value,
                        )];
                    }
                    Algorithm::NAMP => {
                        if self
                            .bpr
                            .get_algorithm((
                                goal.clone(),
                                Algorithm::NAMP,
                                self.bpr.get_na(),
                                self.bpr.get_na(),
                            ))
                            .is_none()
                        {
                            let time = std::time::Instant::now();
                            let mut ordering: Vec<(usize, f64)> = self
                                .bpr
                                .get_probemax(&goal)
                                .into_iter()
                                .enumerate()
                                .map(|(i, d)| (i, d.expected_value()))
                                .collect();
                            ordering.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                            self.bpr.add_non_adaptive_solution((
                                (
                                    goal.clone(),
                                    Algorithm::NAMP,
                                    self.bpr.get_na(),
                                    self.bpr.get_na(),
                                ),
                                time.elapsed().as_secs_f64(),
                                ordering.into_iter().map(|(i, _)| i).collect(),
                                0,
                            ));
                        }
                        let time = std::time::Instant::now();
                        let mut order: Vec<usize> = self
                            .bpr
                            .get_algorithm((
                                goal.clone(),
                                Algorithm::NAMP,
                                self.bpr.get_na(),
                                self.bpr.get_na(),
                            ))
                            .as_ref()
                            .unwrap()
                            .2
                            .clone();
                        order.truncate(k);
                        let mut result: Vec<usize> = order
                            .iter()
                            .map(|i| self.get_probemax_realization(*i, &goal))
                            .collect();
                        result.sort_by(|a, b| b.cmp(a));
                        result.truncate(l);
                        return vec![(
                            (goal.clone(), Algorithm::NAMP, k, l),
                            time.elapsed().as_secs_f64()
                                + self
                                    .bpr
                                    .get_algorithm((
                                        goal,
                                        Algorithm::NAMP,
                                        self.bpr.get_na(),
                                        self.bpr.get_na(),
                                    ))
                                    .as_ref()
                                    .unwrap()
                                    .1,
                            order,
                            result.into_iter().sum(),
                        )];
                    }
                    Algorithm::AMP => {
                        let time = std::time::Instant::now();
                        let mut simple_order: Vec<(usize, f64)> = self
                            .bpr
                            .get_probemax(&goal)
                            .into_iter()
                            .enumerate()
                            .map(|(i, d)| (i, d.expected_value()))
                            .collect();
                        simple_order.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                        simple_order.truncate(l);

                        let (mut probed_subset, values_unsorted): (Vec<usize>, Vec<usize>) =
                            simple_order
                                .into_iter()
                                .map(|(i, _)| (i, self.get_probemax_realization(i, &goal)))
                                .unzip();
                        let mut values_heap = BinaryHeap::from(values_unsorted);
                        let distributions: &Vec<DiscreteDistribution> =
                            self.bpr.get_probemax(&goal);
                        for _ in 0..(k - l) {
                            let values: Vec<usize> = values_heap.clone().into_sorted_vec();
                            let argmax: usize = (0..na)
                                .into_iter()
                                .filter(|i| !probed_subset.contains(i))
                                .map(|i| {
                                    (
                                        i,
                                        distributions[i]
                                            .expected_greater(values[values.len() - l] + 1),
                                    )
                                })
                                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                                .unwrap()
                                .0;
                            probed_subset.push(argmax);
                            values_heap
                                .push(self.probemax_realizations[index].as_ref().unwrap()[argmax]);
                        }

                        let mut values: Vec<usize> =
                            values_heap.into_sorted_vec().into_iter().rev().collect();
                        values.truncate(l);

                        return vec![(
                            (goal, Algorithm::AMP, k, l),
                            time.elapsed().as_secs_f64(),
                            probed_subset,
                            values.into_iter().sum(),
                        )];
                    }
                };
            }
        };
    }
}
