use std::cmp::Ordering;

use crate::opt_traits::*;
use crate::opt_data::*;

// TODO: generalize evolutionary search enough so we can simulate local_search by calling with specific parameters, then remove this one
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

pub fn local_search_evolutionary_api<
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
        _selection: &SelectionT,
        _crossover: &CrossoverT,
        perturbe_mut_op: PerturbeMutOpT,
        _replacement_strategy: &ReplacementStrategyT,
        termination_cond: &TerminationCondT
    )
    -> (Solution<T>, Statistics)
{
    local_search(fitness_func, init_population, perturbe_mut_op, termination_cond)
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

// TODO: maybe put it into struct where parameters are stored at construction. That would also simplify generalization for local search
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
        replacement_strategy: &ReplacementStrategyT,
        termination_cond: &TerminationCondT
    )
    -> (Solution<T>, Statistics)
{
    let mut population = InitPopulation::init(&init_population);
    let mut fitness = Vec::<f64>::with_capacity(population.len());
    let mut parents_indices = Vec::<usize>::new();
    let mut offsprings = Vec::<T>::new();
    let mut offsprings_fitness = Vec::<f64>::new();
    evaluate_population(fitness_func, &population, &mut fitness);
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut best_index = find_best(&fitness);
    let mut best_value = population[best_index].clone();
    let mut best_fitness = fitness[best_index];
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    stats.fitness.push(fitness[best_index]);
    while !termination_cond.eval(iter, diff) {
        selection.select(&fitness, &mut parents_indices);
        crossover.crossover(&population, &parents_indices, &mut offsprings);
        mutate(&mut offsprings, &perturbe_mut_op);
        evaluate_population(fitness_func, &offsprings, &mut offsprings_fitness);
        let prev_best_fitness = fitness[best_index];
        replacement_strategy.replace(&mut population, &mut fitness, &offsprings, &offsprings_fitness);
        best_index = find_best(&fitness);
        let curr_best_fitness = fitness[best_index];
        if curr_best_fitness < best_fitness {
            best_fitness = curr_best_fitness;
            best_value = population[best_index].clone();
        }
        diff = curr_best_fitness - prev_best_fitness;
        perturbe_mut_op.update(diff, population[0].dim());
        stats.fitness.push(best_fitness);
        iter += 1;
    }
    (Solution::<T> { value: best_value, fitness: best_fitness }, stats)
}

pub type EvolutionarySearchFun<
    T: OptData,
    FitnessFuncT : FitnessFunc<T>,
    InitPopulationT: InitPopulation<T>,
    SelectionT: Selection<T>,
    CrossoverT: Crossover<T>,
    PerturbeMutOpT: PerturbeMutOp<T>,
    ReplacementStrategyT: ReplacementStrategy<T>,
    TerminationCondT: TerminationCond<T>
    > =
    fn(
        fitness_func: &mut FitnessFuncT,
        init_population: InitPopulationT,
        selection: &SelectionT,
        crossover: &CrossoverT,
        perturbe_mut_op: PerturbeMutOpT,
        replacement_strategy: &ReplacementStrategyT,
        termination_cond: &TerminationCondT
    )
    -> (Solution<T>, Statistics);

pub struct EvolutionarySearchFunCall<
    'a,
    T: OptData,
    FitnessFuncT : FitnessFunc<T>,
    InitPopulationT: InitPopulation<T>,
    SelectionT: Selection<T>,
    CrossoverT: Crossover<T>,
    PerturbeMutOpT: PerturbeMutOp<T>,
    ReplacementStrategyT: ReplacementStrategy<T>,
    TerminationCondT: TerminationCond<T>
    > {
    pub fitness_func: &'a mut FitnessFuncT,
    pub init_population: &'a InitPopulationT,
    pub selection: &'a SelectionT,
    pub crossover: &'a CrossoverT,
    pub perturbe_mut_op: &'a PerturbeMutOpT,
    pub replacement_strategy: &'a ReplacementStrategyT,
    pub termination_cond: &'a TerminationCondT,
    pub search_fun: EvolutionarySearchFun<T, FitnessFuncT, InitPopulationT, SelectionT, CrossoverT, PerturbeMutOpT, ReplacementStrategyT, TerminationCondT>
}

impl<
'a,
T: OptData,
FitnessFuncT : FitnessFunc<T>,
InitPopulationT: InitPopulation<T>,
SelectionT: Selection<T>,
CrossoverT: Crossover<T>,
PerturbeMutOpT: PerturbeMutOp<T>,
ReplacementStrategyT: ReplacementStrategy<T>,
TerminationCondT: TerminationCond<T>
> EvolutionarySearchFunCall<'a, T, FitnessFuncT, InitPopulationT, SelectionT, CrossoverT, PerturbeMutOpT, ReplacementStrategyT, TerminationCondT> {
    pub fn search(&mut self) -> Statistics {
        (self.search_fun)(self.fitness_func, self.init_population.clone(), self.selection, self.crossover, self.perturbe_mut_op.clone(), self.replacement_strategy, self.termination_cond).1
    }
}

