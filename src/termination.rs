use crate::opt_traits::*;
use crate::opt_data::*;

pub struct MaxIterTerminationCond {
    pub n_iters: usize
}

impl TerminationCond<NaiveBitVec> for MaxIterTerminationCond {
    fn eval(&self, iter: usize, _: f64) -> bool {
        return iter >= self.n_iters;
    }
}

impl TerminationCond<FloatVec> for MaxIterTerminationCond {
    fn eval(&self, iter: usize, _: f64) -> bool {
        return iter >= self.n_iters;
    }
}