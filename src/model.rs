use std::time::Instant;

use rand::Rng;
use ez_bitset::bitset::*;

use crate::distributions::*;

#[derive(Debug, Clone)]
pub struct BipartiteRegulatorProbing {
    // Number of Regulators
    pub na: usize,
    // Number of Positions
    pub nb: usize,
    // Size of Support
    pub vs: usize,
    // Distributions of Edges
    pub edges: Vec<Vec<WeightedDistribution>>,
    // Optional Non-Adaptive COV Policies for given k and l
    pub non_adaptive_cov_policies: Vec<(usize, usize, Vec<usize>, f64)>,
}

impl BipartiteRegulatorProbing {
    pub fn new(na: usize, nb: usize, vs: usize, edges: Vec<Vec<WeightedDistribution>>) -> Self {
        Self {
            na, nb, vs, edges, non_adaptive_cov_policies: Vec::new()
        }
    }

    pub fn create_random<R: Rng>(
        rng: &mut R,
        na: usize,
        nb: usize,
        vs: usize,
        poisson: bool,
        num_instances: usize,
    ) -> Self {
        let edges: Vec<Vec<WeightedDistribution>> = (0..na)
            .map(|_| {
                (0..nb)
                    .map(|_| {
                        if vs <= 1 {
                            return WeightedDistribution::new(rng, &[1.0], num_instances);
                        }

                        let weights = if poisson {
                            create_poisson_weights(rng, vs)
                        } else {
                            create_random_weights(rng, vs)
                        };

                        WeightedDistribution::new(rng, &weights, num_instances)
                    })
                    .collect()
            })
            .collect();

        Self {
            na,
            nb,
            vs,
            edges,
            non_adaptive_cov_policies: Vec::new(),
        }
    }

    /// Get Number of Regulators
    #[inline]
    pub fn get_na(&self) -> usize {
        self.na
    }

    /// Get Number of Positions
    #[inline]
    pub fn get_nb(&self) -> usize {
        self.nb
    }

    /// Get Size of Support
    #[inline]
    pub fn get_vs(&self) -> usize {
        self.vs
    }

    /// Get the Distributions of all incident edges of Regulator a
    #[inline]
    pub fn get_regulator(&self, a: usize) -> &Vec<WeightedDistribution> {
        &self.edges[a]
    }

    /// Get the Distribution of edge (a,b)
    #[inline]
    pub fn get_edge(&self, a: usize, b: usize) -> &WeightedDistribution {
        &self.edges[a][b]
    }

    /// Is there already a policy for this (k,l) pair
    #[inline]
    pub fn has_policy(&self, k: usize, l: usize) -> bool {
        self.non_adaptive_cov_policies
            .iter()
            .find(|(a, b, _, _)| *a == k && *b == l)
            .is_some()
    }

    /// Get the policy for a specific (k,l) pair
    #[inline]
    pub fn get_policy(&self, k: usize, l: usize) -> Option<&Vec<usize>> {
        self.non_adaptive_cov_policies
            .iter()
            .find(|(a, b, _, _)| *a == k && *b == l)
            .map(|(_, _, p, _)| p)
    }

    /// Get the policy time for a specific (k,l) pair
    #[inline]
    pub fn get_policy_time(&self, k: usize, l: usize) -> Option<f64> {
        self.non_adaptive_cov_policies
            .iter()
            .find(|(a, b, _, _)| *a == k && *b == l)
            .map(|(_, _, _, t)| *t)
    }

    /// Get the first l probes from any policy with at least l choosable boxes.
    /// If no such policy exists, take a policy with high enough l and take its first l probes.
    #[inline]
    pub fn get_l_policy(&self, l: usize) -> (Vec<usize>, f64) {
        if self.non_adaptive_cov_policies.len() == 0 {
            return (Vec::new(), 0.0);
        }

        if let Some((_, _, policy, time)) = self
            .non_adaptive_cov_policies
            .iter()
            .find(|(_, b, _, _)| *b >= l)
        {
            (policy.iter().copied().take(l).collect(), *time)
        } else {
            let (_, b, policy, time) = self
                .non_adaptive_cov_policies
                .iter()
                .max_by(|(_, b1, _, _), (_, b2, _, _)| b1.cmp(b2))
                .unwrap();
            (policy.iter().copied().take(*b).collect(), *time)
        }
    }

    /// Add a policy for a (k,l) pair
    #[inline]
    pub fn add_policy(&mut self, k: usize, l: usize, policy: Vec<usize>, time: f64) {
        assert!(policy.len() == k);
        self.non_adaptive_cov_policies.push((k, l, policy, time));
    }

