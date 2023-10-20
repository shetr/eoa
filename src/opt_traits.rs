
pub trait OptData : Clone {
    fn dim(&self) -> usize;
}

pub struct OptDataEntry<T: OptData> {
    pub data: T,
    pub fitness: f64
}

pub trait FitnessFunc<T: OptData> {
    fn eval(&mut self, data: &T) -> f64;
}

pub trait PerturbeMutOp<T: OptData> {
    fn eval(&self, data: &mut T);

    fn update(&mut self, _is_better: bool, _dim: usize) {}
}

pub trait TerminationCond<T: OptData> {
    fn eval(&self, iter: usize, fitness: f64) -> bool;
}

pub trait InitFunc<T : OptData> {
    fn init(&self) -> T;
}

pub trait InitPopulation<T : OptData> {
    fn init(&self) -> Vec<T>;
}

pub trait Selection<T : OptData> {
    fn select(population: &Vec<OptDataEntry<T>>, parents: &mut Vec<OptDataEntry<T>>);
}

pub trait Crossover<T : OptData> {
    fn crossover(parents: &Vec<OptDataEntry<T>>, offsprings: &mut Vec<OptDataEntry<T>>);
}

pub trait ReplacementStrategy<T : OptData> {
    fn replace(population: &mut Vec<OptDataEntry<T>>, offsprings: &Vec<OptDataEntry<T>>);
}