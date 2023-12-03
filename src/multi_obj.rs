use std::cmp::Ordering;

use crate::*;

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

    fn diff(_curr: &Self, _prev: &Self) -> f64 {
        f64::INFINITY
    }
}

struct NSGA2FitnessTransformer {
    front_indices: Vec<usize>,
    fronts_counts: Vec<usize>,
    f_size: Vec<f64>
}

impl<T: OptData> FitnessTransformer<T, Vec<f64>, NSGA2Fitness> for NSGA2FitnessTransformer {
    fn transform(&mut self, _pouplation: &Vec<T>, fitness_in: &Vec<Vec<f64>>, fitness_out: &mut Vec<NSGA2Fitness>) {
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


#[derive(Clone)]
pub struct MultiObjSolution<T: OptData> {
    pub opt_front: Vec<T>,
    pub fitness: Vec<Vec<f64>>
}

impl<T: OptData> Solution<T, Vec<f64>, NSGA2Fitness> for MultiObjSolution<T> {
    fn from_population(population: &Vec<T>, fitness_in: &Vec<Vec<f64>>, _fitness_opt: &Vec<NSGA2Fitness>) -> Self {
        let mut indices = Vec::<usize>::new();
        indices.push(0);
        for i in 1..population.len() {
            let cmp = Vec::<f64>::opt_cmp(&fitness_in[i], &fitness_in[indices[0]]);
            if cmp != Ordering::Greater {
                if cmp == Ordering::Less {
                    indices.clear();
                }
                indices.push(i);
            }
        }
        let mut res = MultiObjSolution { opt_front: Vec::<T>::new(), fitness: Vec::<Vec<f64>>::new() };
        for i in indices {
            res.opt_front.push(population[i].clone());
            res.fitness.push(fitness_in[i].clone());
        }
        res
    }

    fn diff(&self, _other: &Self) -> f64 {
        f64::INFINITY
    }

    fn is_better(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct MultiObjStatistics<T: OptData> {
    pub solutions: Vec<MultiObjSolution<T>>
}

impl<T: OptData> Statistics<T, Vec<f64>, NSGA2Fitness> for MultiObjStatistics<T> {
    fn new() -> Self {
        MultiObjStatistics { solutions: Vec::<MultiObjSolution<T>>::new() }
    }

    fn report_iter(&mut self, _iter: usize, population: &Vec<T>, fitness_in: &Vec<Vec<f64>>, fitness_opt: &Vec<NSGA2Fitness>) {
        let solution = MultiObjSolution::from_population(population, fitness_in, fitness_opt);
        self.solutions.push(solution);
    }
}