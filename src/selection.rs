use crate::opt_traits::*;
use crate::utils::*;
use rand::Rng;
use std::cmp::Ordering;

pub struct TournamentSelection {
    pub select_count: usize,
    pub rounds_count: usize
}

impl<T : OptData, F: Fitness> Selection<T, F> for TournamentSelection {
    fn select(&self, fitness: &Vec<F>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        for _ in 0..self.select_count {
            let mut opt_best_index: Option<usize> = None;
            for _ in 0..self.rounds_count {
                let index = rand::thread_rng().gen_range(0..fitness.len());
                if let Some(best_index) = opt_best_index {
                    if F::opt_cmp(&fitness[index], &fitness[best_index]) == Ordering::Less {
                        opt_best_index = Some(best_index);
                    }
                } else {
                    opt_best_index = Some(index);
                }
                
            }
            parents_indices.push(opt_best_index.unwrap_or(0));
        }
    }
}

struct ParentEntry<F: Fitness> {
    pub fitness: F,
    pub index: usize
}

impl<F: Fitness> Ord for ParentEntry<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        let fitness_cmp = F::opt_cmp(&self.fitness, &other.fitness);
        if fitness_cmp != Ordering::Equal {
            fitness_cmp
        } else {
            self.index.cmp(&other.index)
        }
    }
}

impl<F: Fitness> PartialOrd for ParentEntry<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Fitness> PartialEq for ParentEntry<F> {
    fn eq(&self, other: &Self) -> bool {
        F::opt_cmp(&self.fitness, &other.fitness) == Ordering::Equal && self.index.cmp(&other.index) == Ordering::Equal
    }
}

impl<F: Fitness> Eq for ParentEntry<F> {}

pub struct RankSelection {
    pub select_count: usize
}

impl<T : OptData, F: Fitness> Selection<T, F> for RankSelection {
    fn select(&self, fitness: &Vec<F>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        let mut best_queue = LimitedBinaryHeap::<ParentEntry<F>>::new(self.select_count);
        for i in 0..fitness.len() {
            best_queue.push(ParentEntry::<F> { fitness: fitness[i].clone(), index: i });
        }
        for entry in best_queue.iter() {
            parents_indices.push(entry.index);
        }
    }
}

pub struct RandomSelection {
    pub select_count: usize
}

impl<T : OptData, F: Fitness> Selection<T, F> for RandomSelection {
    fn select(&self, fitness: &Vec<F>, parents_indices: &mut Vec<usize>) {
        parents_indices.clear();
        for _ in 0..self.select_count {
            parents_indices.push(rand::thread_rng().gen_range(0..fitness.len()));
        }
    }
}

pub struct RouletteWheelSelection {
    pub select_count: usize
}

impl<T : OptData> Selection<T, f64> for RouletteWheelSelection {
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