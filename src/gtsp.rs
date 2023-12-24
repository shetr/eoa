use std::rc::Rc;
use rand::Rng;

use crate::*;

#[derive(Clone)]
pub struct GtspProblem {
    pub vert_count: usize,
    pub best_known: f64,
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

impl PartialEq for GroupVert {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group && self.vert == other.vert
    }
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

impl GtspRandGroupVertPerturbation {
    pub fn new(groups_count: usize) -> Self {
        GtspRandGroupVertPerturbation { change_prob: 1.0 / (groups_count as f64) }
    }
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

pub fn crossover_gtsp_data<CrossoverFunT : CrossoverFun<GroupVert>>
    (population: &Vec<GtspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<GtspPermutation>, crossover_fun: &CrossoverFunT)
{
    offsprings.clear();
    for i in (0..parents_indices.len()).step_by(2) {
        if i + 1 >= parents_indices.len() {
            continue;
        }
        let parent1 = population.get(parents_indices[i]).unwrap();
        let parent2 = population.get(parents_indices[i + 1]).unwrap();
        let mut offspring1 = GtspPermutation { spec: parent1.spec.clone(), perm: Vec::<GroupVert>::with_capacity(parent1.perm.len()) };
        let mut offspring2 = GtspPermutation { spec: parent2.spec.clone(), perm: Vec::<GroupVert>::with_capacity(parent2.perm.len()) };
        let curr_parents = [&parent1.perm, &parent2.perm];
        let curr_offsprings = [&mut offspring1.perm, &mut offspring2.perm];
        crossover_fun.crossover_fun(curr_parents, curr_offsprings);
        offsprings.push(offspring1);
        offsprings.push(offspring2);
    }
}

pub fn gtsp_one_point_city_crossover(parents: [&Vec<GroupVert>; 2], offsprings: [&mut Vec<GroupVert>; 2]) {
    let split_index = rand::thread_rng().gen_range(0..parents[0].len());
    let mut group_indices = vec![[0usize;2]; parents[0].len()];
    for i in 0..parents[0].len() {
        for p in 0..2 {
            group_indices[parents[p][i].group][p] = i;
        }
    }
    for i in 0..parents[0].len() {
        for o in 0..2 {
            offsprings[o].push(parents[o][i].clone());
            let g = parents[o][i].group;
            let vert_parent = if g < split_index { o } else { 1 - o } as usize;
            let g_index = group_indices[g][vert_parent];
            offsprings[o][i].vert = parents[vert_parent][g_index].vert;
        }
    }
}

// Sum of the probabilities should be <= 1
struct GtspCycleCrossover {
    pub city_prob: f64,
    pub cycle_prob: f64
}

impl GtspCycleCrossover {
    pub fn new() -> Self { GtspCycleCrossover { city_prob: 0.5, cycle_prob: 0.5 }}
}

impl CrossoverFun<GroupVert> for GtspCycleCrossover {
    fn crossover_fun(&self, parents: [&Vec<GroupVert>; 2], offsprings: [&mut Vec<GroupVert>; 2]) {
        let r = rand::random::<f64>();
        if r <= self.city_prob {
            gtsp_one_point_city_crossover(parents, offsprings);
        } else if (r - self.city_prob) <= self.cycle_prob {
            tsp_cycle_crossover(parents, offsprings);
        } else {
            for p in 0..2 {
                *offsprings[p] = parents[p].clone();
            }
        }
    }
}

impl Crossover<GtspPermutation> for GtspCycleCrossover {
    fn crossover(&self, population: &Vec<GtspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<GtspPermutation>) {
        crossover_gtsp_data(population, parents_indices, offsprings, self);
    }
}

// Sum of the probabilities should be <= 1
struct GtspOrderCrossover {
    pub city_prob: f64,
    pub order_prob: f64
}

impl GtspOrderCrossover {
    pub fn new() -> Self { GtspOrderCrossover { city_prob: 0.5, order_prob: 0.5 }}
}

impl CrossoverFun<GroupVert> for GtspOrderCrossover {
    fn crossover_fun(&self, parents: [&Vec<GroupVert>; 2], offsprings: [&mut Vec<GroupVert>; 2]) {
        let r = rand::random::<f64>();
        if r <= self.city_prob {
            gtsp_one_point_city_crossover(parents, offsprings);
        } else if (r - self.city_prob) <= self.order_prob {
            tsp_order_crossover(parents, offsprings);
        } else {
            for p in 0..2 {
                *offsprings[p] = parents[p].clone();
            }
        }
    }
}

impl Crossover<GtspPermutation> for GtspOrderCrossover {
    fn crossover(&self, population: &Vec<GtspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<GtspPermutation>) {
        crossover_gtsp_data(population, parents_indices, offsprings, self);
    }
}

// Sum of the probabilities should be <= 1
struct GtspGeneralCrossover {
    pub city_prob: f64,
    pub cycle_prob: f64,
    pub order_prob: f64
}

impl CrossoverFun<GroupVert> for GtspGeneralCrossover {
    fn crossover_fun(&self, parents: [&Vec<GroupVert>; 2], offsprings: [&mut Vec<GroupVert>; 2]) {
        let r = rand::random::<f64>();
        if r <= self.city_prob {
            gtsp_one_point_city_crossover(parents, offsprings);
        } else if (r - self.city_prob) <= self.cycle_prob {
            tsp_cycle_crossover(parents, offsprings);
        } else if (r - self.city_prob - self.cycle_prob) <= self.order_prob {
            tsp_order_crossover(parents, offsprings);
        } else {
            for p in 0..2 {
                *offsprings[p] = parents[p].clone();
            }
        }
    }
}

impl Crossover<GtspPermutation> for GtspGeneralCrossover {
    fn crossover(&self, population: &Vec<GtspPermutation>, parents_indices: &Vec<usize>, offsprings: &mut Vec<GtspPermutation>) {
        crossover_gtsp_data(population, parents_indices, offsprings, self);
    }
}
