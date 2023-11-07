use crate::opt_traits::*;
use crate::opt_data::*;
use rand::Rng;

pub struct IdentityCrossover {

}

impl<T : OptData> Crossover<T> for IdentityCrossover {
    fn crossover(&self, population: &Vec<T>, parents_indices: &Vec<usize>, offsprings: &mut Vec<T>) {
        offsprings.clear();
        for i in 0..parents_indices.len() {
            offsprings.push(population[parents_indices[i]].clone());
        }
    }
}

pub struct OnePointCrossover {
}

type CrossoverFun<T> = fn([&Vec<T>; 2], [&mut Vec<T>; 2]) -> ();

fn one_point_crossover<T : Clone>(parents: [&Vec<T>; 2], offsprings: [&mut Vec<T>; 2]) {
    let split_index = rand::thread_rng().gen_range(0..parents[0].len());
    for i in 0..parents[0].len() {
        for o in 0..2 {
            let offspring_parent = if i < split_index { o } else { 1 - o } as usize;
            offsprings[o].push(parents[offspring_parent][i].clone());
        }
    }
}

pub fn crossover_vec_data<T : Clone, VecOptDataT : VecOptData<T>>
    (population: &Vec<VecOptDataT>, parents_indices: &Vec<usize>, offsprings: &mut Vec<VecOptDataT>, crossover_fun: CrossoverFun<T>)
{
    offsprings.clear();
    for i in (0..parents_indices.len()).step_by(2) {
        if i + 1 >= parents_indices.len() {
            continue;
        }
        let parent1 = population.get(parents_indices[i]).unwrap();
        let parent2 = population.get(parents_indices[i + 1]).unwrap();
        let mut offspring1 = VecOptDataT::with_capacity(parent1.get().len());
        let mut offspring2 = VecOptDataT::with_capacity(parent1.get().len());
        let curr_parents = [parent1.get(), parent2.get()];
        let curr_offsprings = [offspring1.get_mut(), offspring2.get_mut()];
        crossover_fun(curr_parents, curr_offsprings);
        offsprings.push(offspring1);
        offsprings.push(offspring2);
    }
}

impl Crossover<FloatVec> for OnePointCrossover {
    fn crossover(&self, population: &Vec<FloatVec>, parents_indices: &Vec<usize>, offsprings: &mut Vec<FloatVec>) {
        crossover_vec_data(population, parents_indices, offsprings, one_point_crossover);
    }
}

impl Crossover<NaiveBitVec> for OnePointCrossover {
    fn crossover(&self, population: &Vec<NaiveBitVec>, parents_indices: &Vec<usize>, offsprings: &mut Vec<NaiveBitVec>) {
        crossover_vec_data(population, parents_indices, offsprings, one_point_crossover);
    }
}