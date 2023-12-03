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
    -> (SingleObjSolution<T>, SingleObjStatistics)
{
    let init_value = init_func.init();
    let mut stats = SingleObjStatistics { fitness: Vec::<f64>::new() };
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
    (SingleObjSolution::<T> { value: curr_value, fitness: curr_fitness }, stats)
}

pub fn local_search_evolutionary_api<
        T: OptData,
        FitnessFuncT : FitnessFunc<T>,
        InitPopulationT: InitPopulation<T>,
        SelectionT: Selection<T, f64>,
        CrossoverT: Crossover<T>,
        PerturbeMutOpT: PerturbeMutOp<T>,
        ReplacementStrategyT: ReplacementStrategy<T, f64, f64>,
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
    -> (SingleObjSolution<T>, SingleObjStatistics)
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

fn mutate<T: OptData, PerturbeMutOpT: PerturbeMutOp<T>>(population: &mut Vec<T>, perturbe_mut_op: &PerturbeMutOpT)
{
    for value in population {
        perturbe_mut_op.eval(value);
    }
}

fn join_populations<T: OptData, F: Fitness>(population: &mut Vec<T>, fitness: &mut Vec<F>, offsprings: &mut Vec<T>, offsprings_fitness: &mut Vec<F>)
{
    population.append(offsprings);
    fitness.append(offsprings_fitness);
}

// TODO: maybe put it into struct where parameters are stored at construction. That would also simplify generalization for local search
pub fn evolutionary_search<
        T: OptData,
        FitnessFuncT : FitnessFunc<T>,
        InitPopulationT: InitPopulation<T>,
        SelectionT: Selection<T, f64>,
        CrossoverT: Crossover<T>,
        PerturbeMutOpT: PerturbeMutOp<T>,
        ReplacementStrategyT: ReplacementStrategy<T, f64, f64>,
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
    -> (SingleObjSolution<T>, SingleObjStatistics)
{
    let mut population = InitPopulation::init(&init_population);
    let mut fitness = Vec::<f64>::with_capacity(population.len());
    // just so we can call replace
    let mut fitness2 = Vec::<f64>::with_capacity(population.len());
    let mut parents_indices = Vec::<usize>::new();
    let mut offsprings = Vec::<T>::new();
    let mut offsprings_fitness = Vec::<f64>::new();
    evaluate_population(fitness_func, &population, &mut fitness);
    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut best_index = find_best(&fitness);
    let mut best_value = population[best_index].clone();
    let mut best_fitness = fitness[best_index];
    let mut stats = SingleObjStatistics { fitness: Vec::<f64>::new() };
    stats.fitness.push(fitness[best_index]);
    while !termination_cond.eval(iter, diff) {
        selection.select(&fitness, &mut parents_indices);
        crossover.crossover(&population, &parents_indices, &mut offsprings);
        mutate(&mut offsprings, &perturbe_mut_op);
        evaluate_population(fitness_func, &offsprings, &mut offsprings_fitness);
        let prev_best_fitness = fitness[best_index];
        let offsprings_from = population.len();
        join_populations(&mut population, &mut fitness, &mut offsprings, &mut offsprings_fitness);
        fitness2.resize(fitness.len(), 0.0);
        fitness2.copy_from_slice(&fitness);
        replacement_strategy.replace(&mut population, &mut fitness, &mut fitness2, offsprings_from);
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
    (SingleObjSolution::<T> { value: best_value, fitness: best_fitness }, stats)
}


#[derive(Clone)]
pub struct NSGA2Fitness {
    pub front: usize,
    pub crowding_dist: f64
}

impl Fitness for NSGA2Fitness {
    fn opt_cmp(f1: &Self, f2: &Self) -> Ordering {
        let front_cmp = f1.front.cmp(&f2.front);
        if front_cmp == Ordering::Equal {
            f1.crowding_dist.total_cmp(&f2.crowding_dist).reverse()
        } else {
            front_cmp
        }
    }

    fn diff(curr: &Self, prev: &Self) -> f64 {
        f64::INFINITY
    }
}

struct NSGA2FitnessTransformer {
    front_indices: Vec<usize>,
    fronts_counts: Vec<usize>,
    f_size: Vec<f64>
}

impl<T: OptData> FitnessTransformer<T, Vec<f64>, NSGA2Fitness> for NSGA2FitnessTransformer {
    fn transform(&mut self, poulation: &Vec<T>, fitness_in: &Vec<Vec<f64>>, fitness_out: &mut Vec<NSGA2Fitness>) {
        fitness_out.resize(fitness_in.len(), NSGA2Fitness { front: 0, crowding_dist: 0.0});
        self.eval_fronts(fitness_in, fitness_out);
        self.eval_crowding_dist(fitness_in, fitness_out);
    }
}

impl NSGA2FitnessTransformer {

    fn new() -> Self {
        NSGA2FitnessTransformer { front_indices: Vec::<usize>::new(), fronts_counts: Vec::<usize>::new(), f_size: Vec::<f64>::new()}
    }

    fn eval_fronts(&mut self, fitness: &Vec<Vec<f64>>, nsga2_fitness: &mut Vec<NSGA2Fitness>) {
        self.front_indices.resize(fitness.len(), 0);
        for i in 0..self.front_indices.len() {
            self.front_indices[i] = i;
        }
        self.front_indices.sort_by(|a, b| {
            Vec::<f64>::opt_cmp(&fitness[*a], &fitness[*b])
        });
        self.fronts_counts.clear();
        nsga2_fitness[0].front = 0;
        self.fronts_counts.push(1);
        for i in 1..self.front_indices.len() {
            let i_curr = self.front_indices[i];
            let i_prev = self.front_indices[i - 1];
            if Vec::<f64>::opt_cmp(&fitness[i_prev], &fitness[i_curr]) == Ordering::Less {
                nsga2_fitness[i_curr].front = nsga2_fitness[i_prev].front + 1;
                self.fronts_counts.push(1);
            } else {
                nsga2_fitness[i_curr].front = nsga2_fitness[i_prev].front;
                self.fronts_counts[nsga2_fitness[i_prev].front] += 1;
            }
        }
    }

    fn eval_crowding_dist(&mut self, fitness: &Vec<Vec<f64>>, nsga2_fitness: &mut Vec<NSGA2Fitness>) {
        let dim = fitness[0].len();
        let mut front_start = 0usize;
        self.f_size.clear();
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
            self.f_size.push(f_max - f_min);
        }
        for front in 0..self.fronts_counts.len() {
            let front_end = front_start + self.fronts_counts[front];
            for i in &self.front_indices[front_start..front_end] {
                nsga2_fitness[*i].crowding_dist = 0.0;
            }
            for m in 0..dim {
                self.front_indices[front_start..front_end].sort_by(|a, b| {
                    fitness[*a][m].total_cmp(&fitness[*b][m])
                });
                nsga2_fitness[self.front_indices[front_start]].crowding_dist = f64::INFINITY;
                nsga2_fitness[self.front_indices[front_end]].crowding_dist = f64::INFINITY;
                for i in &self.front_indices[(front_start + 1)..(front_end - 1)] {
                    let i_prev = self.front_indices[i - 1];
                    let i_curr = self.front_indices[*i];
                    let i_next = self.front_indices[i + 1];
                    nsga2_fitness[i_curr].crowding_dist += (fitness[i_prev][m] - fitness[i_next][m]).abs() / self.f_size[m];
                }
            }
            self.front_indices[front_start..front_end].sort_by(|a, b| {
                nsga2_fitness[*a].crowding_dist.total_cmp(&nsga2_fitness[*b].crowding_dist).reverse()
            });
            front_start = front_end;
        }
    }

}

