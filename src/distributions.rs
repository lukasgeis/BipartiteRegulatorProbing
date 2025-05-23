//! # Weighted Sampling
//!
//! This crate provides a simple implementation of Walkers AliasTable to allow for (static) weighted sampling.
//! It also provides a weighted distribution which extends the AliasTable to allow for computation with
//! individual probabilities and expected values.

use rand::Rng;

use statrs::distribution::{Discrete, Poisson};

/// Extends `WeightedSampling` by also storing the individual probabilities and expected values
/// to allow for computation with those as well as sampling.
#[derive(Debug, Clone)]
pub struct WeightedDistribution {
    n: usize,
    cum_prob: Vec<f64>,
    cum_expe: Vec<f64>,
    samples: Vec<usize>,
}

impl WeightedDistribution {
    /// Creates a weighted distribution from a list of weights.
    /// Allows for constant time
    /// - Probabilities
    /// - Expected Values
    pub fn new<R: Rng>(rng: &mut R, weights: &[f64], num_samples: usize) -> Self {
        let n = weights.len();
        assert!(n > 0);

        let total_weight = {
            let mut sum = 0.0f64;
            for w in weights {
                sum += *w;
            }
            sum
        };

        let exact_probabilities: Vec<f64> = weights.iter().map(|w| w / total_weight).collect();

        let sample_probs: Vec<f64> = (0..num_samples)
            .map(|_| rng.random_range(0.0..=1.0))
            .collect();
        let mut samples: Vec<usize> = vec![0; num_samples];

        let mut cum_prob = Vec::with_capacity(n);
        let mut cum_expe = Vec::with_capacity(n);

        cum_prob.push(exact_probabilities[0]);
        cum_expe.push(0.0);

        for (i, p) in exact_probabilities.into_iter().enumerate() {
            if i == 0 {
                continue;
            }

            cum_prob.push(cum_prob[i - 1] + p);
            cum_expe.push(cum_expe[i - 1] + p * i as f64);

            (0..num_samples).for_each(|j| {
                if sample_probs[j] <= cum_prob[i] && sample_probs[j] >= cum_prob[i - 1] {
                    samples[j] = i;
                }
            });
        }

        Self {
            n,
            cum_prob,
            cum_expe,
            samples,
        }
    }

    pub fn new_without_rng(weights: &[f64], samples: Vec<usize>) -> Self {
        let n = weights.len();
        assert!(n > 0);

        let total_weight = {
            let mut sum = 0.0f64;
            for w in weights {
                sum += *w;
            }
            sum
        };

        let exact_probabilities: Vec<f64> = weights.iter().map(|w| w / total_weight).collect();

        let mut cum_prob = Vec::with_capacity(n);
        let mut cum_expe = Vec::with_capacity(n);

        cum_prob.push(exact_probabilities[0]);
        cum_expe.push(0.0);

        for (i, p) in exact_probabilities.into_iter().enumerate() {
            if i == 0 {
                continue;
            }

            cum_prob.push(cum_prob[i - 1] + p);
            cum_expe.push(cum_expe[i - 1] + p * i as f64);
        }

        Self {
            n,
            cum_prob,
            cum_expe,
            samples,
        }
    }

    /// Returns the vector of probabilities
    #[inline]
    pub fn get_probs(&self) -> Vec<f64> {
        (0..self.n).map(|i| self.prob_equal(i)).collect()
    }

    /// Returns the size of the distribution
    #[inline]
    pub fn size(&self) -> usize {
        self.n
    }

    #[inline]
    pub fn get_sample(&self, i: usize) -> usize {
        self.samples[i]
    }

    #[inline]
    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    /// `P[X = i]`
    #[inline]
    pub fn prob_equal(&self, i: usize) -> f64 {
        if i == 0 {
            self.cum_prob[0]
        } else {
            self.cum_prob[i] - self.cum_prob[i - 1]
        }
    }

    /// `P[X < i]`
    #[inline]
    pub fn prob_less(&self, i: usize) -> f64 {
        if i == 0 {
            0.0
        } else {
            self.cum_prob[i - 1]
        }
    }

