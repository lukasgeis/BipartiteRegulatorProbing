use std::collections::BinaryHeap;

use rand::Rng;

use crate::{
    boolean_array_to_usize, distributions::DiscreteDistribution, model::BipartiteRegulatorProbing,
    usize_array_to_usize, Algorithm, GoalFunction, Probability, ProbemaxState, ProbingAction,
    Reward, Solution, Time,
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
            GoalFunction::COV => {}
            GoalFunction::MAX | GoalFunction::SUM => {
                let index: usize = (goal == GoalFunction::SUM) as usize;
                match algorithm {
                    Algorithm::ALL => {
                        let mut solutions: Vec<Solution> =
                            self.run_algorithm(goal.clone(), Algorithm::POLY, k, l);
                        solutions.append(&mut self.run_algorithm(goal, Algorithm::MDP, k, l));
                        return solutions;
                    }
                    Algorithm::POLY => {
                        let mut solutions: Vec<Solution> =
                            self.run_algorithm(goal.clone(), Algorithm::FAST, k, l);
                        solutions.append(&mut self.run_algorithm(goal, Algorithm::SCG, k, l));
                        return solutions;
                    }
                    Algorithm::FAST => {
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
                                .max_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
                                .unwrap()
                                .0;
                            probed_subset.push(argmax);
                            values_heap
                                .push(self.probemax_realizations[index].as_ref().unwrap()[argmax]);
                        }

                        let mut values = values_heap.into_sorted_vec();
                        values.truncate(l);

                        return vec![(
                            (goal, Algorithm::AMP, k, l),
                            time.elapsed().as_secs_f64(),
                            probed_subset,
                            values.into_iter().sum(),
                        )];
                    }
                    Algorithm::SCG => {
                        if self
                            .bpr
                            .get_algorithm((goal.clone(), Algorithm::SCG, k, l))
                            .is_none()
                        {
                            let time = std::time::Instant::now();
                            let delta: usize = 3 * k * self.bpr.get_na();
                            let mut y: Vec<Probability> = vec![0.0; self.bpr.get_na()];

                            for _ in 0..delta {
                                let mut w: Vec<usize> = vec![0; self.bpr.get_na()];
                                let samples: usize = 4
                                    * delta
                                    * delta
                                    * (1 + (self.bpr.get_na() as f64).ln() as usize
                                        - (0.5 * (delta as f64).ln()) as usize);
                                for _ in 0..samples {
                                    let mut sample: Vec<usize> =
                                        Vec::with_capacity(self.bpr.get_na());
                                    for i in 0..self.bpr.get_na() {
                                        if rand::thread_rng().gen::<Probability>() <= y[i] {
                                            sample
                                                .push(self.bpr.get_probemax(&goal)[i].draw_value())
                                        } else {
                                            sample.push(0);
                                        }
                                    }
                                    for i in 0..self.bpr.get_na() {
                                        let i_value: usize =
                                            self.bpr.get_probemax(&goal)[i].draw_value();
                                        if i_value > sample[i] {
                                            let mut sample_sorted: Vec<usize> = sample.clone();
                                            sample_sorted.sort_by(|a, b| b.cmp(a));
                                            if i_value > sample_sorted[l - 1] {
                                                w[i] += i_value - sample_sorted[l - 1];
                                            }
                                        }
                                    }
                                }
                                let mut best_weights: Vec<(usize, usize)> =
                                    w.into_iter().enumerate().collect();
                                best_weights.sort_by(|(_, a), (_, b)| b.cmp(a));
                                best_weights.truncate(k);
                                for (i, _) in best_weights {
                                    y[i] += 1.0 / (delta as f64);
                                }
                            }
                            let mut result: Vec<(usize, f64)> = y.into_iter().enumerate().collect();
                            result.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
                            result.truncate(k);

                            let (subset, _): (Vec<usize>, Vec<f64>) = result.into_iter().unzip();
                            self.bpr.add_non_adaptive_solution((
                                (goal.clone(), Algorithm::SCG, k, l),
                                time.elapsed().as_secs_f64(),
                                subset,
                                0,
                            ));
                        }
                        let time = std::time::Instant::now();
                        let order: Vec<usize> = self
                            .bpr
                            .get_algorithm((goal.clone(), Algorithm::SCG, k, l))
                            .as_ref()
                            .unwrap()
                            .2
                            .clone();
                        let mut result: Vec<usize> = order
                            .iter()
                            .map(|i| self.get_probemax_realization(*i, &goal))
                            .collect();
                        result.sort_by(|a, b| b.cmp(a));
                        result.truncate(l);
                        return vec![(
                            (goal.clone(), Algorithm::SCG, k, l),
                            time.elapsed().as_secs_f64()
                                + self
                                    .bpr
                                    .get_algorithm((goal, Algorithm::SCG, k, l))
                                    .as_ref()
                                    .unwrap()
                                    .1,
                            order,
                            result.into_iter().sum(),
                        )];
                    }
                    Algorithm::MDP => {
                        self.get_probemax_realization(0, &goal);
                        if self
                            .bpr
                            .get_optimal_adaptive_probemax_policy((
                                goal.clone(),
                                Algorithm::MDP,
                                k,
                                l,
                            ))
                            .is_none()
                        {
                            ProbemaxMDP::init(self.bpr, goal.clone(), k, l)
                                .calculate_optimal_policy();
                        }
                        let (optimum_table, mdp_time): (
                            &Vec<Vec<Option<(ProbingAction, Reward)>>>,
                            Time,
                        ) = self
                            .bpr
                            .get_optimal_adaptive_probemax_policy((
                                goal.clone(),
                                Algorithm::MDP,
                                k,
                                l,
                            ))
                            .unwrap();
                        let time = std::time::Instant::now();
                        let mut probed_subset: Vec<usize> = Vec::with_capacity(k);
                        let mut current_state: ProbemaxState =
                            (vec![false; self.bpr.get_na()], vec![0; self.bpr.get_na()]);
                        let vs: usize = match goal {
                            GoalFunction::COV => {
                                panic!("There is no ProbeMax-Reduction for Coverage!")
                            }
                            GoalFunction::MAX => self.bpr.get_vs(),
                            GoalFunction::SUM => self.bpr.get_vs() * self.bpr.get_na(),
                        };

                        for i in 0..k {
                            let next_action: Option<(ProbingAction, Reward)> =
                                optimum_table[k - i][state_to_index(&current_state, vs)];
                            if next_action.is_none() {
                                break;
                            }
                            let action: usize = next_action.unwrap().0;
                            probed_subset.push(action);
                            current_state.0[action - 1] = true;
                            current_state.1[action - 1] =
                                self.probemax_realizations[index].as_ref().unwrap()[action - 1];
                        }

                        let mut sorted_subset: Vec<usize> = probed_subset
                            .clone()
                            .into_iter()
                            .map(|a| self.probemax_realizations[index].as_ref().unwrap()[a - 1])
                            .collect();
                        sorted_subset.sort_by(|a, b| b.cmp(a));
                        sorted_subset.truncate(l);

                        return vec![(
                            (goal.clone(), Algorithm::MDP, k, l),
                            mdp_time + time.elapsed().as_secs_f64(),
                            probed_subset,
                            sorted_subset.into_iter().sum(),
                        )];
                    }
                };
            }
        };
        vec![]
    }
}

