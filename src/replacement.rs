use crate::opt_traits::*;

pub struct GenerationalReplacementStrategy {
}

impl<T : OptData> ReplacementStrategy<T> for GenerationalReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>) {
        population.clear();
        fitness.clear();
        population.clone_from(offsprings);
        fitness.clone_from(offsprings_fitness);
    }
}

pub struct RandomReplacementStrategy {
    pub select_offspring_prob: f64
}

impl<T : OptData> ReplacementStrategy<T> for RandomReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>) {
        for i in 0..population.len() {
            if rand::random::<f64>() < self.select_offspring_prob {
                population[i] = offsprings[i].clone();
                fitness[i] = offsprings_fitness[i];
            }
        }
    }
}

pub struct TruncationReplacementStrategy {
}

impl<T : OptData> ReplacementStrategy<T> for TruncationReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>) {
        let population_size = population.len();
        let mut next_population = Vec::<(f64, T)>::with_capacity(population.len() + offsprings.len());
        for i in 0..population.len() {
            next_population.push((fitness[i], population[i].clone()));
        }
        for i in 0..offsprings.len() {
            next_population.push((offsprings_fitness[i], offsprings[i].clone()));
        }
        population.clear();
        fitness.clear();
        next_population.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for i in 0..population_size {
            fitness.push(next_population[i].0.clone());
            population.push(next_population[i].1.clone());
        }
    }
}

// TODO: implement:
// Turnament.
// Roulette.
// Rank-based.
