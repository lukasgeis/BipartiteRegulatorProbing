use std::f64::consts::E;

use rand::Rng;

use crate::{factorial, is_close, Probability, Reward};

/// Structure representing a DiscreteDistribution
#[derive(Debug, Clone, PartialEq)]
pub struct DiscreteDistribution {
    vs: usize,
    exact: Vec<Probability>,
    cumulative: Vec<Probability>,
    expected: Vec<Reward>,
}

impl DiscreteDistribution {
    /// Creates a new DiscreteDistribution
    pub fn new(vs: usize, poisson: bool) -> Self {
        if vs <= 1 {
            return Self {
                vs: 1,
                exact: vec![1.0],
                cumulative: vec![1.0],
                expected: vec![1.0],
            };
        }

        let mut rng = rand::thread_rng();

        if !poisson {
            let mut probabilities: Vec<Probability> = Vec::with_capacity(vs);
            let mut total_value: Probability = 0.0;

            for _ in 0..vs {
                let p: Probability = rng.gen();
                total_value += p;
                probabilities.push(p);
            }

            let ep: Vec<Probability> = probabilities.into_iter().map(|p| p / total_value).collect();
            let mut cp: Vec<Probability> = Vec::with_capacity(vs);
            let mut ev: Vec<Probability> = Vec::with_capacity(vs);

            cp.push(ep[0]);
            ev.push(0.0);

            for k in 1..vs {
                cp.push(cp[k - 1] + ep[k]);
                ev.push(ev[k - 1] + k as f64 * ep[k]);
            }

            return Self {
                vs: vs,
                exact: ep,
                cumulative: cp,
                expected: ev,
            };
        } else {
            let lambda: f64 = rng.gen_range(0.5..2.5);
            let e_term: f64 = E.powf(-lambda);
            let mut ep: Vec<Probability> = Vec::with_capacity(vs);
            for k in 0..(vs - 1) {
                ep.push(lambda.powi(k as i32) * e_term / factorial(k) as f64);
            }
            ep.push(1.0 - ep.iter().sum::<f64>());

            let mut cp: Vec<Probability> = Vec::with_capacity(vs);
            let mut ev: Vec<Probability> = Vec::with_capacity(vs);

            cp.push(ep[0]);
            ev.push(0.0);

            for k in 1..vs {
                cp.push(cp[k - 1] + ep[k]);
                ev.push(ev[k - 1] + k as f64 * ep[k]);
            }

            return Self {
                vs: vs,
                exact: ep,
                cumulative: cp,
                expected: ev,
            };
        }
    }

    /// Creates a new Distribution from a given list of proabilities
    pub fn from_list(list: &Vec<Probability>) -> Self {
        if !is_close(list.into_iter().sum::<Probability>(), 1.0)
            || !list.into_iter().all(|p| 0.0 <= *p && *p <= 1.0)
        {
            return Self {
                vs: 1,
                exact: vec![1.0],
                cumulative: vec![1.0],
                expected: vec![0.0],
            };
        }

        let v = list.len();

        let mut ep: Vec<Probability> = Vec::with_capacity(v);
        let mut cp: Vec<Probability> = Vec::with_capacity(v);
        let mut ev: Vec<f64> = Vec::with_capacity(v);

        ep.push(list[0]);
        cp.push(list[0]);
        ev.push(0.0);

        for k in 1..v {
            ep.push(list[k]);
            cp.push(cp[k - 1] + list[k]);
            ev.push(ev[k - 1] + k as f64 * list[k]);
        }

        Self {
            vs: v,
            exact: ep,
            cumulative: cp,
            expected: ev,
        }
    }

    /// Get Size of Support
    pub fn size(&self) -> usize {
        self.vs
    }

    /// Get Probability that k is realized
    pub fn equal(&self, k: usize) -> Probability {
        if k >= self.vs {
            0.0 as Probability
        } else {
            self.exact[k]
        }
    }

    /// Get Probability that a value less or equal to k is realized
    pub fn less(&self, k: usize) -> Probability {
        if k >= self.vs {
            1.0 as Probability
        } else {
            self.cumulative[k]
        }
    }

    /// Get Probability that a value greater or equal to k is realized
    pub fn greater(&self, k: usize) -> Probability {
        if k >= self.vs {
            0.0 as Probability
        } else if k == 0 {
            1.0 as Probability
        } else {
            1.0 - self.cumulative[k - 1]
        }
    }

    /// Get expected value of DiscreteDistribution
    pub fn expected_value(&self) -> f64 {
        self.expected[self.vs - 1]
    }

    /// Get expected value of values less or equal to k
    pub fn expected_less(&self, k: usize) -> f64 {
        if k >= self.vs {
            return 0.0;
        }
        self.expected[k]
    }

    /// Get expected value of values greater or equal to k
    pub fn expected_greater(&self, k: usize) -> f64 {
        if k == 0 {
            self.expected[self.vs - 1]
        } else if k < self.vs {
            self.expected[self.vs - 1] - self.expected[k - 1]
        } else {
            return 0.0;
        }
    }

    /// Draw value from DiscreteDistribution
    pub fn draw_value(&self) -> usize {
        let random_value: Probability = rand::thread_rng().gen();
        for k in 0..(self.size() - 1) {
            if random_value <= self.less(k) {
                return k;
            }
        }
        self.size() - 1
    }
}

/// Computes the DiscreteDistribution max{X_1,...,X_n} for DiscreteDistributions X_1,...,X_n
pub fn max_distributions(distributions: &Vec<DiscreteDistribution>) -> DiscreteDistribution {
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

    DiscreteDistribution::from_list(&values)
}

/// Computes the DiscreteDistribution sum{X_1,...,X_n} for DiscreteDistributions X_1,...,X_n
pub fn sum_distributions(distributions: &Vec<DiscreteDistribution>) -> DiscreteDistribution {
    /// Computes the DiscreteDistribution of sum(a, b) for DiscreteDistributions a, b
    fn two_sum_distributions(
        a: &DiscreteDistribution,
        b: &DiscreteDistribution,
    ) -> DiscreteDistribution {
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

        DiscreteDistribution::from_list(&values)
    }

    let mut computed_distributions: Vec<DiscreteDistribution> = distributions.clone();

    while computed_distributions.len() > 1 {
        let mut old_distributions: Vec<DiscreteDistribution> = computed_distributions.clone();
        computed_distributions.truncate(0);

        if old_distributions.len() % 2 == 1 {
            computed_distributions.push(old_distributions.pop().unwrap());
        }

        for k in 0..(old_distributions.len() / 2usize) {
            computed_distributions.push(two_sum_distributions(
                &old_distributions[2 * k],
                &old_distributions[2 * k + 1],
            ));
        }
    }

    computed_distributions[0].clone()
}
