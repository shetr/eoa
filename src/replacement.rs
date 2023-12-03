use crate::opt_traits::*;

pub struct GenerationalReplacementStrategy {
}

impl<T : OptData, FIn: Fitness, FOpt: Fitness> ReplacementStrategy<T, FIn, FOpt> for GenerationalReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness_in: &mut Vec<FIn>, fitness_opt: &mut Vec<FOpt>, offsprings_from: usize) {
        population.drain(0..offsprings_from);
        fitness_in.drain(0..offsprings_from);
        fitness_opt.drain(0..offsprings_from);
    }
}

// assumes equal size of previous population and the nuber of offsprings
pub struct RandomReplacementStrategy {
    pub select_offspring_prob: f64
}

impl<T : OptData, FIn: Fitness, FOpt: Fitness> ReplacementStrategy<T, FIn, FOpt> for RandomReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness_in: &mut Vec<FIn>, fitness_opt: &mut Vec<FOpt>, offsprings_from: usize) {
        for i in 0..offsprings_from {
            if rand::random::<f64>() < self.select_offspring_prob {
                population[i] = population[i + offsprings_from].clone();
                fitness_in[i] = fitness_in[i + offsprings_from].clone();
                fitness_opt[i] = fitness_opt[i + offsprings_from].clone();
            }
        }
        population.truncate(offsprings_from);
        fitness_in.truncate(offsprings_from);
        fitness_opt.truncate(offsprings_from);
    }
}

pub struct TruncationReplacementStrategy {
}

impl<T : OptData, FIn: Fitness, FOpt: Fitness> ReplacementStrategy<T, FIn, FOpt> for TruncationReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness_in: &mut Vec<FIn>, fitness_opt: &mut Vec<FOpt>, offsprings_from: usize) {
        let population_size = population.len();
        let mut next_population = Vec::<(FIn, FOpt, T)>::with_capacity(population.len());
        for i in 0..population.len() {
            next_population.push((fitness_in[i].clone(), fitness_opt[i].clone(), population[i].clone()));
        }
        population.clear();
        fitness_in.clear();
        fitness_opt.clear();
        next_population.sort_by(|a, b| FOpt::opt_cmp(&a.1, &b.1));
        for i in 0..population_size {
            fitness_in.push(next_population[i].0.clone());
            fitness_opt.push(next_population[i].1.clone());
            population.push(next_population[i].2.clone());
        }
    }
}

// TODO: implement:
// Turnament.
// Roulette.
// Rank-based.
