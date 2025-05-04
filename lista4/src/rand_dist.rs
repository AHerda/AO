use rand::{distr::weighted::WeightedIndex, prelude::*};

pub struct RandDist {
    dist_type: RandDistType,
    dist: WeightedIndex<f64>,
    rng: ThreadRng,
}

#[derive(Clone, Copy, Debug)]
pub enum RandDistType {
    Uniform,
    Harmonic,
    DoubleHarmonic,
    Geometric,
}

impl RandDist {
    pub fn new_uniform(max: usize) -> Self {
        Self::new(RandDistType::Uniform, max)
    }
    pub fn new_harmonic(max: usize) -> Self {
        Self::new(RandDistType::Harmonic, max)
    }
    pub fn new_double_harmonic(max: usize) -> Self {
        Self::new(RandDistType::DoubleHarmonic, max)
    }
    pub fn new_geometric(max: usize) -> Self {
        Self::new(RandDistType::Geometric, max)
    }

    pub fn new(dist_type: RandDistType, max: usize) -> Self {
        let dist = match dist_type {
            RandDistType::Uniform => {
                WeightedIndex::new((1..=max).map(|_| 1.0 / (max as f64)).collect::<Vec<f64>>())
            }
            RandDistType::Harmonic => {
                let harmonic = (1..=max).fold(0.0, |acc, i| acc + (1.0 / i as f64));
                WeightedIndex::new(
                    (1..=max)
                        .map(|i| 1.0 / (harmonic * i as f64))
                        .collect::<Vec<f64>>(),
                )
            }
            RandDistType::DoubleHarmonic => {
                let dharmonic = (1..=max).fold(0.0, |acc, i| acc + (1.0 / i.pow(2) as f64));
                WeightedIndex::new(
                    (1..=max)
                        .map(|i| 1.0 / (dharmonic * i.pow(2) as f64))
                        .collect::<Vec<f64>>(),
                )
            }
            RandDistType::Geometric => {
                let mut weights = vec![0.0; max];
                weights[0] = 0.5;
                for i in 1..max {
                    weights[i] = weights[i - 1] / 2.0;
                }
                weights[max - 1] = weights[max - 2];
                WeightedIndex::new(weights)
            }
        }
        .unwrap();

        Self {
            dist_type,
            dist,
            rng: rand::rng(),
        }
    }

    pub fn sample(&mut self) -> usize {
        self.dist.sample(&mut self.rng) + 1
    }

    pub fn sample_n(&mut self, n: usize) -> Vec<usize> {
        (0..n).map(|_| self.sample()).collect()
    }
}

impl std::fmt::Display for RandDist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dist_type)
    }
}

impl std::fmt::Display for RandDistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RandDistType::Uniform => write!(f, "Uniform"),
            RandDistType::Harmonic => write!(f, "Harmonic"),
            RandDistType::DoubleHarmonic => write!(f, "Double_Harmonic"),
            RandDistType::Geometric => write!(f, "Geometric"),
        }
    }
}
