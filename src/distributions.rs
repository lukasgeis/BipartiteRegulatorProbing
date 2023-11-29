//! # Weighted Sampling
//!
//! This crate provides a simple implementation of Walkers AliasTable to allow for (static) weighted sampling.
//! It also provides a weighted distribution which extends the AliasTable to allow for computation with
//! individual probabilities and expected values.

use rand::{prelude::Distribution, Rng};

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub},
};

use num::{One, Zero};
use rand::distributions::uniform::SampleUniform;
use statrs::distribution::{Discrete, Poisson};

use crate::is_close;

pub trait Weight:
    Copy
    + Debug
    + Sized
    + PartialEq
    + PartialOrd
    + Default
    + Zero
    + One
    + SampleUniform
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
{
    fn as_f64(self) -> f64;
    fn from_f64(from: f64) -> Self;
}

macro_rules! impl_weight {
    ($t:ty) => {
        impl Weight for $t {
            #[inline]
            fn as_f64(self) -> f64 {
                self as f64
            }

            #[inline]
            fn from_f64(from: f64) -> Self {
                from as $t
            }
        }
    };
}

impl_weight!(u8);
impl_weight!(u16);
impl_weight!(u32);
impl_weight!(u64);
impl_weight!(u128);
impl_weight!(usize);

impl_weight!(f32);
impl_weight!(f64);

/// A data structure allowing for constant time sampling of weighted indices `0..n`.
/// Represented using a static Alias-Table.
#[derive(Debug, Clone)]
pub struct WeightedSampling {
    alias: Vec<usize>,
    probs: Vec<f64>,
}

impl WeightedSampling {
    /// Creates a new AliasTable from a list of weights.
    /// Note that if all weights are `0`, then it returns a uniform distribution over all indices.
    pub fn new<W: Weight>(weights: &[W]) -> Self {
        let (alias, probs) = Self::compute_table(weights);
        Self { alias, probs }
    }

    /// Creates a weighted distribution from a list of weights.
    pub fn with_distribution<W: Weight>(weights: &[W]) -> WeightedDistribution {
        WeightedDistribution::new(weights)
    }

    /// Initializes an ALiasTable using a list of aliases and probabilities.
    /// Correctness must be enforce by the user.
    pub fn init(alias: Vec<usize>, probs: Vec<f64>) -> Self {
        Self { alias, probs }
    }

    /// Computes an AliasTable based off a list of weights.
    pub fn compute_table<W: Weight>(weights: &[W]) -> (Vec<usize>, Vec<f64>) {
        let n = weights.len();
        let mean = {
            let mut sum = W::zero();
            for w in weights {
                sum += *w;
            }
            sum.as_f64() / n as f64
        };

        let (mut below, mut above) = {
            let mut above = Vec::with_capacity(n);
            let mut below = Vec::with_capacity(n);
            for (i, w) in weights.iter().enumerate() {
                let w_f64 = (*w).as_f64();
                if w_f64 < mean {
                    below.push((i, w_f64));
                } else {
                    above.push((i, w_f64));
                }
            }
            (below, above)
        };

        let mut alias = vec![0; n];
        let mut probs = vec![0.0; n];

        while let Some((i, wi)) = below.pop() {
            let (j, wj) = above.pop().unwrap();

            alias[i] = j;
            probs[i] = wi / mean;

            let wd = {
                let wd = wj + wi - mean;
                if is_close(wd, mean) {
                    mean
                } else {
                    wd
                }
            };
            if wd <= 0.0 {
                continue;
            }

            if wd < mean {
                below.push((j, wd));
            } else {
                above.push((j, wd));
            }
        }

        while let Some((i, _)) = above.pop() {
            alias[i] = i;
            probs[i] = 1.0;
        }

        (alias, probs)
    }

    /// Samples an index from the AliasTable.
    /// Note that `.sample(rng)` is preserved for the [Distribution Trait](rand::distributions::Distribution).
    pub fn sample_index<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let row = rng.gen_range(0..self.alias.len());
        let val = rng.gen_range(0.0..1.0);

        if val < self.probs[row] {
            row
        } else {
            self.alias[row]
        }
    }
}

impl Distribution<usize> for WeightedSampling {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        self.sample_index(rng)
    }
}

/// Extends `WeightedSampling` by also storing the individual probabilities and expected values
/// to allow for computation with those as well as sampling.
#[derive(Debug, Clone)]
pub struct WeightedDistribution {
    n: usize,
    cum_prob: Vec<f64>,
    cum_expe: Vec<f64>,
    sampling: WeightedSampling,
}

impl WeightedDistribution {
    /// Creates a weighted distribution from a list of weights.
    /// Allows for constant time
    /// - Sampling
    /// - Probabilities
    /// - Expected Values
    pub fn new<W: Weight>(weights: &[W]) -> Self {
        let n = weights.len();
        assert!(n > 0);

        let sampling = WeightedSampling::new(weights);

        let total_weight = {
            let mut sum = W::zero();
            for w in weights {
                sum += *w;
            }
            sum.as_f64()
        };

        let exact_probabilities: Vec<f64> = weights
            .into_iter()
            .map(|w| w.as_f64() / total_weight)
            .collect();

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
            sampling,
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

    /// Samples an index from the AliasTable.
    /// Note that `.sample(rng)` is preserved for the [Distribution Trait](rand::distributions::Distribution).
    #[inline]
    pub fn sample_index<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        self.sampling.sample_index(rng)
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
            return Self::new(&[1.0]);
        }

        let weights: Vec<f64> = (0..n)
            .map(|i| {
                let mut inc = 1.0;
                let mut out = 1.0;
                for j in 0..dist.len() {
                    if i < n - 1 {
                        inc *= dist[j].prob_less(i + 1);
                    }
                    out *= dist[j].prob_less(i);
                }

                inc - out
            })
            .collect();

        Self::new(&weights)
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

        let mut all_weights: Vec<Vec<f64>> = dist.into_iter().map(|d| d.get_probs()).collect();

        while all_weights.len() > 1 {
            let mut new_weights: Vec<Vec<f64>> = Vec::with_capacity(all_weights.len() + 1 / 2);
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

        Self::new(&all_weights[0])
    }
}

impl Distribution<usize> for WeightedDistribution {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        self.sampling.sample_index(rng)
    }
}

/// Creates a vector of random weights between 0.0 and 1.0
#[inline]
pub fn create_random_weights<R: Rng>(rng: &mut R, n: usize) -> Vec<f64> {
    (0..n).map(|_| rng.gen_range(0.1..=10.0)).collect()
}

/// Creates a list of weights mirroring a Poisson-Distribution up to a fixed length `n`.
#[inline]
pub fn create_poisson_weights<R: Rng>(rng: &mut R, n: usize) -> Vec<f64> {
    let lambda = rng.gen_range(0.5..=2.5);
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
