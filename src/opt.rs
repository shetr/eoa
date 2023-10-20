use crate::funcs::*;
use crate::helpers::*;

pub trait OptValue : Clone {
    fn dim(&self) -> usize;
}

#[derive(Clone)]
pub struct FloatVec {
    pub values: Vec<f64>
}

#[derive(Clone)]
pub struct NaiveBitVec {
    pub bits: Vec<u8>
}

impl OptValue for FloatVec {
    fn dim(&self) -> usize {
        self.values.len()
    }
}

impl OptValue for NaiveBitVec {
    fn dim(&self) -> usize {
        self.bits.len()
    }
}

pub trait InitFunc<T : OptValue> {
    fn init(&self) -> T;
}

pub struct InitValue<T : OptValue> {
    pub value: T
}

impl<T: OptValue> InitFunc<T> for InitValue<T> {
    fn init(&self) -> T {
        self.value.clone()
    }
}

pub struct Solution<T: OptValue> {
    pub value: T,
    pub fitness: f64
}

pub struct Statistics {
    pub fitness: Vec<f64>
}

pub fn local_search
    <T: OptValue, FitnessT : FitnessFunc<T>, PerturbeMutOpT : PerturbeMutOp<T>, TerminationCondT: TerminationCond<T>, InitFuncT: InitFunc<T>>
    (fitness: &mut FitnessT, mut perturbe_mut_op: PerturbeMutOpT, termination_cond: &TerminationCondT, init_func: InitFuncT)
    -> (Solution<T>, Statistics)
{
    let init_value = init_func.init();
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut curr_value = init_value.clone();
    let mut curr_fitness = fitness.eval(&curr_value);
    let mut next_value = init_value.clone();
    stats.fitness.push(curr_fitness);
    while !termination_cond.eval(iter, diff) {
        next_value.clone_from(&curr_value);
        perturbe_mut_op.eval(&mut next_value);
        let next_fitness = fitness.eval(&next_value);
        diff = next_fitness - curr_fitness;
        let is_better = next_fitness < curr_fitness;
        perturbe_mut_op.update(is_better, init_value.dim());
        if is_better {
            curr_value.clone_from(&next_value);
            curr_fitness = next_fitness;
        }
        stats.fitness.push(curr_fitness);
        iter += 1;
    }
    (Solution::<T> { value: curr_value, fitness: curr_fitness }, stats)
}