pub struct ProbemaxMDP<'a> {
    bpr: &'a mut BipartiteRegulatorProbing,
    na: usize,
    vs: usize,
    k: usize,
    l: usize,
    goal: GoalFunction,
    optimal_table: Vec<Vec<Option<(ProbingAction, Reward)>>>,
}

/// Get Index of State
pub fn state_to_index(state: &ProbemaxState, vs: usize) -> usize {
    let subset_index: usize = boolean_array_to_usize(&state.0);
    let values_index: usize = usize_array_to_usize(&state.1, vs);

    subset_index * (vs + 1).pow(state.0.len() as u32) + values_index
}

impl<'a> ProbemaxMDP<'a> {
    /// Create the ProbemaxMDP from a model and setting
    pub fn init(
        model: &'a mut BipartiteRegulatorProbing,
        goal: GoalFunction,
        k: usize,
        l: usize,
    ) -> Self {
        let na: usize = model.get_na();
        let vs: usize = match goal {
            GoalFunction::COV => panic!("There is no ProbeMax-Reduction for Coverage!"),
            GoalFunction::MAX => model.get_vs(),
            GoalFunction::SUM => model.get_vs() * model.get_na(),
        };

        Self {
            bpr: model,
            na: na,
            vs: vs,
            k: k,
            l: l,
            goal: goal,
            optimal_table: vec![vec![None; (2 * (vs + 1)).pow(na as u32)]; na + 1],
        }
    }

