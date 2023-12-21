use std::rc::Rc;
use rand::Rng;

use crate::*;

#[derive(Clone)]
pub struct GtspProblem {
    pub vert_count: usize,
    pub groups: Vec<Vec<usize>>,
    pub distances: DistanceHalfMatrix
}

#[derive(Copy, Clone)]
pub struct GroupVert {
    // group index
    pub group: usize,
    // local vertex index in the group
    pub vert: usize
}

#[derive(Clone)]
pub struct GtspPermutation {
    pub spec: Rc<GtspProblem>,
    pub perm: Vec<GroupVert>
}

impl OptData for GtspPermutation {
    fn dim(&self) -> usize {
        self.spec.vert_count
    }
}

pub struct GtspFitness {
}

impl FitnessFunc<GtspPermutation> for GtspFitness {
    fn eval(&self, perm: &GtspPermutation) -> f64 {
        let mut total_len = 0f64;
        for i in 0..perm.perm.len() {
            let gvert1 = perm.perm[i];
            let gvert2 = perm.perm[(i + 1) % perm.perm.len()];
            let vert1 = perm.spec.groups[gvert1.group][gvert1.vert];
            let vert2 = perm.spec.groups[gvert2.group][gvert2.vert];
            total_len += perm.spec.distances.get(vert1, vert2);
        }
        total_len
    }
}

#[derive(Clone)]
pub struct InitRandomGtspPopulation {
    pub spec: Rc<GtspProblem>,
    pub size: usize
}

impl InitRandomGtspPopulation {
    fn gen_perm(&self, place_used: &mut Vec<bool>) -> GtspPermutation {
        let mut perm = GtspPermutation {
            spec: self.spec.clone(),
            perm: Vec::<GroupVert>::with_capacity(self.spec.groups.len())
        };
        place_used.fill(false);
        for i in 0..self.spec.groups.len() {
            let mut gen_group_index = rand::thread_rng().gen_range(0..(self.spec.groups.len() - i));
            for j in 0..self.spec.groups.len() {
                if j > gen_group_index {
                    break;
                }
                if place_used[j] {
                    gen_group_index += 1;
                }
            }
            place_used[gen_group_index] = true;
            let gen_vert_index = rand::thread_rng().gen_range(0..self.spec.groups[gen_group_index].len());
            perm.perm.push(GroupVert { group: gen_group_index, vert: gen_vert_index });
        }
        perm
    }
}

impl InitPopulation<GtspPermutation> for InitRandomGtspPopulation {
    fn init(&self) -> Vec<GtspPermutation> {
        let mut population = Vec::<GtspPermutation>::with_capacity(self.size);
        let mut place_used: Vec<bool> = vec![false; self.spec.vert_count];
        for _ in 0..self.size {
            population.push(self.gen_perm(&mut place_used));
        }
        population
    }
}

impl InitFunc<GtspPermutation> for InitRandomGtspPopulation {
    fn init(&self) -> GtspPermutation {
        let mut place_used: Vec<bool> = vec![false; self.spec.vert_count];
        self.gen_perm(&mut place_used)
    }
}

#[derive(Clone)]
pub struct GtspMoveGroupPerturbation {
}

impl PerturbeMutOp<GtspPermutation> for GtspMoveGroupPerturbation {
    fn eval(&self, data: &mut GtspPermutation) {
        tsp_move_perturbation(&mut data.perm);
    }
}

#[derive(Clone)]
pub struct GtspSwapGroupPerturbation {
}

impl PerturbeMutOp<GtspPermutation> for GtspSwapGroupPerturbation {
    fn eval(&self, data: &mut GtspPermutation) {
        tsp_swap_perturbation(&mut data.perm);
    }
}

#[derive(Clone)]
pub struct GtspReverseGroupPerturbation {
}

impl PerturbeMutOp<GtspPermutation> for GtspReverseGroupPerturbation {
    fn eval(&self, data: &mut GtspPermutation) {
        tsp_reverse_perturbation(&mut data.perm);
    }
}

#[derive(Clone)]
pub struct GtspRandGroupVertPerturbation {
    // recommended to set to 1/number of groups
    pub change_prob: f64
}

impl PerturbeMutOp<GtspPermutation> for GtspRandGroupVertPerturbation {
    fn eval(&self, data: &mut GtspPermutation) {
        for i in 0..data.perm.len() {
            if rand::random::<f64>() < self.change_prob {
                data.perm[i].vert = rand::thread_rng().gen_range(0..data.spec.groups[i].len());
            }
        }
    }
}
