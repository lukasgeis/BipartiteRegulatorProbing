use crate::helper::*;

pub trait MDP<State, Action> {
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

/// Unit-Test with Example-Implementation for PrizeCollection
mod tests {
    use super::*;

    type PrizeCollectionState = Option<Vec<bool>>;
    type PrizeCollectionAction = usize;

    struct PrizeCollectionMDP {
        n: usize,
        p: Vec<Probability>,
        r: Vec<Reward>,
        table: Vec<Vec<Option<(PrizeCollectionAction, Reward)>>>,
    }

    impl PrizeCollectionMDP {
        fn init(n: usize, probabilities: Vec<Probability>, rewards: Vec<Reward>) -> Self {
            PrizeCollectionMDP {
                n: n,
                p: probabilities,
                r: rewards,
                table: vec![vec![None; 2_usize.pow(n as u32) + 1]; 3],
            }
        }
    }

    impl MDP<PrizeCollectionState, PrizeCollectionAction> for PrizeCollectionMDP {
        fn get_initial_state(&self) -> PrizeCollectionState {
            Some(vec![false; self.n])
        }

        fn get_time_horizon(&self) -> usize {
            self.n
        }

        fn get_states(&self) -> Vec<PrizeCollectionState> {
            let mut states: Vec<PrizeCollectionState> = boolean_combination(self.n)
                .into_iter()
                .map(|x| Some(x))
                .collect();
            states.push(None);

            states
        }

        fn get_actions(&self) -> Vec<PrizeCollectionAction> {
            (1..(self.n + 1)).collect()
        }

        fn get_possible_actions(&self, state: &PrizeCollectionState) -> Vec<PrizeCollectionAction> {
            if state.is_none() {
                return vec![];
            }

            self.get_actions()
                .into_iter()
                .filter(|a| !state.as_ref().unwrap()[*a - 1])
                .collect()
        }

        fn get_possible_transitions(
            &self,
            state: &PrizeCollectionState,
            action: &PrizeCollectionAction,
        ) -> Vec<(PrizeCollectionState, Probability)> {
            if state.is_none() {
                return vec![(None, 1.0)];
            } else if state.as_ref().unwrap()[*action - 1] {
                return vec![(state.clone(), 1.0)];
            }

            let mut transitions: Vec<(PrizeCollectionState, Probability)> = Vec::with_capacity(2);

            // EMPTY state transition
            transitions.push((None, 1.0 - self.p[*action - 1]));

            // Value state transition
            let mut new_subset: Vec<bool> = state.as_ref().unwrap().clone();
            new_subset[*action - 1] = true;
            transitions.push((Some(new_subset), self.p[*action - 1]));

            transitions
        }

        fn get_reward(
            &self,
            state: &PrizeCollectionState,
            action: &PrizeCollectionAction,
        ) -> Reward {
            if state.is_none() {
                return 0.0;
            } else if state.as_ref().unwrap()[*action - 1] {
                return f64::NEG_INFINITY;
            }

            self.p[*action - 1] * self.r[*action - 1]
        }

        fn get_best_action(
            &mut self,
            state: &PrizeCollectionState,
            t: usize,
        ) -> Option<(PrizeCollectionAction, Reward)> {
            if t > self.n || state.is_none() {
                return None;
            } else if self.table[t][boolean_array_to_usize(state.as_ref().unwrap())].is_some() {
                return self.table[t][boolean_array_to_usize(state.as_ref().unwrap())];
            }
            let action: Option<(PrizeCollectionAction, Reward)> =
                self.calculate_optimal_action(state, t);
            self.table[t][boolean_array_to_usize(state.as_ref().unwrap())] = action;

            action
        }
    }

    #[test]
    fn example_prizecollection_mdp() {
        let mut instance: PrizeCollectionMDP =
            PrizeCollectionMDP::init(2, vec![0.01, 1.0], vec![1000.0, 1.0]);
        instance.calculate_optimal_policy();

        assert_eq!(instance.table[1][1].unwrap(), (2, 1.0));
        assert_eq!(instance.table[1][2].unwrap(), (1, 10.0));
        assert_eq!(instance.table[2][0].unwrap(), (2, 11.0));
    }
}
