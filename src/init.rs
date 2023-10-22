use rand_distr::{Normal, Distribution};

use crate::{opt_traits::*, FloatVec, NaiveBitVec};

pub struct InitValue<T : OptData> {
    pub value: T
}

impl<T: OptData> InitFunc<T> for InitValue<T> {
    fn init(&self) -> T {
        self.value.clone()
    }
}

pub struct InitPopulationFromValues<T : OptData> {
    pub population: Vec<T>
}

impl<T : OptData> InitPopulation<T> for InitPopulationFromValues<T> {
    fn init(&self) -> Vec<T> {
        self.population.to_vec()
    }
}

pub struct InitRandomFloatVecPopulation {
    pub size: usize,
    pub vec_size: usize,
    pub mean: f64,
    pub std_dev: f64
}

impl InitPopulation<FloatVec> for InitRandomFloatVecPopulation {
    fn init(&self) -> Vec<FloatVec> {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        let mut population = Vec::<FloatVec>::with_capacity(self.size);
        for _ in 0..self.size {
            let mut data = Vec::<f64>::with_capacity(self.vec_size);
            for _ in 0..self.vec_size {
                data.push(normal.sample(&mut rand::thread_rng()));
            }
            population.push(FloatVec { values: data });
        }
        population
    }
}

pub struct InitRandomNaiveBitVecPopulation {
    pub size: usize,
    pub bits_count: usize
}

impl InitPopulation<NaiveBitVec> for InitRandomNaiveBitVecPopulation {
    fn init(&self) -> Vec<NaiveBitVec> {
        let mut population = Vec::<NaiveBitVec>::with_capacity(self.size);
        for _ in 0..self.size {
            let mut bits = Vec::<u8>::with_capacity(self.bits_count);
            for _ in 0..self.bits_count {
                bits.push(if rand::random::<bool>() { 1u8 } else { 0u8 });
            }
            population.push(NaiveBitVec { bits: bits });
        }
        population
    }
}