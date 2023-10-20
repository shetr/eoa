
pub trait OptValue : Clone {
    fn dim(&self) -> usize;
}

pub trait FitnessFunc<T: OptValue> {
    fn eval(&mut self, data: &T) -> f64;
}

pub trait PerturbeMutOp<T: OptValue> {
    fn eval(&self, data: &mut T);

    fn update(&mut self, _is_better: bool, _dim: usize) {}
}

pub trait TerminationCond<T: OptValue> {
    fn eval(&self, iter: usize, fitness: f64) -> bool;
}

pub trait InitFunc<T : OptValue> {
    fn init(&self) -> T;
}