    /// `P[X > i]`
    #[inline]
    pub fn prob_greater(&self, i: usize) -> f64 {
        if i == self.n - 1 {
            0.0
        } else {
            1.0 - self.cum_prob[i]
        }
    }

    /// `i * P[X = i]`
    #[inline]
    pub fn expected_equal(&self, i: usize) -> f64 {
        if i == 0 {
            0.0
        } else {
            self.cum_expe[i] - self.cum_expe[i - 1]
        }
    }

    /// `E[X | X < i] * P[X < i]`
    #[inline]
    pub fn expected_less(&self, i: usize) -> f64 {
        if i == 0 || i == 1 {
            0.0
        } else {
            self.cum_expe[i - 1]
        }
    }

    /// `E[X | X > i] * P[X > i]`
    #[inline]
    pub fn expected_greater(&self, i: usize) -> f64 {
        if i == self.n - 1 {
            0.0
        } else {
            self.cum_expe[self.n - 1] - self.cum_expe[i]
        }
    }

    /// `E[X]`
    #[inline]
    pub fn expected_value(&self) -> f64 {
        self.cum_expe[self.n - 1]
    }

    pub fn max_distribution(dist: &[Self]) -> Self {
        let n: usize = dist[0].size();

        if n <= 1 {
            return Self {
                n,
                cum_prob: vec![1.0f64],
                cum_expe: vec![0.0f64],
                samples: vec![0],
            };
        }

        let weights: Vec<f64> = (0..n)
            .map(|i| {
                let mut inc = 1.0;
                let mut out = 1.0;
                for d in dist {
                    if i < n - 1 {
                        inc *= d.prob_less(i + 1);
                    }
                    out *= d.prob_less(i);
                }

                inc - out
            })
            .collect();

        let samples: Vec<usize> = (0..dist[0].num_samples())
            .map(|i| {
                let mut max = 0usize;
                for d in dist {
                    if d.get_sample(i) > max {
                        max = d.get_sample(i);
                    }
                }
                max
            })
            .collect();

        Self::new_without_rng(&weights, samples)
    }

    pub fn sum_distribution(dist: &[Self]) -> Self {
        fn two_sum(a: &[f64], b: &[f64]) -> Vec<f64> {
            let n = a.len() + b.len() - 1;
            (0..n)
                .map(|i| {
                    (0..a.len().min(i + 1))
                        .map(|j| {
                            if i - j < b.len() {
                                a[j] * b[i - j]
                            } else {
                                0.0
                            }
                        })
                        .sum()
                })
                .collect()
        }

        let mut all_weights: Vec<Vec<f64>> = dist.iter().map(|d| d.get_probs()).collect();

        while all_weights.len() > 1 {
            let mut new_weights: Vec<Vec<f64>> = Vec::with_capacity(all_weights.len());
            while all_weights.len() > 1 {
                let a = all_weights.pop().unwrap();
                let b = all_weights.pop().unwrap();
                new_weights.push(two_sum(&a, &b));
            }
            if let Some(w) = all_weights.pop() {
                new_weights.push(w);
            }
            all_weights = new_weights;
        }

        let samples: Vec<usize> = (0..dist[0].num_samples())
            .map(|i| {
                let mut sum = 0usize;
                for d in dist {
                    sum += d.get_sample(i);
                }
                sum
            })
            .collect();

        Self::new_without_rng(&all_weights[0], samples)
    }
}

/// Creates a vector of random weights between 0.0 and 1.0
#[inline]
pub fn create_random_weights<R: Rng>(rng: &mut R, n: usize) -> Vec<f64> {
    (0..n).map(|_| rng.random_range(0.1..=10.0)).collect()
}

/// Creates a list of weights mirroring a Poisson-Distribution up to a fixed length `n`.
#[inline]
pub fn create_poisson_weights<R: Rng>(rng: &mut R, n: usize) -> Vec<f64> {
    let lambda = rng.random_range(0.5..=2.5);
    let poisson = Poisson::new(lambda).unwrap();

    let mut total_weight = 0.0;

    let mut weights: Vec<f64> = (0..(n - 1))
        .map(|i| {
            let p = poisson.pmf(i as u64);
            total_weight += p;
            p
        })
        .collect();

    assert!(total_weight <= 1.0);
    weights.push(1.0 - total_weight);

    weights
}