fn dominates(f1: &Vec<f64>, f2: &Vec<f64>) -> Ordering {
    let mut at_least_one_better = false;
    for i in 0..f1.len() {
        if f1[i] > f2[i] {
            return Ordering::Greater;
        } else if f1[i] < f2[i] {
            at_least_one_better = true;
        }
    }
    if at_least_one_better {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn eval_fronts<T: OptData>(fitness: &Vec<Vec<f64>>, front_indices: &mut Vec<usize>, front: &mut Vec<usize>, fronts_counts: &mut Vec<usize>) {
    for i in 0..front_indices.len() {
        front_indices[i] = i;
    }
    front_indices.sort_by(|a, b| {
        dominates(&fitness[*a], &fitness[*b])
    });
    fronts_counts.clear();
    front[0] = 0;
    fronts_counts.push(1);
    for i in 1..front_indices.len() {
        let i_curr = front_indices[i];
        let i_prev = front_indices[i - 1];
        if dominates(&fitness[i_prev], &fitness[i_curr]) == Ordering::Less {
            front[i_curr] = front[i_prev] + 1;
            fronts_counts.push(1);
        } else {
            front[i_curr] = front[i_prev];
            fronts_counts[front[i_prev]] += 1;
        }
    }
}

fn eval_crowding_dist<T: OptData>(fitness: &Vec<Vec<f64>>, fronts_counts: &Vec<usize>, front_indices: &Vec<usize>, crowding_dist: &mut Vec<f64>) {
    let dim = fitness[0].len();
    let mut front_start = 0usize;
    let mut f_size = Vec::<f64>::with_capacity(dim);
    for m in 0..dim {
        let mut f_min = f64::INFINITY;
        let mut f_max = f64::NEG_INFINITY;
        for i in 0..fitness.len() {
            if fitness[i][m] < f_min {
                f_min = fitness[i][m];
            }
            if fitness[i][m] > f_max {
                f_max = fitness[i][m];
            }
        }
        f_size.push(f_max - f_min);
    }
    for front in 0..fronts_counts.len() {
        let front_end = front_start + fronts_counts[front];
        for i in &front_indices[front_start..front_end] {
            crowding_dist[*i] = 0.0;
        }
        for m in 0..dim {
            front_indices[front_start..front_end].sort_by(|a, b| {
                fitness[*a][m].total_cmp(&fitness[*b][m])
            });
            crowding_dist[front_indices[front_start]] = f64::INFINITY;
            crowding_dist[front_indices[front_end]] = f64::INFINITY;
            for i in &front_indices[(front_start + 1)..(front_end - 1)] {
                let i_prev = front_indices[i - 1];
                let i_curr = front_indices[*i];
                let i_next = front_indices[i + 1];
                crowding_dist[i_curr] += (fitness[i_prev][m] - fitness[i_next][m]).abs() / f_size[m];
            }
        }
        front_indices[front_start..front_end].sort_by(|a, b| {
            crowding_dist[*a].total_cmp(&crowding_dist[*b]).reverse()
        });
        front_start = front_end;
    }
}

pub fn multi_obj_evolutionary_search<
        T: OptData,
        MultiObjFitnessFuncT : MultiObjFitnessFunc<T>,
        InitPopulationT: InitPopulation<T>,
        SelectionT: Selection<T>,
        CrossoverT: Crossover<T>,
        PerturbeMutOpT: PerturbeMutOp<T>,
        ReplacementStrategyT: ReplacementStrategy<T>,
        TerminationCondT: TerminationCond<T>
    >(
        fitness_func: &mut MultiObjFitnessFuncT,
        init_population: InitPopulationT,
        selection: &SelectionT,
        crossover: &CrossoverT,
        mut perturbe_mut_op: PerturbeMutOpT,
        replacement_strategy: &ReplacementStrategyT,
        termination_cond: &TerminationCondT
    )
{
    let mut population = InitPopulation::init(&init_population);
    let mut fitness = Vec::<Vec<f64>>::with_capacity(population.len());
    let mut front_indices = Vec::<usize>::with_capacity(population.len());
    let mut front = Vec::<usize>::with_capacity(population.len());
    let mut fronts_counts = Vec::<usize>::new();
    let mut crowding_dist = Vec::<f64>::with_capacity(population.len());
    let mut parents_indices = Vec::<usize>::new();
    let mut offsprings = Vec::<T>::new();
    let mut offsprings_fitness = Vec::<f64>::new();
    fitness_func.eval_population(&mut population, &mut fitness);
    eval_fronts(&fitness, &mut front_indices, &mut front, &mut fronts_counts);
    eval_crowding_dist(&fitness, &fronts_counts, &front_indices, &mut crowding_dist);
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut best_index = find_best(&fitness);
    let mut best_value = population[best_index].clone();
    let mut best_fitness = fitness[best_index];
    let mut stats = Statistics { fitness: Vec::<f64>::new() };
    stats.fitness.push(fitness[best_index]);
    while !termination_cond.eval(iter, diff) {
        selection.select(&fitness, &mut parents_indices);
        crossover.crossover(&population, &parents_indices, &mut offsprings);
        mutate(&mut offsprings, &perturbe_mut_op);
        evaluate_population(fitness_func, &offsprings, &mut offsprings_fitness);
        let prev_best_fitness = fitness[best_index];
        replacement_strategy.replace(&mut population, &mut fitness, &offsprings, &offsprings_fitness);
        best_index = find_best(&fitness);
        let curr_best_fitness = fitness[best_index];
        if curr_best_fitness < best_fitness {
            best_fitness = curr_best_fitness;
            best_value = population[best_index].clone();
        }
        diff = curr_best_fitness - prev_best_fitness;
        perturbe_mut_op.update(diff, population[0].dim());
        stats.fitness.push(best_fitness);
        iter += 1;
    }
    (Solution::<T> { value: best_value, fitness: best_fitness }, stats)
}