    /// Get the optimal table
    pub fn get_table(self) -> Vec<Vec<Option<(ProbingAction, Reward)>>> {
        self.optimal_table
    }

    /// Get all possible actions
    fn get_possible_actions(&self, state: &ProbemaxState) -> Vec<ProbingAction> {
        (1..(self.na + 1))
            .into_iter()
            .filter(|a| !state.0[*a - 1])
            .collect()
    }

    /// Get all possible transitions
    fn get_possible_transitions(
        &mut self,
        state: &ProbemaxState,
        action: ProbingAction,
    ) -> Vec<(ProbemaxState, Probability)> {
        if state.0[action - 1] {
            return vec![(state.clone(), 1.0)];
        }

        let mut possible_transitions: Vec<(ProbemaxState, Probability)> = Vec::new();

        let mut subset: Vec<bool> = state.0.clone();
        subset[action - 1] = true;

        for v in 0..self.vs {
            let p: Probability = self.bpr.get_probemax(&self.goal)[action - 1].equal(v);
            if p > 0.0 {
                let mut values: Vec<usize> = state.1.clone();
                values[action - 1] = v;
                possible_transitions.push(((subset.clone(), values), p));
            }
        }

        possible_transitions
    }

    /// Get the reward for taking an action in a state
    fn get_reward(&mut self, state: &ProbemaxState, action: ProbingAction) -> Reward {
        let mut ordered_values: Vec<usize> = state.1.clone();
        ordered_values.sort_by(|a, b| b.cmp(a));

        let min_value: usize = ordered_values[self.l - 1];
        if min_value == self.vs {
            return 0.0;
        }

        let distribution: &DiscreteDistribution = &self.bpr.get_probemax(&self.goal)[action - 1];
        let mut r: Reward = distribution.expected_greater(min_value + 1);
        if min_value > 0 {
            r *= distribution.greater(min_value + 1);
        }

        r
    }

    /// Get the best action for a state and time
    fn get_best_action(
        &mut self,
        state: &ProbemaxState,
        t: usize,
    ) -> Option<(ProbingAction, Reward)> {
        let index: usize = state_to_index(state, self.vs);
        if t > self.na || t == 0 {
            return None;
        } else if self.optimal_table[t][index].is_some() {
            return self.optimal_table[t][index];
        }
        let mut optimal_action: Option<(ProbingAction, Reward)> = None;
        for action in self.get_possible_actions(&state) {
            let mut optimal_reward: Reward = self.get_reward(&state, action);
            for next_state in self.get_possible_transitions(&state, action) {
                let down_action: Option<(ProbingAction, Reward)> =
                    self.get_best_action(&next_state.0, t - 1);
                if down_action.is_some() {
                    optimal_reward += next_state.1 * down_action.unwrap().1;
                }
            }
            if optimal_action.is_none() || optimal_action.as_ref().unwrap().1 < optimal_reward {
                optimal_action = Some((action, optimal_reward));
            }
        }

        self.optimal_table[t][index] = optimal_action;
        optimal_action
    }

    /// Calculates an optimal policy by calling the best action of the time horizon and initial state
    pub fn calculate_optimal_policy(mut self) {
        let time = std::time::Instant::now();
        self.get_best_action(&(vec![false; self.na], vec![0; self.na]), self.k);

        self.bpr.add_optimal_adaptive_probemax_policy(
            (self.goal.clone(), Algorithm::MDP, self.k, self.l),
            self.optimal_table,
            time.elapsed().as_secs_f64(),
        );
    }
}
