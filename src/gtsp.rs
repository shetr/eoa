use std::rc::Rc;

use crate::*;

pub struct GtspProblem {
    pub vert_count: usize,
    pub groups: Vec<usize>,
    pub distances: DistanceHalfMatrix
}

#[derive(Clone)]
pub struct GtspPermutation {
    pub spec: Rc<GtspProblem>,
    pub groups_perm: Vec<usize>,
    pub group_vert_indices: Vec<usize>
}

impl OptData for GtspPermutation {
    fn dim(&self) -> usize {
        self.spec.vert_count
    }
}

pub struct GtspFitness {
}