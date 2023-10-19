use crate::funcs::*;
use crate::helpers::*;

pub struct Solution<T: Clone + Copy> {
    pub value: Vec<T>,
    pub fitness: f64
}

pub struct Statistics {
    pub fitness: Vec<f64>
}

pub fn local_search<T: Clone + Copy, FitnessT : FitnessFunc<T>, PerturbeMutOpT : PerturbeMutOp<T>, TerminationCondT: TerminationCond<T>>
    (fitness: &FitnessT, mut perturbe_mut_op: PerturbeMutOpT, termination_cond: &TerminationCondT, bounds: &[Bounds], init_value: &[T])
    -> (Solution<T>, Statistics)
{
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut temp_value = Vec::<f64>::with_capacity(bounds.len());
    let mut curr_value = Vec::<T>::from(init_value);
    let mut curr_fitness = fitness.eval(&curr_value, bounds, &mut temp_value);
    let mut next_value = Vec::<T>::from(init_value);
    stats.fitness.push(curr_fitness);
    while !termination_cond.eval(iter, diff) {
        next_value.copy_from_slice(&curr_value);
        perturbe_mut_op.eval(&mut next_value);
        let next_fitness = fitness.eval(&next_value, bounds, &mut temp_value);
        diff = next_fitness - curr_fitness;
        let is_better = next_fitness < curr_fitness;
        perturbe_mut_op.update(is_better, init_value.len());
        if is_better {
            curr_value.copy_from_slice(&next_value);
            curr_fitness = next_fitness;
        }
        stats.fitness.push(curr_fitness);
        iter += 1;
    }
    (Solution::<T> { value: curr_value, fitness: curr_fitness }, stats)
}

