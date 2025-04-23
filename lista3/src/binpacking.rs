use rand::prelude::*;

pub enum PackType {
    NextFit,
    RandomFit,
    FirstFit,
    BestFit,
    WorstFit,
}

impl std::fmt::Display for PackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackType::NextFit => write!(f, "Next"),
            PackType::RandomFit => write!(f, "Random"),
            PackType::FirstFit => write!(f, "First"),
            PackType::BestFit => write!(f, "Best"),
            PackType::WorstFit => write!(f, "Worst"),
        }
    }

}

pub struct BinPacking {
    bins: Vec<f64>,
    pack_type: PackType,
}

impl BinPacking {
    pub fn new(pack_type: PackType) -> Self {
        Self { bins: vec![ 0.0 ], pack_type }
    }

    pub fn optimal_packing(weights: &[f64]) -> usize {
        weights.iter().sum::<f64>().ceil() as usize
    }

    pub fn find_idx(&self, weight: f64) -> Option<usize> {
        match self.pack_type {
            PackType::NextFit => {
                if self.bins.last()? + weight <= 1.0 {
                    Some(self.bins.len() - 1)
                } else {
                    None
                }
            },
            PackType::RandomFit => {
                Some( *self.bins.iter().enumerate().filter_map(|(idx, &x)| { if x + weight <= 1.0 { Some(idx) } else { None } }).collect::<Vec<_>>().choose(&mut rand::rng())? )
            },
            PackType::FirstFit => {
                self.bins.iter().position(|&x| x + weight <= 1.0)
            },
            PackType::BestFit => {
                Some ( self.bins.iter().enumerate().filter(|(_, &x)| x + weight <= 1.0).max_by(|(_, x), (_, y)| x.total_cmp(y))?.0 )
            },
            PackType::WorstFit => {
                Some ( self.bins.iter().enumerate().filter(|(_, &x)| x + weight <= 1.0).min_by(|(_, x), (_, y)| x.total_cmp(y))?.0 )
            },
        }
    }

    pub fn pack(&mut self, weight: f64) {
        match self.find_idx(weight) {
            Some(idx) => self.bins[idx] += weight,
            None => self.bins.push(weight),
        }
    }

    pub fn pack_multiple(&mut self, weights: &[f64]) {
        for &weight in weights {
            self.pack(weight);
        }
    }

    pub fn number_of_bins(&self) -> usize {
        self.bins.len()
    }
}
