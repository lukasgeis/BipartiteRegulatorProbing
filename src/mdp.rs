use crate::{distributions::Distribution, *};
use std::fmt::Debug;

pub trait MDP<State: Debug, Action: Debug> {
    /// Get initial state of MDP
    fn get_initial_state(&self) -> State;

    /// Get Time Horizon
    fn get_time_horizon(&self) -> usize;

    /// Get all possible states
    fn get_states(&self) -> Vec<State>;

    /// Get all possible actions
    fn get_actions(&self) -> Vec<Action>;

    /// Get all possible actions in the current state
    fn get_possible_actions(&self, state: &State) -> Vec<Action>;

    /// Get all possible next states given the current state and an action (i.e. p_a(s,s') > 0)
    fn get_possible_transitions(&self, state: &State, action: &Action)
        -> Vec<(State, Probability)>;

    /// Get the reward for taking the given action in the given state
    fn get_reward(&self, state: &State, action: &Action) -> Reward;

    /// Get best action and the expected reward in the current state and time
    fn get_best_action(&mut self, state: &State, t: usize) -> Option<(Action, Reward)>;

    /// Calculate the best action and the expected reward in the current state and time
    fn calculate_optimal_action(&mut self, state: &State, t: usize) -> Option<(Action, Reward)> {
        if t == 0 {
            return None;
        }
        let mut optimal_action: Option<(Action, Reward)> = None;

        for action in self.get_possible_actions(&state) {
            let mut optimal_reward: Reward = self.get_reward(&state, &action);
            for next_state in self.get_possible_transitions(&state, &action) {
                let down_action: Option<(Action, Reward)> =
                    self.get_best_action(&next_state.0, t - 1);
                if down_action.is_some() {
                    optimal_reward += next_state.1 * down_action.unwrap().1;
                }
            }
            if (&optimal_action).is_none() || optimal_action.as_ref().unwrap().1 < optimal_reward {
                optimal_action = Some((action, optimal_reward));
            }
        }

        optimal_action
    }

    /// Calculate the optimal policy
    fn calculate_optimal_policy(&mut self) {
        self.get_best_action(&self.get_initial_state(), self.get_time_horizon());
    }
}

pub type ProbemaxState = (Vec<bool>, Vec<usize>);
pub type ProbemaxAction = usize;

pub struct ProbemaxMDP<'a> {
    bpr: &'a BipartiteRegulatorProbing,
    na: usize,
    vs: usize,
    k: usize,
    l: usize,
    states: Vec<ProbemaxState>,
    goal: GoalType,
    optimal_table: Vec<Vec<Option<(ProbemaxAction, Reward)>>>,
}

impl<'a> ProbemaxMDP<'a> {
    pub fn init(model: &'a BipartiteRegulatorProbing, goal: GoalType, k: usize, l: usize) -> Self {
        let na: usize = model.get_na();
        let vs: usize = match goal {
            GoalType::COV => panic!("That does not make sense!"),
            GoalType::MAX => model.get_vs(),
            GoalType::SUM => model.get_vs() * model.get_na(),
        };

        let mut all_states: Vec<ProbemaxState> = Vec::with_capacity((2 * (vs + 1)).pow(na as u32));
        let all_values: Vec<Vec<usize>> = combinations(na, vs + 1);
        for subset in boolean_combination(na) {
            for values in &all_values {
                all_states.push((subset.clone(), values.to_vec()));
            }
        }

        let number_states: usize = all_states.len();

        ProbemaxMDP {
            bpr: model,
            na: na,
            vs: vs,
            k: k,
            l: l,
            states: all_states,
            goal: goal,
            optimal_table: vec![vec![None; number_states]; na + 1],
        }
    }

    pub fn get_table(&self) -> Vec<Vec<Option<(ProbemaxAction, Reward)>>> {
        self.optimal_table.clone()
    }
}

pub fn index_of_state(state: &ProbemaxState, vs: usize) -> usize {
    let subset_index: usize = boolean_array_to_usize(&state.0);
    let values_index: usize = usize_array_to_usize(&state.1, vs);

    subset_index * (vs + 1).pow(state.0.len() as u32) + values_index
}

