use crate::opt_traits::*;
use crate::opt_data::*;

pub fn local_search<
        T: OptData,
        FitnessFuncT : FitnessFunc<T>,
        InitFuncT: InitFunc<T>,
        PerturbeMutOpT : PerturbeMutOp<T>,
        TerminationCondT: TerminationCond<T>
    >(
        fitness: &mut FitnessFuncT,
        init_func: InitFuncT,
        mut perturbe_mut_op: PerturbeMutOpT,
        termination_cond: &TerminationCondT
    )
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
        perturbe_mut_op.update(diff, init_value.dim());
        if is_better {
            curr_value.clone_from(&next_value);
            curr_fitness = next_fitness;
        }
        stats.fitness.push(curr_fitness);
        iter += 1;
    }
    (Solution::<T> { value: curr_value, fitness: curr_fitness }, stats)
}

pub fn evaluate_population<T: OptData, FitnessFuncT : FitnessFunc<T>>(fitness_func: &mut FitnessFuncT, population: &Vec<T>, fitness: &mut Vec<f64>)
{
    fitness.clear();
    for value in population {
        fitness.push(fitness_func.eval(value));
    }
}

pub fn find_best(fitness: &Vec<f64>) -> usize
{
    let mut best_index = 0;
    for i in 0..fitness.len() {
        if fitness[i] < fitness[best_index] {
            best_index = i;
        }
    }
    best_index
}

fn mutate<T: OptData, PerturbeMutOpT: PerturbeMutOp<T>>(population: &mut Vec<T>, perturbe_mut_op: &PerturbeMutOpT)
{
    for value in population {
        perturbe_mut_op.eval(value);
    }
}

pub fn evolutionary_search<
        T: OptData,
        FitnessFuncT : FitnessFunc<T>,
        InitPopulationT: InitPopulation<T>,
        SelectionT: Selection<T>,
        CrossoverT: Crossover<T>,
        PerturbeMutOpT: PerturbeMutOp<T>,
        ReplacementStrategyT: ReplacementStrategy<T>,
        TerminationCondT: TerminationCond<T>
    >(
        fitness_func: &mut FitnessFuncT,
        init_population: InitPopulationT,
        selection: &SelectionT,
        crossover: &CrossoverT,
        mut perturbe_mut_op: PerturbeMutOpT,
        replacement_strategy: ReplacementStrategyT,
        termination_cond: &TerminationCondT
    )
    -> (Solution<T>, Statistics)
{
    let mut population = init_population.init();
    let mut fitness = Vec::<f64>::with_capacity(population.len());
    let mut parents = Vec::<T>::new();
    let mut offsprings = Vec::<T>::new();
    let mut offsprings_fitness = Vec::<f64>::new();
    evaluate_population(fitness_func, &population, &mut fitness);
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut best_index = find_best(&fitness);
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    stats.fitness.push(fitness[best_index]);
    while !termination_cond.eval(iter, diff) {
        selection.select(&population, &fitness, &mut parents);
        crossover.crossover(&parents, &mut offsprings);
        mutate(&mut offsprings, &perturbe_mut_op);
        evaluate_population(fitness_func, &offsprings, &mut offsprings_fitness);
        let prev_best_fitness = fitness[best_index];
        replacement_strategy.replace(&mut population, &mut fitness, &offsprings, &offsprings_fitness);
        best_index = find_best(&fitness);
        let curr_best_fitness = fitness[best_index];
        diff = curr_best_fitness - prev_best_fitness;
        perturbe_mut_op.update(diff, population[0].dim());
        stats.fitness.push(curr_best_fitness);
        iter += 1;
    }
    (Solution::<T> { value: population[best_index].clone(), fitness: fitness[best_index] }, stats)
}
