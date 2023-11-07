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

#[derive(Clone)]
pub struct InitPopulationFromValues<T : OptData> {
    pub population: Vec<T>
}

impl<T : OptData> InitFunc<T> for InitPopulationFromValues<T> {
    fn init(&self) -> T {
        self.population[0].clone()
    }
}

impl<T : OptData> InitPopulation<T> for InitPopulationFromValues<T> {
    fn init(&self) -> Vec<T> {
        self.population.to_vec()
    }
}

#[derive(Clone)]
pub struct InitRandomFloatVecPopulation {
    pub size: usize,
    pub vec_size: usize,
    pub mean: f64,
    pub std_dev: f64
}

impl InitRandomFloatVecPopulation {
    fn rand_data(&self) -> Vec<f64>  {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        let mut data = Vec::<f64>::with_capacity(self.vec_size);
        for _ in 0..self.vec_size {
            data.push(normal.sample(&mut rand::thread_rng()));
        }
        data
    }
}

impl InitFunc<FloatVec> for InitRandomFloatVecPopulation {
    fn init(&self) -> FloatVec {
        FloatVec { values: self.rand_data() }
    }
}

impl InitPopulation<FloatVec> for InitRandomFloatVecPopulation {
    fn init(&self) -> Vec<FloatVec> {
        let mut population = Vec::<FloatVec>::with_capacity(self.size);
        for _ in 0..self.size {
            population.push(FloatVec { values: self.rand_data() });
        }
        population
    }
}

#[derive(Clone)]
pub struct InitRandomNaiveBitVecPopulation {
    pub size: usize,
    pub bits_count: usize
}

impl InitRandomNaiveBitVecPopulation {
    fn rand_data(&self) -> Vec<u8>  {
        let mut bits = Vec::<u8>::with_capacity(self.bits_count);
        for _ in 0..self.bits_count {
            bits.push(if rand::random::<bool>() { 1u8 } else { 0u8 });
        }
        bits
    }
}

impl InitFunc<NaiveBitVec> for InitRandomNaiveBitVecPopulation {
    fn init(&self) -> NaiveBitVec {
        NaiveBitVec { bits: self.rand_data() }
    }
}

impl InitPopulation<NaiveBitVec> for InitRandomNaiveBitVecPopulation {
    fn init(&self) -> Vec<NaiveBitVec> {
        let mut population = Vec::<NaiveBitVec>::with_capacity(self.size);
        for _ in 0..self.size {
            population.push(NaiveBitVec { bits: self.rand_data() });
        }
        population
    }
}