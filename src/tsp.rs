use std::mem::swap;

use crate::opt_traits::*;
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

struct TspMovePerturbation {
}

impl PerturbeMutOp<TspPermutation> for TspMovePerturbation {
    fn eval(&self, data: &mut TspPermutation) {
        let move_from = rand::thread_rng().gen_range(0..data.dim());
        let move_to = rand::thread_rng().gen_range(move_from..data.dim());
        let vert_to_move = data.vert_perm[move_from];
        for i in move_from..move_to {
            data.vert_perm[i] = data.vert_perm[i+1];
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
        swap(&mut data.vert_perm[pos1], &mut data.vert_perm[pos2]);
    }
}
