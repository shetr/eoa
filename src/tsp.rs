use std::mem::swap;

use crate::opt_traits::*;
use crate::opt_data::*;
use crate::crossover::*;
use crate::termination::*;
use rand::Rng;

#[derive(Clone)]
pub struct TspPermutation {
    pub vert_perm: Vec<usize>
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

#[derive(Clone)]
pub struct DistanceHalfMatrix {
    vert_count: usize,
    distances: Vec<f64>
}

impl DistanceHalfMatrix {
    pub fn new(vert_count: usize) -> Self {
        DistanceHalfMatrix { vert_count: vert_count, distances: vec![0.0f64; Self::get_dist_matrix_size(vert_count)] }
    }

    pub fn from(vert_count: usize, distances: Vec<f64>) -> Self {
        if Self::get_dist_matrix_size(vert_count) != distances.len() {
            panic!("incorrect size of distances array");
        }
        DistanceHalfMatrix { vert_count: vert_count, distances: distances.clone() }
    }

    pub fn get(&self, v1: usize, v2: usize) -> f64 {
        self.distances[self.get_index(v1, v2)]
    }

    pub fn set(&mut self, v1: usize, v2: usize, dist: f64) {
        let index = self.get_index(v1, v2);
        self.distances[index] = dist;
    }

    pub fn get_index(&self, mut v1: usize, mut v2: usize) -> usize {
        v1 = v1 % self.vert_count;
        v2 = v2 % self.vert_count;
        if v1 > v2 {
            swap(&mut v1, &mut v2);
        }
        v1 + Self::get_dist_matrix_size(v2)
    }

    fn get_dist_matrix_size(vert_count: usize) -> usize {
        vert_count + (vert_count * vert_count - vert_count) / 2
    }
}

pub struct TspFitness {
    pub distances: DistanceHalfMatrix
}

impl FitnessFunc<TspPermutation> for TspFitness {
    fn eval(&self, data: &TspPermutation) -> f64 {
        let mut total_len = 0f64;
        for i in 0..data.dim() {
            total_len += self.distances.get(data.vert_perm[(i + 1) % data.dim()], data.vert_perm[i]);
        }
        total_len
    }
}

#[derive(Clone)]
pub struct InitTspPopulation {
    pub size: usize,
    pub vert_count: usize
}

impl InitTspPopulation {
    fn gen_vert_perm(&self, place_used: &mut Vec<bool>) -> Vec::<usize> {
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
        vert_perm
    }
}

impl InitPopulation<TspPermutation> for InitTspPopulation {
    fn init(&self) -> Vec<TspPermutation> {
        let mut population = Vec::<TspPermutation>::with_capacity(self.size);
        let mut place_used: Vec<bool> = vec![false; self.vert_count];
        for _ in 0..self.size {
            population.push(TspPermutation { vert_perm: self.gen_vert_perm(&mut place_used) });
        }
        population
    }
}

impl InitFunc<TspPermutation> for InitTspPopulation {
    fn init(&self) -> TspPermutation {
        let mut place_used: Vec<bool> = vec![false; self.vert_count];
        TspPermutation { vert_perm: self.gen_vert_perm(&mut place_used) }
    }
}

#[derive(Clone)]
pub struct TspMovePerturbation {
}

pub fn tsp_move_perturbation<V: Copy>(perm: &mut Vec<V>) {
    let move_from = rand::thread_rng().gen_range(0..perm.len());
    let move_to = rand::thread_rng().gen_range(0..perm.len());
    let vert_to_move = perm[move_from];
    if move_to >= move_from {
        for i in move_from..move_to {
            perm[i] = perm[i + 1];
        }
    } else {
        for i in (move_to..move_from).rev() {
            perm[i + 1] = perm[i];
        }
    }
    perm[move_to] = vert_to_move;
}

impl PerturbeMutOp<TspPermutation> for TspMovePerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        tsp_move_perturbation(&mut data.vert_perm);
    }
}

#[derive(Clone)]
pub struct TspSwapPerturbation {
}

pub fn tsp_swap_perturbation<V: Copy>(perm: &mut Vec<V>) {
    let pos1 = rand::thread_rng().gen_range(0..perm.len());
    let pos2 = rand::thread_rng().gen_range(0..perm.len());
    let temp = perm[pos1];
    perm[pos1] = perm[pos2];
    perm[pos2] = temp;
}

impl PerturbeMutOp<TspPermutation> for TspSwapPerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        tsp_swap_perturbation(&mut data.vert_perm);
    }
}

#[derive(Clone)]
pub struct TspReversePerturbation {
}

pub fn tsp_reverse_perturbation<V: Copy>(perm: &mut Vec<V>) {
    let from = rand::thread_rng().gen_range(0..perm.len());
    let mut to = rand::thread_rng().gen_range(0..perm.len());
    if to < from {
        to += perm.len();
    }
    let range_len = to - from;
    for offset in 0..(range_len / 2) {
        let pos1 = (from + offset) % perm.len();
        let pos2 = (to - offset) % perm.len();
        let temp = perm[pos1];
        perm[pos1] = perm[pos2];
        perm[pos2] = temp;
    }
}

impl PerturbeMutOp<TspPermutation> for TspReversePerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        tsp_reverse_perturbation(&mut data.vert_perm);
    }
}

pub struct TspCycleCrossover {
}

impl CrossoverFun<usize> for TspCycleCrossover {
    fn crossover_fun(&self, parents: [&Vec<usize>; 2], offsprings: [&mut Vec<usize>; 2]) {
        for i in 0..parents[0].len() {
            for o in 0..2 {
                offsprings[o].push(parents[o][i].clone());
            }
        }
        let start_index = rand::thread_rng().gen_range(0..parents[0].len());
        let mut current_index = start_index;
        if offsprings[1][current_index] == offsprings[0][current_index] {
            return;
        }
        loop {
            let next_vert = offsprings[1][current_index];
            let prev_index = current_index;
            let mut index_found = false;
            for i in 0..offsprings[0].len() {
                if offsprings[0][i] == next_vert {
                    current_index = i;
                    index_found = true;
                    break;
                }
            }
            let temp = offsprings[0][prev_index];
            offsprings[0][prev_index] = offsprings[1][prev_index];
            offsprings[1][prev_index] = temp;
            if !index_found {
                break;
            }
        }
    }
}

impl Crossover<TspPermutation> for TspCycleCrossover {
    fn crossover(&self, population: &Vec<TspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<TspPermutation>) {
        crossover_vec_data(population, parents_indices, offsprings, self);
    }
}

pub struct TspOrderCrossover {
}

impl CrossoverFun<usize> for TspOrderCrossover {
    fn crossover_fun(&self, parents: [&Vec<usize>; 2], offsprings: [&mut Vec<usize>; 2]) {
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
}

impl Crossover<TspPermutation> for TspOrderCrossover {
    fn crossover(&self, population: &Vec<TspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<TspPermutation>) {
        crossover_vec_data(population, parents_indices, offsprings, self);
    }
}

impl TerminationCond<TspPermutation> for MaxIterTerminationCond {
    fn eval(&self, iter: usize, _: f64) -> bool {
        return iter >= self.n_iters;
    }
}