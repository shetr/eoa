
pub trait OptData : Clone {
    fn dim(&self) -> usize;
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
    fn select(&self, population: &Vec<T>, fitness: &Vec<f64>, parents: &mut Vec<T>);
}

pub trait Crossover<T : OptData> {
    fn crossover(&self, parents: &Vec<T>, offsprings: &mut Vec<T>);
}

pub trait ReplacementStrategy<T : OptData> {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>);
}