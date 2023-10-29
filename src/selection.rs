use crate::opt_traits::*;
use crate::utils::*;
use rand::Rng;
use std::cmp::Ordering;

pub struct TournamentSelection {
    select_count: usize,
    rounds_count: usize
}

impl<T : OptData> Selection<T> for TournamentSelection {
    fn select(&self, fitness: &Vec<f64>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        for _ in 0..self.select_count {
            let mut best_index = 0usize;
            let mut best_fitness = f64::INFINITY;
            for _ in 0..self.rounds_count {
                let index = rand::thread_rng().gen_range(0..fitness.len());
                if fitness[index] < best_fitness {
                    best_index = index;
                    best_fitness = fitness[index];
                }
            }
            parents_indices.push(best_index);
        }
    }
}

struct ParentEntry {
    fitness: f64,
    index: usize
}

impl Ord for ParentEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let fitness_equal = self.fitness == other.fitness;
        if fitness_equal {
            if self.fitness < other.fitness { Ordering::Less } else { Ordering::Greater }
        } else {
            self.index.cmp(&other.index)
        }
    }
}

impl PartialOrd for ParentEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ParentEntry {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness && self.index == other.index
    }
}

impl Eq for ParentEntry {}

pub struct RankSelection {
    select_count: usize
}

impl<T : OptData> Selection<T> for RankSelection {
    fn select(&self, fitness: &Vec<f64>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        let mut best_queue = LimitedBinaryHeap::<ParentEntry>::new(self.select_count);
        for i in 0..fitness.len() {
            best_queue.push(ParentEntry { fitness: fitness[i], index: i });
        }
        for entry in best_queue.iter() {
            parents_indices.push(entry.index);
        }
    }
}

pub struct RandomSelection {
    select_count: usize
}

impl<T : OptData> Selection<T> for RandomSelection {
    fn select(&self, fitness: &Vec<f64>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        for _ in 0..self.select_count {
            parents_indices.push(rand::thread_rng().gen_range(0..fitness.len()));
        }
    }
}

pub struct RouletteWheelSelection {
    select_count: usize
}

impl<T : OptData> Selection<T> for RouletteWheelSelection {
    fn select(&self, fitness: &Vec<f64>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        let fitness_sum: f64 = fitness.iter().sum();
        let select_step = fitness_sum / (self.select_count as f64);
        let mut current_offset = rand::random::<f64>() * select_step;
        let mut fitness_acc = 0f64;
        for i in 0..fitness.len() {
            let next_fitness_acc = fitness_acc + fitness[i];
            while fitness_acc <= current_offset && next_fitness_acc > current_offset {
                parents_indices.push(i);
                current_offset += select_step;
            }
            fitness_acc = next_fitness_acc;
        }
    }
}