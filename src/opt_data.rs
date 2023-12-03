use std::cmp::Ordering;

use crate::opt_traits::*;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub upper: f64,
    pub lower: f64
}

#[derive(Clone)]
pub struct FloatVec {
    pub values: Vec<f64>
}

#[derive(Clone)]
pub struct NaiveBitVec {
    pub bits: Vec<u8>
}

impl OptData for FloatVec {
    fn dim(&self) -> usize {
        self.values.len()
    }
}

impl OptData for NaiveBitVec {
    fn dim(&self) -> usize {
        self.bits.len()
    }
}

pub trait VecOptData<T : Clone> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn get(&self) -> &Vec<T>;
    fn get_mut(&mut self) -> &mut Vec<T>;
}

impl VecOptData<f64> for FloatVec {
    fn new() -> Self {
        FloatVec { values: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        FloatVec { values: Vec::with_capacity(capacity) }
    }
    fn get(&self) -> &Vec<f64> {
        &self.values
    }
    fn get_mut(&mut self) -> &mut Vec<f64> {
        &mut self.values
    }
}

impl VecOptData<u8> for NaiveBitVec {
    fn new() -> Self {
        NaiveBitVec { bits: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        NaiveBitVec { bits: Vec::with_capacity(capacity) }
    }
    fn get(&self) -> &Vec<u8> {
        &self.bits
    }
    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bits
    }
}

fn find_best<F: Fitness>(fitness: &Vec<F>) -> usize
{
    let mut best_index = 0;
    for i in 0..fitness.len() {
        if F::opt_cmp(&fitness[i], &fitness[best_index]) == Ordering::Less {
            best_index = i;
        }
    }
    best_index
}

#[derive(Clone)]
pub struct EmptySolution {}

impl<T: OptData, FIn: Fitness, FOpt: Fitness> Solution<T, FIn, FOpt> for EmptySolution {
    fn from_population(_population: &Vec<T>, _fitness_in: &Vec<FIn>, _fitness_opt: &Vec<FOpt>) -> Self {
        EmptySolution { }
    }
    fn diff(&self, _other: &Self) -> f64 {
        f64::INFINITY
    }
    fn is_better(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct EmptyStatistics {}

impl<T: OptData, FIn: Fitness, FOpt: Fitness> Statistics<T, FIn, FOpt> for EmptyStatistics {
    fn new() -> Self {
        EmptyStatistics {  }
    }
    fn report_iter(&mut self, _iter: usize, _population: &Vec<T>, _fitness_in: &Vec<FIn>, _fitness_opt: &Vec<FOpt>) {}
} 

#[derive(Clone)]
pub struct SingleObjSolution<T: OptData> {
    pub value: T,
    pub fitness: f64
}

impl<T: OptData> Solution<T, f64, f64> for SingleObjSolution<T> {
    fn from_population(population: &Vec<T>, _fitness_in: &Vec<f64>, fitness_opt: &Vec<f64>) -> Self {
        let best_index = find_best(fitness_opt);
        SingleObjSolution { value: population[best_index].clone(), fitness: fitness_opt[best_index] }
    }

    fn diff(&self, other: &Self) -> f64 {
        self.fitness - other.fitness
    }

    fn is_better(&self, other: &Self) -> bool {
        self.fitness < other.fitness
    }
}

#[derive(Clone)]
pub struct SingleObjStatistics {
    pub fitness: Vec<f64>
}

impl<T: OptData> Statistics<T, f64, f64> for SingleObjStatistics {
    fn new() -> Self {
        SingleObjStatistics { fitness: Vec::<f64>::new() }
    }

    fn report_iter(&mut self, _iter: usize, _population: &Vec<T>, _fitness_in: &Vec<f64>, fitness_opt: &Vec<f64>) {
        let best_index = find_best(fitness_opt);
        self.fitness.push(fitness_opt[best_index]);
    }
}