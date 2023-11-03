use crate::opt_traits::*;
use crate::opt_data::*;
use crate::crossover::*;
use rand::Rng;

#[derive(Clone)]
struct TspPermutation {
    vert_perm: Vec<usize>
}

impl OptData for TspPermutation {
    fn dim(&self) -> usize {
        self.vert_perm.len()
    }
}

impl VecOptData<usize> for TspPermutation {
    fn new() -> Self {
        TspPermutation { vert_perm: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        TspPermutation { vert_perm: Vec::with_capacity(capacity) }
    }
    fn get(&self) -> &Vec<usize> {
        &self.vert_perm
    }
    fn get_mut(&mut self) -> &mut Vec<usize> {
        &mut self.vert_perm
    }
}

// TODO: implemnt half matrix
struct TspFitness {
    distances: Vec<f64>
}

impl FitnessFunc<TspPermutation> for TspFitness {
    fn eval(&mut self, data: &TspPermutation) -> f64 {
        let mut total_len = 0f64;
        for i in 0..data.dim() {
            // TODO: check, reimplement with half matrix
            total_len += self.distances[data.vert_perm[(i + 1) % data.dim()] + data.vert_perm[i] * data.dim()];
        }
        total_len
    }
}

pub struct InitTspPopulation {
    pub size: usize,
    pub vert_count: usize
}

impl InitPopulation<TspPermutation> for InitTspPopulation {
    fn init(&self) -> Vec<TspPermutation> {
        let mut population = Vec::<TspPermutation>::with_capacity(self.size);
        let mut place_used: Vec<bool> = vec![false; self.size];
        for _ in 0..self.size {
            place_used.fill(false);
            let mut vert_perm = Vec::<usize>::with_capacity(self.vert_count);
            for i in 0..self.vert_count {
                let mut gen_index = rand::thread_rng().gen_range(0..(self.vert_count - i));
                for j in 0..self.vert_count {
                    if j > gen_index {
                        break;
                    }
                    if place_used[j] {
                        gen_index += 1;
                    }
                }
                place_used[gen_index] = true;
                vert_perm.push(gen_index);
            }
            population.push(TspPermutation { vert_perm: vert_perm });
        }
        population
    }
}

struct TspMovePerturbation {
}

impl PerturbeMutOp<TspPermutation> for TspMovePerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        let move_from = rand::thread_rng().gen_range(0..data.dim());
        let move_to = rand::thread_rng().gen_range(0..data.dim());
        let vert_to_move = data.vert_perm[move_from];
        if move_to >= move_from {
            for i in move_from..move_to {
                data.vert_perm[i] = data.vert_perm[i + 1];
            }
        } else {
            for i in (move_to..move_from).rev() {
                data.vert_perm[i + 1] = data.vert_perm[i];
            }
        }
        data.vert_perm[move_to] = vert_to_move;
    }
}

struct TspSwapPerturbation {
}

impl PerturbeMutOp<TspPermutation> for TspSwapPerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        let pos1 = rand::thread_rng().gen_range(0..data.dim());
        let pos2 = rand::thread_rng().gen_range(0..data.dim());
        let temp = data.vert_perm[pos1];
        data.vert_perm[pos1] = data.vert_perm[pos2];
        data.vert_perm[pos2] = temp;
    }
}

struct TspReversePerturbation {
}

impl PerturbeMutOp<TspPermutation> for TspReversePerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        let from = rand::thread_rng().gen_range(0..data.dim());
        let mut to = rand::thread_rng().gen_range(0..data.dim());
        if to < from {
            to += data.dim();
        }
        let range_len = to - from;
        for offset in 0..(range_len / 2) {
            let pos1 = (from + offset) % data.dim();
            let pos2 = (to - offset) % data.dim();
            let temp = data.vert_perm[pos1];
            data.vert_perm[pos1] = data.vert_perm[pos2];
            data.vert_perm[pos2] = temp;
        }
    }
}

fn cycle_crossover(parents: [&Vec<usize>; 2], offsprings: [&mut Vec<usize>; 2]) {
    for i in 0..parents[0].len() {
        for o in 0..2 {
            offsprings[o].push(parents[o][i].clone());
        }
    }
    let start_index = rand::thread_rng().gen_range(0..parents[0].len());
    let start_vert = offsprings[0][start_index];
    let mut current_index = start_index;
    let mut next_vert = offsprings[1][current_index];
    while next_vert != start_vert {
        let temp = offsprings[0][current_index];
        offsprings[0][current_index] = offsprings[1][current_index];
        offsprings[1][current_index] = temp;
        for i in 0..offsprings[0].len() {
            if offsprings[0][i] == next_vert {
                current_index = i;
                break;
            }
        }
        next_vert = offsprings[1][current_index];
    }
    let temp = offsprings[0][current_index];
    offsprings[0][current_index] = offsprings[1][current_index];
    offsprings[1][current_index] = temp;
}

fn order_crossover(parents: [&Vec<usize>; 2], offsprings: [&mut Vec<usize>; 2]) {
    for i in 0..parents[0].len() {
        for o in 0..2 {
            offsprings[o].push(parents[o][i].clone());
        }
    }
    let from = rand::thread_rng().gen_range(0..parents[0].len());
    let to = rand::thread_rng().gen_range(from..parents[0].len());
    for o in 0..2 {
        let p1 = o;
        let p2 = 1 - p1 as usize;
        for i in from..to {
            offsprings[o][i] = parents[p1][i];
        }
        let mut p2_index = 0;
        for i in (0..from).chain(to..parents[0].len()) {
            let mut inside_p1 = true;
            while inside_p1 {
                inside_p1 = false;
                for j in from..to {
                    if parents[p1][j] == parents[p2][p2_index] {
                        inside_p1 = true;
                        p2_index += 1;
                        break;
                    }
                }
            }
            offsprings[o][i] = parents[p2][p2_index];
            p2_index += 1;
        }
    }
}

struct TspCycleCrossover {
}

impl Crossover<TspPermutation> for TspCycleCrossover {
    fn crossover(&self, population: &Vec<TspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<TspPermutation>) {
        crossover_vec_data(population, parents_indices, offsprings, cycle_crossover);
    }
}

struct TspOrderCrossover {
}

impl Crossover<TspPermutation> for TspOrderCrossover {
    fn crossover(&self, population: &Vec<TspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<TspPermutation>) {
        crossover_vec_data(population, parents_indices, offsprings, order_crossover);
    }
}