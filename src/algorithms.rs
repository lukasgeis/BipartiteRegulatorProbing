use std::{collections::BinaryHeap, time::Instant};

use ez_bitset::bitset::*;

use crate::{
    distributions::WeightedDistribution,
    model::{BipartiteRegulatorProbing, ProbeMax, ProbeMaxInstance, Instance},
};

impl ProbeMax {
    /// Computes a Non-Adaptive Policy for this ProbeMax Instance
    pub fn compute_namp_policy(boxes: &Vec<WeightedDistribution>) -> (Vec<usize>, f64) {
        let timer = Instant::now();

        let mut exp_pairs: Vec<(usize, f64)> = boxes
            .iter()
            .enumerate()
            .map(|(i, b)| (i, b.expected_value()))
            .collect();
        exp_pairs.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        (exp_pairs.into_iter().map(|(i, _)| i).collect(), timer.elapsed().as_secs_f64())
    }
}

impl ProbeMaxInstance<'_> {
    pub fn adaptive_policy(&self, k: usize, l: usize) -> (usize, f64) {
        let timer = Instant::now();

        let n = self.get_probemax().get_n();
        
        let mut probed_subset: Vec<usize> = self.get_probemax().get_policy().iter().copied().take(l).collect();
        if k == l {
            return (probed_subset.iter().map(|x| self.get_realization(*x)).sum(), self.get_probemax().get_policy_time() + timer.elapsed().as_secs_f64());
        }

        let mut unprobed_regulators = BitSet::new_all_set_but(n, probed_subset.clone());
        let mut values_heap = BinaryHeap::from(probed_subset.iter().map(|x| self.get_realization(*x)).collect::<Vec<usize>>());

        while probed_subset.len() < k {
            let lval: usize = values_heap.clone().into_iter_sorted().take(l).last().unwrap();

            let argmax: usize = unprobed_regulators.iter().map(|x| -> (usize, f64) {
                (x, self.get_probemax().get_box(x).expected_greater(lval))
            }).max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;

            probed_subset.push(argmax);
            unprobed_regulators.unset_bit(argmax);
            values_heap.push(self.get_realization(argmax));
        }   

        (values_heap.into_iter_sorted().take(l).sum(), self.get_probemax().get_policy_time() + timer.elapsed().as_secs_f64())
    }
}

impl BipartiteRegulatorProbing {
    pub fn compute_namp_cov_policy(&mut self, k: usize, l: usize) {
        // Only compute if the policy does not already exist
        if self.has_policy(k, l) {
            return;
        }

        let timer = Instant::now();

        let (mut probed_subset, l_time): (Vec<usize>, f64) = self.get_l_policy(l);
        let mut probed_values: Vec<f64> = (0..self.get_nb())
            .map(|b| {
                probed_subset
                    .iter()
                    .map(|a| self.get_edge(*a, b).expected_value())
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0)
            })
            .collect();
        let mut unprobed_regulators = BitSet::new_all_set_but(self.get_na(), probed_subset.iter().copied());

        while probed_subset.len() < l {
            // Compute next Regulator to probe
            let argmax: usize = unprobed_regulators
                .iter()
                .map(|a| -> (usize, f64) {
                    (a, (0..self.get_nb()).map(|b| {
                        self.get_edge(a, b).expected_greater(probed_values[b].floor() as usize)
                    }).sum())
                })
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;

            // Add argmax to probed Regulators
            probed_subset.push(argmax);
            unprobed_regulators.unset_bit(argmax);
            for b in 0..self.get_nb() {
                probed_values[b] = self
                    .get_edge(argmax, b)
                    .expected_value()
                    .max(probed_values[b]);
            }
        }

        for _ in l..k {
            let argmax: usize = unprobed_regulators
                .iter()
                .map(|a| -> (usize, f64) {
                    (a, (0..self.get_nb()).map(|b| {
                        self.get_edge(a, b).expected_value()
                    }).sum())
                })
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
                
            probed_subset.push(argmax);
            unprobed_regulators.unset_bit(argmax);
        } 

        self.add_policy(k, l, probed_subset, l_time + timer.elapsed().as_secs_f64());
    }

    /// Just compute the policy for `l = k` and use `Greedy` to approximate.
    #[inline]
    pub fn compute_namp_ext_cov_policy(&mut self, k: usize) {
        self.compute_namp_cov_policy(k, k);
    }
}

impl Instance<'_> {
    pub fn adaptive_policy(&self, k: usize, l: usize) -> (usize, f64) {
        let timer = Instant::now();

        let probed_regulators = self.adaptive_policy_regulators(k, l); 

        (self.eval_policy(&probed_regulators, l), timer.elapsed().as_secs_f64())
    }

    pub fn adaptive_policy_regulators(&self, k: usize, l: usize) -> Vec<usize> {
        let mut unprobed_regulators = BitSet::new_all_set(self.get_model().get_na());
        let mut current_values: Vec<usize> = vec![0; self.get_model().get_nb()];
        let mut probed_regulators: Vec<usize> = Vec::with_capacity(k);
        
        for _ in 0..l {
            let argmax = unprobed_regulators.iter().map(|a| -> (usize, f64) {
                (a, (0..self.get_model().get_nb()).map(|b| self.get_model().get_edge(a, b).expected_greater(current_values[b])).sum())
            }).max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;

            unprobed_regulators.unset_bit(argmax);
            probed_regulators.push(argmax);
            for b in 0..self.get_model().get_nb() {
                current_values[b] = self.get_realization(argmax, b).max(current_values[b]);
            }
        }

        for _ in l..k {
            let argmax: usize = unprobed_regulators.iter().map(|a| -> (usize, f64) {
                let mut temp_subset = BitSet::new_all_set_but(self.get_model().get_na(), unprobed_regulators.iter());
                temp_subset.set_bit(a);

                let mut temp_values: Vec<f64> = vec![0.0; self.get_model().get_nb()];

                for _ in 0..l {
                    let temp_argmax: usize = temp_subset.iter().map(|temp_a| -> (usize, f64) {
                        (temp_a, (0..self.get_model().get_nb()).map(|b| {
                            if a == temp_a {
                                if self.get_model().get_edge(temp_a, b).expected_value() > temp_values[b] {
                                    self.get_model().get_edge(temp_a, b).expected_value() - temp_values[b]
                                } else {
                                    0.0
                                }
                            } else {
                                if self.get_realization(temp_a, b) as f64 > temp_values[b] {
                                    self.get_realization(temp_a, b) as f64 - temp_values[b]
                                } else {
                                    0.0
                                }
                            }
                        }).sum())
                    }).max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;
                
                    temp_subset.unset_bit(temp_argmax);
                    for b in 0..self.get_model().get_nb() {
                        if a == temp_argmax {
                            if self.get_model().get_edge(temp_argmax, b).expected_value() > temp_values[b] {
                                temp_values[b] = self.get_model().get_edge(temp_argmax, b).expected_value() - temp_values[b];
                            } 
                        } else {
                            if self.get_realization(temp_argmax, b) as f64 > temp_values[b] {
                                temp_values[b] = self.get_realization(temp_argmax, b) as f64 - temp_values[b];
                            } 
                        }
                    }
                }

                (a, temp_values.into_iter().sum())
            }).max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;
        
            unprobed_regulators.unset_bit(argmax);
            probed_regulators.push(argmax);
        }

        probed_regulators
    }
}