    /// Create an Instance
    #[inline]
    pub fn create_instance(&self, instance_index: usize) -> Instance {
        Instance::new(self, instance_index)
    }
}

#[derive(Debug, Clone)]
pub struct Instance<'a> {
    bpr: &'a BipartiteRegulatorProbing,
    realizations: Vec<Vec<usize>>,
    greedy_cov_values: Vec<usize>,
    opt_time: f64,
}

impl<'a> Instance<'a> {
    /// Create an Instance from a BPR model and computes the vector of a GREEDY algorithm for Offline-Cov
    #[inline]
    pub fn new(bpr: &'a BipartiteRegulatorProbing, instance_index: usize) -> Self {
        let realizations: Vec<Vec<usize>> = (0..bpr.get_na())
            .map(|a| {
                (0..bpr.get_nb())
                    .map(|b| bpr.get_edge(a, b).get_sample(instance_index))
                    .collect()
            })
            .collect();

        let timer = Instant::now();

        let mut current_values: Vec<usize> = vec![0; bpr.get_nb()];
        let mut greedy_cov_values: Vec<usize> = Vec::with_capacity(bpr.get_na() + 1);
        let mut chosen_regulators = BitSet::new_all_set(bpr.get_na());

        greedy_cov_values.push(0);

        for _ in 0..bpr.get_na() {
            let (inc, argmax): (usize, usize) = chosen_regulators.iter().map(|a| -> (usize, usize) {
                ((0..bpr.get_nb()).filter_map(|b| {
                    if realizations[a][b] > current_values[b] {
                        Some(realizations[a][b] - current_values[b])
                    } else {
                        None
                    }
                }).sum(), a)
            }).max().unwrap();

            greedy_cov_values.push(*greedy_cov_values.last().unwrap() + inc);
            chosen_regulators.unset_bit(argmax);
            (0..bpr.get_nb()).for_each(|b| {
                if realizations[argmax][b] > current_values[b] {
                    current_values[b] = realizations[argmax][b];
                }
            })
        }

        Self { bpr, realizations, greedy_cov_values, opt_time: timer.elapsed().as_secs_f64() }
    }

    /// Get the BPR-model
    #[inline]
    pub fn get_model(&self) -> &BipartiteRegulatorProbing {
        &self.bpr
    }

    /// Get an Edge-Realization
    #[inline]
    pub fn get_realization(&self, a: usize, b: usize) -> usize {
        self.realizations[a][b]
    }

    #[inline]
    pub fn get_opt_cov_value(&self, l: usize) -> usize {
        self.greedy_cov_values[l]
    }

    #[inline]
    pub fn get_opt_cov_time(&self) -> f64 {
        self.opt_time
    }

    #[inline]
    pub fn eval_policy(&self, policy: &[usize], l: usize) -> usize {
        if policy.len() == l {
            return (0..self.bpr.get_nb()).map(|b| {
                policy.iter().map(|a| self.realizations[*a][b]).max().unwrap_or(0)
            }).sum();
        }

        let mut current_values: Vec<usize> = vec![0; self.bpr.get_nb()];
        let mut greedy_value = 0usize;
        let mut chosen_regulators = BitSet::new_all_set(policy.len());

        for _ in 0..l {
            let (inc, argmax): (usize, usize) = chosen_regulators.iter().map(|a| -> (usize, usize) {
                ((0..self.bpr.get_nb()).filter_map(|b| {
                    if self.realizations[a][b] > current_values[b] {
                        Some(self.realizations[a][b] - current_values[b])
                    } else {
                        None
                    }
                }).sum(), a)
            }).max().unwrap();

            greedy_value += inc;
            chosen_regulators.unset_bit(argmax);
            (0..self.bpr.get_nb()).for_each(|b| {
                if self.realizations[argmax][b] > current_values[b] {
                    current_values[b] = self.realizations[argmax][b];
                }
            })
        }

        greedy_value
    }
}

#[derive(Debug, Clone)]
pub struct ProbeMax {
    /// Number of Boxes
    n: usize,
    /// Size of Support
    v: usize,
    /// Boxes
    boxes: Vec<WeightedDistribution>,
    /// Non-Adaptive Policy
    non_adaptive_policy: (Vec<usize>, f64),
}

impl ProbeMax {
    /// Create a ProbeMax Instance using a Max-Reduction
    pub fn from_bpr_max(bpr: &BipartiteRegulatorProbing) -> Self {
        let n = bpr.get_na();
        let v = bpr.get_vs();
        let boxes: Vec<WeightedDistribution> = (0..n)
            .map(|i| WeightedDistribution::max_distribution(&bpr.get_regulator(i)))
            .collect();

        let non_adaptive_policy = ProbeMax::compute_namp_policy(&boxes);

        Self {
            n,
            v,
            boxes,
            non_adaptive_policy,
        }
    }