pub fn general_evolutionary_search<
        T: OptData,
        FIn: Fitness,
        FOpt: Fitness,
        FitnessFuncT : GeneralFitnessFunc<T, FIn>,
        InitPopulationT: InitPopulation<T>,
        SelectionT: Selection<T, FOpt>,
        CrossoverT: Crossover<T>,
        PerturbeMutOpT: PerturbeMutOp<T>,
        ReplacementStrategyT: ReplacementStrategy<T, FIn, FOpt>,
        TerminationCondT: TerminationCond<T>,
        FitnessTransformerT: FitnessTransformer<T, FIn, FOpt>,
        SolutionT: Solution<T, FIn, FOpt>,
        StatisticsT: Statistics<T, FIn, FOpt>,
    >(
        fitness_func: &mut FitnessFuncT,
        init_population: InitPopulationT,
        selection: &SelectionT,
        crossover: &CrossoverT,
        mut perturbe_mut_op: PerturbeMutOpT,
        replacement_strategy: &ReplacementStrategyT,
        termination_cond: &TerminationCondT,
        mut fitness_transformer: FitnessTransformerT
    )
    -> (SolutionT, StatisticsT)
{
    let mut population = InitPopulation::init(&init_population);
    let mut fitness = Vec::<FIn>::with_capacity(population.len());
    let mut opt_fitness = Vec::<FOpt>::with_capacity(population.len());
    let mut parents_indices = Vec::<usize>::new();
    let mut offsprings = Vec::<T>::new();
    let mut offsprings_fitness = Vec::<FIn>::new();
    fitness_func.eval_population(&mut population, &mut fitness);
    fitness_transformer.transform(&population, &fitness, &mut opt_fitness);

    let mut iter: usize = 0;
    let mut diff = f64::INFINITY;
    let mut prev_iter_solution = SolutionT::from_population(&population, &fitness, &opt_fitness);
    let mut best_solution = prev_iter_solution.clone();
    let mut stats = StatisticsT::new();
    while !termination_cond.eval(iter, diff) {
        selection.select(&opt_fitness, &mut parents_indices);
        crossover.crossover(&population, &parents_indices, &mut offsprings);
        mutate(&mut offsprings, &perturbe_mut_op);
        fitness_func.eval_population(&mut offsprings, &mut offsprings_fitness);
        let offsprings_from = population.len();
        join_populations(&mut population, &mut fitness, &mut offsprings, &mut offsprings_fitness);
        fitness_transformer.transform(&population, &fitness, &mut opt_fitness);
        replacement_strategy.replace(&mut population, &mut fitness, &mut opt_fitness, offsprings_from);
        fitness_transformer.transform(&population, &fitness, &mut opt_fitness);
        
        let curr_solution = SolutionT::from_population(&population, &fitness, &opt_fitness);
        diff = curr_solution.diff(&prev_iter_solution);
        if curr_solution.is_better(&best_solution) {
            best_solution = curr_solution.clone();
        }
        prev_iter_solution = curr_solution;
        perturbe_mut_op.update(diff, population[0].dim());
        stats.report_iter(iter, &population, &fitness, &opt_fitness);
        iter += 1;
    }
    (best_solution, stats)
}
