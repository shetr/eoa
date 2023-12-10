use std::{cmp::Ordering, marker::PhantomData};

use crate::*;

#[derive(Clone)]
pub struct StochasticRankFitness {
    pub fitness: f64,
    pub violations: f64,
    pub prob: f64
}

impl Fitness for StochasticRankFitness {
    fn opt_cmp(f1: &Self, f2: &Self) -> Ordering {
        let both_feasible = f1.violations <= 0.0 && f2.violations <= 0.0;
        let fitness_cmp = f1.fitness.total_cmp(&f2.fitness);
        if both_feasible {
            fitness_cmp
        } else {
            if rand::random::<f64>() < f1.prob {
                fitness_cmp
            } else {
                f1.violations.total_cmp(&f2.violations)
            }
        }
    }

    fn diff(curr: &Self, prev: &Self) -> f64 {
        curr.fitness - prev.fitness
    }
}

#[derive(Clone)]
pub struct StochasticRankFitnessTransformer<T: OptData, ConstraintsT : Constraints<T>> {
    prob: f64,
    constraints: ConstraintsT,
    _phantom: PhantomData<T>
}

impl<T: OptData, ConstraintsT : Constraints<T>> StochasticRankFitnessTransformer<T, ConstraintsT> {
    pub fn new(prob: f64, constraints: ConstraintsT) -> Self {
        StochasticRankFitnessTransformer { prob: prob, constraints: constraints, _phantom: PhantomData::<T> {} }
    }
}

impl<T: OptData, ConstraintsT : Constraints<T>> FitnessTransformer<T, f64, StochasticRankFitness> for StochasticRankFitnessTransformer<T, ConstraintsT> {
    
    fn transform(&mut self, pouplation: &Vec<T>, fitness_in: &Vec<f64>, fitness_out: &mut Vec<StochasticRankFitness>) {
        fitness_out.resize(fitness_in.len(), StochasticRankFitness { fitness: 0.0, violations: 0.0, prob: self.prob});
        for i in 0..pouplation.len() {
            fitness_out[i].fitness = fitness_in[i];
            fitness_out[i].violations = self.constraints.violations_sum(&pouplation[i]);
        }
    }
}

#[derive(Clone)]
pub struct StochasticRankSolution<T: OptData> {
    pub data: T,
    pub fitness: f64,
    pub violations: f64
}

impl<T: OptData> Solution<T, f64, StochasticRankFitness> for StochasticRankSolution<T> {
    fn from_population(population: &Vec<T>, fitness_in: &Vec<f64>, fitness_opt: &Vec<StochasticRankFitness>) -> Self {
        let mut best_index = 0usize;
        for i in 1..population.len() {
            if fitness_opt[i].violations < fitness_opt[best_index].violations {
                best_index = i;
            } else if fitness_opt[i].violations == fitness_opt[best_index].violations {
                if fitness_in[i] < fitness_in[best_index] {
                    best_index = i;
                }
            }
        }
        StochasticRankSolution { data: population[best_index].clone(), fitness: fitness_in[best_index], violations: fitness_opt[best_index].violations }
    }

    fn diff(&self, other: &Self) -> f64 {
        self.fitness - other.fitness 
    }

    fn is_better(&self, other: &Self) -> bool {
        self.violations < other.violations || (self.violations == other.violations && self.fitness < other.fitness)
    }
}

#[derive(Clone)]
pub struct StochasticRankStatistics<T: OptData> {
    pub solutions: Vec<StochasticRankSolution<T>>
}

impl<T: OptData> Statistics<T, f64, StochasticRankFitness> for StochasticRankStatistics<T> {
    fn new() -> Self {
        StochasticRankStatistics { solutions: Vec::<StochasticRankSolution<T>>::new() }
    }

    fn report_iter(&mut self, _iter: usize, population: &Vec<T>, fitness_in: &Vec<f64>, fitness_opt: &Vec<StochasticRankFitness>) {
        let mut solution = StochasticRankSolution::from_population(population, fitness_in, fitness_opt);
        if let Some(last) = self.solutions.last() {
            if last.is_better(&solution) {
                solution = last.clone();
            }
        }
        self.solutions.push(solution);
    }
}