    /// Create a ProbeMax Instance using a Sum-Reduction
    pub fn from_bpr_sum(bpr: &BipartiteRegulatorProbing) -> Self {
        let n = bpr.get_na();
        let v = (bpr.get_vs() - 1) * bpr.get_nb() + 1;
        let boxes: Vec<WeightedDistribution> = (0..n)
            .map(|i| WeightedDistribution::sum_distribution(&bpr.get_regulator(i)))
            .collect();

        let non_adaptive_policy = ProbeMax::compute_namp_policy(&boxes);

        Self {
            n,
            v,
            boxes,
            non_adaptive_policy,
        }
    }

    /// Get the number of Boxes
    #[inline]
    pub fn get_n(&self) -> usize {
        self.n
    }

    /// Get the Size of Support
    #[inline]
    pub fn get_v(&self) -> usize {
        self.v
    }

    /// Get all boxes
    #[inline]
    pub fn get_boxes(&self) -> &Vec<WeightedDistribution> {
        &self.boxes
    }

    /// Get a single box
    #[inline]
    pub fn get_box(&self, i: usize) -> &WeightedDistribution {
        &self.boxes[i]
    }

    /// Get the Non-Adaptive Policy
    #[inline]
    pub fn get_policy(&self) -> &Vec<usize> {
        &self.non_adaptive_policy.0
    }

    /// Get the Non-Adaptive Policy Time
    #[inline]
    pub fn get_policy_time(&self) -> f64 {
        self.non_adaptive_policy.1
    }

    /// Create an Instance
    #[inline]
    pub fn create_instance(&self, instance_index: usize) -> ProbeMaxInstance {
        ProbeMaxInstance::new(self, instance_index)
    }
}

#[derive(Debug, Clone)]
pub struct ProbeMaxInstance<'a> {
    /// Reference to ProbeMax
    pm: &'a ProbeMax,
    /// Realizations of Boxes
    realizations: Vec<usize>,
    /// Cumulative optimal value of Boxes
    cum_opt_realizations: Vec<usize>,
    /// Time taken for computation of optimal values
    opt_time: f64,
}

impl<'a> ProbeMaxInstance<'a> {
    /// Create an Instance from a BPR model
    #[inline]
    pub fn new(pm: &'a ProbeMax, instance_index: usize) -> Self {
        let realizations: Vec<usize> = (0..pm.get_n())
            .map(|i| pm.get_box(i).get_sample(instance_index))
            .collect();

        let timer = Instant::now();
        
        // Sorted Realizations based on value
        let mut sorted_realizations = realizations.clone();
        sorted_realizations.sort_by(|a, b| b.cmp(a));

        // Cumulative optimal values
        let mut cum_opt_realizations: Vec<usize> = Vec::with_capacity(sorted_realizations.len());
        cum_opt_realizations.push(sorted_realizations[0]);
        for i in 1..sorted_realizations.len() {
            cum_opt_realizations.push(cum_opt_realizations[i - 1] + sorted_realizations[i]);
        }

        Self {
            pm,
            realizations,
            cum_opt_realizations,
            opt_time: timer.elapsed().as_secs_f64(),
        }
    }

    /// Get ProbeMax
    #[inline]
    pub fn get_probemax(&self) -> &ProbeMax {
        &self.pm
    }

    /// Get the realization of a single box
    #[inline]
    pub fn get_realization(&self, i: usize) -> usize {
        self.realizations[i]
    }

    /// Get the optimal offline value for l boxes
    #[inline]
    pub fn get_optimal_value(&self, l: usize) -> usize {
        self.cum_opt_realizations[l - 1]
    }

    /// Get the time needed for computation of optimal subsets
    #[inline]
    pub fn get_optimal_time(&self) -> f64 {
        self.opt_time
    }

    /// Get the value of the non-adaptive policy
    #[inline]
    pub fn get_non_adap_value(&self, k: usize, l: usize) -> (usize, f64) {
        let timer = Instant::now();

        let mut non_adap_realizations: Vec<usize> = self
            .pm
            .get_policy()
            .iter()
            .take(k)
            .map(|i| self.realizations[*i])
            .collect();
        non_adap_realizations.sort_by(|a, b| b.cmp(a));

        (non_adap_realizations.into_iter().take(l).sum(), self.pm.get_policy_time() + timer.elapsed().as_secs_f64())
    }
}
