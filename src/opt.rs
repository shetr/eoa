use crate::funcs::*;
use crate::helpers::*;

pub struct NaiveBitSolution {
    pub value: Vec<u8>,
    pub fitness: f64
}

pub struct RealSolution {
    pub value: Vec<f64>,
    pub fitness: f64
}

pub struct Statistics {
    pub fitness: Vec<f64>
}

pub fn naive_bit_local_search<Fitness : NaiveBitFitnessFunc, PerturbeMutOp : NaiveBitPerturbeMutOp, TerminationCond: NaiveBitTerminationCond>
    (fitness: &Fitness, perturbe_mut_op: &PerturbeMutOp, termination_cond: &TerminationCond, bounds: &[Bounds], init_value: &[u8])
    -> (NaiveBitSolution, Statistics)
{
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut temp_value = Vec::<f64>::with_capacity(bounds.len());
    let mut curr_value = Vec::<u8>::from(init_value);
    let mut curr_fitness = fitness.eval(&curr_value, bounds, &mut temp_value);
    let mut next_value = Vec::<u8>::from(init_value);
    stats.fitness.push(curr_fitness);
    while !termination_cond.eval(iter, diff) {
        next_value.copy_from_slice(&curr_value);
        perturbe_mut_op.eval(&mut next_value);
        let next_fitness = fitness.eval(&next_value, bounds, &mut temp_value);
        diff = next_fitness - curr_fitness;
        if next_fitness < curr_fitness {
            curr_value.copy_from_slice(&next_value);
            curr_fitness = next_fitness;
        }
        stats.fitness.push(curr_fitness);
        iter += 1;
    }
    (NaiveBitSolution { value: curr_value, fitness: curr_fitness }, stats)
}

pub fn bit_local_search() {
    
}

pub fn real_local_search<Fitness : RealFitnessFunc, PerturbeMutOp : PerturbeRealMutOp, TerminationCond: RealTerminationCond>
    (fitness: &Fitness, perturbe_mut_op: &PerturbeMutOp, termination_cond: &TerminationCond, init_value: &[f64])
    -> (RealSolution, Statistics)
{
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut curr_value = Vec::<f64>::from(init_value);
    let mut curr_fitness = fitness.eval(&curr_value);
    let mut next_value = Vec::<f64>::from(init_value);
    stats.fitness.push(curr_fitness);
    while !termination_cond.eval(iter, diff) {
        next_value.copy_from_slice(&curr_value);
        perturbe_mut_op.eval(&mut next_value);
        let next_fitness = fitness.eval(&next_value);
        diff = next_fitness - curr_fitness;
        if next_fitness < curr_fitness {
            curr_value.copy_from_slice(&next_value);
            curr_fitness = next_fitness;
        }
        stats.fitness.push(curr_fitness);
        iter += 1;
    }
    (RealSolution { value: curr_value, fitness: curr_fitness }, stats)
}