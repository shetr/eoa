use crate::opt_traits::*;

pub struct GenerationalReplacementStrategy {
}

impl<T : OptData> ReplacementStrategy<T> for GenerationalReplacementStrategy {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>) {
        population.clear();
        fitness.clear();
        for offspring in offsprings {
            population.push(offspring.clone());
        }
        for offspring_fitness in offsprings_fitness {
            fitness.push(*offspring_fitness);
        }
    }
}

// TODO: implement:
// Random (join old and new population and choose survivors randomly).
// Truncation (join old and new population and throw away the worst individuals).
// Turnament.
// Roulette.
// Rank-based.
