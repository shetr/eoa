use rand_distr::{Cauchy, Normal, Distribution};

use crate::opt_traits::*;
use crate::opt_data::*;

pub fn perturbe_mut(bits: &mut [u8], prob: f64) {
    for bit in bits.iter_mut() {
        *bit = if rand::random::<f64>() > prob { *bit } else { *bit ^ 1 };
    }
}

pub fn perturbe(bits: &[u8], prob: f64) -> Vec<u8> {
    let mut res = Vec::from(bits);
    perturbe_mut(&mut res[..], prob);
    res
}

pub struct BasicNaiveBitPerturbeMutOp {}

impl PerturbeMutOp<NaiveBitVec> for BasicNaiveBitPerturbeMutOp {
    fn eval(&self, data: &mut NaiveBitVec) {
        let bit_count = data.bits.len() as f64;
        perturbe_mut(&mut data.bits, 1.0 / bit_count)
    }
}

pub struct NormalPerturbeRealMutOp {
    normal: Normal<f64>
}

impl NormalPerturbeRealMutOp {
    pub fn new(sigma: f64) -> Self {
        NormalPerturbeRealMutOp { normal: Normal::new(0.0, sigma).unwrap() }
    }
}

impl PerturbeMutOp<FloatVec> for NormalPerturbeRealMutOp {
    fn eval(&self, data: &mut FloatVec) {
        for value in &mut data.values {
            *value = *value + self.normal.sample(&mut rand::thread_rng());
        }
    }
}

pub struct NormalOneFiftPerturbeRealMutOp {
    normal: Normal<f64>
}

impl NormalOneFiftPerturbeRealMutOp {
    pub fn new(sigma: f64) -> Self {
        NormalOneFiftPerturbeRealMutOp { normal: Normal::new(0.0, sigma).unwrap() }
    }
}

impl PerturbeMutOp<FloatVec> for NormalOneFiftPerturbeRealMutOp {
    fn eval(&self, data: &mut FloatVec) {
        for value in &mut data.values {
            *value = *value + self.normal.sample(&mut rand::thread_rng());
        }
    }

    fn update(&mut self, is_better: bool, dim: usize) {
        let sigma = self.normal.std_dev() * (if is_better { 1.0 } else { 0.0 } - 0.2f64).exp().powf(1.0 / (dim as f64));
        self.normal = Normal::new(0.0, sigma).unwrap();
    }
}

pub struct CauchyPerturbeRealMutOp {
    cauchy: Cauchy<f64>
}

impl CauchyPerturbeRealMutOp {
    pub fn new(scale: f64) -> Self {
        CauchyPerturbeRealMutOp { cauchy: Cauchy::new(0.0, scale).unwrap() }
    }
}

impl PerturbeMutOp<FloatVec> for CauchyPerturbeRealMutOp {
    fn eval(&self, data: &mut FloatVec) {
        for value in &mut data.values {
            *value = *value + self.cauchy.sample(&mut rand::thread_rng());
        }
    }
}