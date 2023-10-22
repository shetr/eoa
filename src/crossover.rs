use crate::{opt_traits::*, FloatVec};
use rand::Rng;

struct OnePointCrossover {
}

fn one_point_crossover<T : Clone>(parent1: &Vec<T>, parent2: &Vec<T>, offspring1: &mut Vec<T>, offspring2: &mut Vec<T>) {
    offspring1.clear();
    offspring2.clear();
    let parents = [parent1, parent2];
    let offsprings = [offspring1, offspring2];
    let split_index = rand::thread_rng().gen_range(0..parent1.len());
    for i in 0..parent1.len() {
        for o in 0..2 {
            let offspring_parent = if i < split_index { o } else { 1 - o } as usize;
            offsprings[o].push(parents[offspring_parent][i].clone());
        }
    }
}

impl Crossover<FloatVec> for OnePointCrossover {
    fn crossover(&self, population: &Vec<FloatVec>, parents_indices: &Vec<usize>, offsprings: &mut Vec<FloatVec>) {
        offsprings.clear();
        for i in (0..parents_indices.len()).step_by(2) {
            if i + 1 >= parents_indices.len() {
                continue;
            }
            let parent1 = population.get(parents_indices[i]).unwrap();
            let parent2 = population.get(parents_indices[i + 1]).unwrap();
            let mut offspring1 = FloatVec { values: Vec::new() };
            let mut offspring2 = FloatVec { values: Vec::new() };
            one_point_crossover(&parent1.values, &parent2.values, &mut offspring1.values, &mut offspring2.values);
            offsprings.push(offspring1);
            offsprings.push(offspring2);
        }
    }
}