/// Checks if two numbers of f64 are close enough to let their difference be considered a numerical error
pub fn is_close(a: f64, b: f64, tol: Option<f64>) -> bool {
    (b - a).abs() < tol.unwrap_or(1e-09)
}

pub type Probability = f64;

/// Discrete Distribution over [0,1,...,v - 1]
#[derive(Debug, Clone, PartialEq)]
pub struct Distribution {
    /// Size of Support
    v: usize,
    /// Exact Probabilities
    ep: Vec<Probability>,
    /// Cumulative Ascending Probabilities
    ap: Vec<Probability>,
    /// Cumulative Descending Probabilities
    dp: Vec<Probability>,
}

impl Distribution {
    /// Creates a new Distribution from a given list of proabilities
    fn from_list(list: &Vec<Probability>) -> Self {
        if !is_close(list.into_iter().sum::<Probability>(), 1.0, None)
            || !list.into_iter().all(|p| 0.0 <= *p && *p <= 1.0)
        {
            return Self {
                v: 1,
                ep: vec![1.0],
                ap: vec![1.0],
                dp: vec![1.0],
            };
        }

        let v = list.len();

        let mut ep: Vec<Probability> = Vec::with_capacity(v);
        let mut ap: Vec<Probability> = Vec::with_capacity(v);
        let mut dp: Vec<Probability> = Vec::with_capacity(v);

        ep.push(list[0]);
        ap.push(list[0]);
        dp.push(list[v - 1]);

        for k in 1..v {
            ep.push(list[k]);
            ap.push(ap[k - 1] + list[k]);
            dp.push(dp[k - 1] + list[v - k - 1]);
        }

        Self {
            v: v,
            ep: ep,
            ap: ap,
            dp: dp.into_iter().rev().collect(),
        }
    }

    /// Get Size of Support
    fn size(&self) -> usize {
        self.v
    }

    /// Get Probability that k is realized
    fn equal(&self, k: usize) -> Probability {
        if k >= self.v {
            0.0 as Probability
        } else {
            self.ep[k]
        }
    }

    /// Get Probability that a value less or equal to k is realized
    fn less(&self, k: usize) -> Probability {
        if k >= self.v {
            1.0 as Probability
        } else {
            self.ap[k]
        }
    }

    /// Get Probability that a value greater or equal to k is realized
    fn greater(&self, k: usize) -> Probability {
        if k >= self.v {
            0.0 as Probability
        } else {
            self.dp[k]
        }
    }
}

/// Computes the Distribution max{X_1,...,X_n} for Distributions X_1,...,X_n
pub fn max_distribution(distributions: &Vec<Distribution>) -> Distribution {
    let v: usize = distributions.into_iter().map(|d| d.size()).max().unwrap();

    let mut values: Vec<Probability> = Vec::with_capacity(v);

    for i in 0..v {
        let mut inc: Probability = 1.0;
        let mut out: Probability = 1.0;
        for j in 0..distributions.len() {
            inc *= distributions[j].less(i);
            if i > 0 {
                out *= distributions[j].less(i - 1);
            } else {
                out *= 0.0;
            }
        }
        values.push(inc - out);
    }

    Distribution::from_list(&values)
}

/// Computes the Distribution sum{X_1,...,X_n} for Distributions X_1,...,X_n
pub fn sum_distribution(distributions: &Vec<Distribution>) -> Distribution {
    /// Computes the Distribution of sum(a, b) for Distributions a, b
    fn two_sum_distribution(a: &Distribution, b: &Distribution) -> Distribution {
        let v: usize = a.size() + b.size() - 1;
        let mut values: Vec<Probability> = Vec::with_capacity(v);
        for i in 0..v {
            let mut i_prob = 0.0;
            for j in 0..a.size() {
                if i >= j {
                    i_prob += a.equal(j) * b.equal(i - j);
                }
            }
            values.push(i_prob);
        }

        Distribution::from_list(&values)
    }

    let mut computed_distributions: Vec<Distribution> = distributions.clone();

    while computed_distributions.len() > 1 {
        let mut old_distributions: Vec<Distribution> = computed_distributions.clone();
        computed_distributions.truncate(0);

        if old_distributions.len() % 2 == 1 {
            computed_distributions.push(old_distributions.pop().unwrap());
        }

        for k in 0..(old_distributions.len() / 2usize) {
            computed_distributions.push(two_sum_distribution(
                &old_distributions[2 * k],
                &old_distributions[2 * k + 1],
            ));
        }
    }

    computed_distributions[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;

    #[test]
    fn not_a_distribution() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut values: Vec<Probability> = Vec::new();
            let mut total: Probability = 0.0;

            while total <= 1.1 {
                values.push(rng.gen());
                total += values[values.len() - 1];
            }

            assert_eq!(
                Distribution::from_list(&values),
                Distribution {
                    v: 1,
                    ep: vec![1.0],
                    ap: vec![1.0],
                    dp: vec![1.0],
                }
            );
        }
    }

    #[test]
    fn distribution_accesses() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut values: Vec<Probability> = Vec::new();

            let size: usize = rng.gen_range(10..20) as usize;

            for _ in 0..size {
                values.push(1.0 + rng.gen::<Probability>());
            }

            let total: Probability = (&values).into_iter().sum();

            let exact_values: Vec<Probability> = values.into_iter().map(|v| v / total).collect();
            let distribution: Distribution = Distribution::from_list(&exact_values);

            for _ in 0..10 {
                let k: usize = rng.gen_range(0..size);
                assert!(is_close(distribution.equal(k), (&exact_values)[k], None));
                assert!(is_close(
                    distribution.less(k),
                    (&exact_values).into_iter().take(k + 1).sum(),
                    None
                ));
                assert!(is_close(
                    distribution.greater(k),
                    (&exact_values)
                        .into_iter()
                        .rev()
                        .take(size - k)
                        .sum::<Probability>(),
                    None
                ));
            }
        }
    }

    #[test]
    fn example_max_distribution() {
        let distributions: Vec<Distribution> = vec![
            Distribution::from_list(&vec![0.250, 0.500, 0.25]),
            Distribution::from_list(&vec![0.125, 0.625, 0.25]),
            Distribution::from_list(&vec![
                0.125, 0.125, 0.125, 0.125, 0.125, 0.125, 0.125, 0.125,
            ]),
        ];

        let correct_distribution: Vec<Probability> = vec![
            0.00390625, 0.13671875, 0.234375, 0.125, 0.125, 0.125, 0.125, 0.125,
        ];

        let proposed_distribution: Vec<Probability> = max_distribution(&distributions).ep;

        for k in 0..8 {
            assert!(is_close(
                proposed_distribution[k],
                correct_distribution[k],
                None
            ));
        }
    }

    #[test]
    fn example_sum_distribution() {
        let distributions: Vec<Distribution> = vec![
            Distribution::from_list(&vec![0.250, 0.500, 0.25]),
            Distribution::from_list(&vec![0.125, 0.625, 0.25]),
            Distribution::from_list(&vec![0.125, 0.125, 0.750]),
        ];

        let correct_distribution: Vec<Probability> = vec![
            0.00390625, 0.03125, 0.1015625, 0.25, 0.34765625, 0.21875, 0.046875,
        ];

        let proposed_distribution: Vec<Probability> = sum_distribution(&distributions).ep;
        for k in 0..7 {
            assert!(is_close(
                proposed_distribution[k],
                correct_distribution[k],
                None
            ));
        }
    }
}