impl MDP<ProbemaxState, ProbemaxAction> for ProbemaxMDP<'_> {
    fn get_initial_state(&self) -> ProbemaxState {
        (vec![false; self.na], vec![0; self.na])
    }

    fn get_time_horizon(&self) -> usize {
        self.k
    }

    fn get_states(&self) -> Vec<ProbemaxState> {
        self.states.clone()
    }

    fn get_actions(&self) -> Vec<ProbemaxAction> {
        (1..(self.na + 1)).into_iter().collect()
    }

    fn get_possible_actions(&self, state: &ProbemaxState) -> Vec<ProbemaxAction> {
        self.get_actions()
            .into_iter()
            .filter(|a| !state.0[*a - 1])
            .collect()
    }

    fn get_possible_transitions(
        &self,
        state: &ProbemaxState,
        action: &ProbemaxAction,
    ) -> Vec<(ProbemaxState, Probability)> {
        if state.0[*action - 1] {
            return vec![(state.clone(), 1.0)];
        }

        let mut possible_transitions: Vec<(ProbemaxState, Probability)> = Vec::new();

        for next_state in &self.states {
            let mut is_possible: bool = true;
            for i in 0..self.na {
                if i != *action - 1
                    && (next_state.0[i] != state.0[i] || next_state.1[i] != state.1[i])
                {
                    is_possible = false;
                    break;
                }
            }
            if !is_possible || !next_state.0[*action - 1] {
                continue;
            }

            let prob: Probability = self.bpr.get_probemax(self.goal.clone()).unwrap()[*action - 1]
                .equal(next_state.1[*action - 1]);

            if prob > 0.0 {
                possible_transitions.push((next_state.clone(), prob));
            }
        }

        possible_transitions
    }

    fn get_reward(&self, state: &ProbemaxState, action: &ProbemaxAction) -> Reward {
        let mut ordered_values: Vec<usize> = state.1.clone();
        ordered_values.sort_by(|a, b| b.cmp(a));
        let min_value: usize = ordered_values[self.l - 1];
        if min_value == self.vs {
            return 0.0;
        }

        let distribution: &Distribution =
            &self.bpr.get_probemax(self.goal.clone()).unwrap()[*action - 1];
        let mut r: Reward = distribution.expected_greater(min_value + 1);
        if min_value > 0 {
            r *= distribution.greater(min_value + 1);
        }

        r
    }

    fn get_best_action(
        &mut self,
        state: &ProbemaxState,
        t: usize,
    ) -> Option<(ProbemaxAction, Reward)> {
        let index: usize = index_of_state(state, self.vs);
        if t > self.na {
            return None;
        } else if self.optimal_table[t][index].is_some() {
            return self.optimal_table[t][index];
        }
        let action: Option<(ProbemaxAction, Reward)> = self.calculate_optimal_action(state, t);
        self.optimal_table[t][index] = action;

        action
    }
}

/// Unit-Test with Example-Implementation for PrizeCollection
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_probemax_mdp() {
        let mut bpr: BipartiteRegulatorProbing = BipartiteRegulatorProbing {
            na: 3,
            nb: 1,
            vs: 5,
            name: "TEST".to_string(),
            edges: vec![],
            probemax: Some((
                vec![
                    Distribution {
                        v: 6,
                        exact_probabilities: vec![0.8, 0.0, 0.0, 0.0, 0.0, 0.2],
                        cumulative_probabilities: vec![0.8, 0.8, 0.8, 0.8, 0.8, 1.0],
                        expected_values: vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                    },
                    Distribution {
                        v: 6,
                        exact_probabilities: vec![0.5, 0.0, 0.0, 0.0, 0.5, 0.0],
                        cumulative_probabilities: vec![0.5, 0.5, 0.5, 0.5, 1.0, 1.0],
                        expected_values: vec![0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
                    },
                    Distribution {
                        v: 6,
                        exact_probabilities: vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
                        cumulative_probabilities: vec![0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
                        expected_values: vec![0.0, 0.0, 2.0, 2.0, 2.0, 2.0],
                    },
                ],
                vec![],
            )),
            non_adaptive_algorithms: vec![],
            optimal_adaptive_probemax: vec![],
        };

        let mut probemax_mdp: ProbemaxMDP = ProbemaxMDP::init(&mut bpr, GoalType::MAX, 2, 1);
        probemax_mdp.calculate_optimal_policy();

        let entries_to_check: Vec<(usize, ProbemaxState, Option<(usize, Reward)>)> = vec![
            (
                2,
                (vec![false, false, false], vec![0, 0, 0]),
                Some((2, 3.1)),
            ),
            (1, (vec![true, false, false], vec![0, 0, 0]), Some((2, 2.0))),
            (1, (vec![true, false, false], vec![5, 0, 0]), Some((2, 0.0))),
            (1, (vec![false, true, false], vec![0, 0, 0]), Some((3, 2.0))),
            (1, (vec![false, true, false], vec![0, 4, 0]), Some((1, 0.2))),
            (1, (vec![false, false, true], vec![0, 0, 0]), None),
            (1, (vec![false, false, true], vec![0, 0, 2]), Some((2, 1.0))),
        ];
        for entry in &entries_to_check {
            let proposed_solution: Option<(usize, Reward)> =
                probemax_mdp.optimal_table[entry.0][index_of_state(&entry.1, 5)];
            assert!(
                (entry.2.is_none() && proposed_solution.is_none())
                    || (entry.2.is_some() && proposed_solution.is_some())
            );
            if proposed_solution.is_some() {
                assert!(
                    entry.2.unwrap().0 == proposed_solution.unwrap().0
                        && is_close(entry.2.unwrap().1, proposed_solution.unwrap().1, None)
                );
            }
        }
    }
}
