
pub trait OptData : Clone {
    fn dim(&self) -> usize;
}

pub trait GeneralFitnessFunc<T: OptData, F> {
    fn eval_general(&mut self, data: &T, out: &mut F);
}

pub trait FitnessFunc<T: OptData> {
    fn eval(&mut self, data: &T) -> f64;
}

impl<T: OptData, FitnessFuncT : FitnessFunc<T>> GeneralFitnessFunc<T, f64> for FitnessFuncT {
    fn eval_general(&mut self, data: &T, out: &mut f64) {
        *out = self.eval(data);
    }
}

pub trait MultiObjFitnessFunc<T: OptData> : GeneralFitnessFunc<T, Vec<f64>> {
    fn eval(&mut self, data: &T, out: &mut Vec<f64>);
}

impl<T: OptData, MultiObjFitnessFuncT : MultiObjFitnessFunc<T>> GeneralFitnessFunc<T, Vec<f64>> for MultiObjFitnessFuncT {
    fn eval_general(&mut self, data: &T, out: &mut Vec<f64>) {
        self.eval(data, out)
    }
}

pub trait Constraints<T: OptData> {
    fn has_constrains(&self) -> bool { false }
    fn is_feasible(&self, _data: &T) -> bool { true }
}

pub struct OptDataEntry<P, T: OptData> {
    pub data: T,
    pub props: P
}

pub trait FitnessEvaluator<P, T: OptData> {
    fn eval(data: &mut Vec<T>, data_entries: &mut Vec<OptDataEntry<P, T>>);
    fn is_better(e1: &OptDataEntry<P, T>, e2: &OptDataEntry<P, T>) -> bool;
}

pub trait ConstraintFilter<T: OptData> {
    
}

pub trait PerturbeMutOp<T: OptData> : Clone {
    fn eval(&self, data: &mut T);

    fn update(&mut self, _iter_diff: f64, _dim: usize) {}
}

pub trait TerminationCond<T: OptData> {
    fn eval(&self, iter: usize, fitness: f64) -> bool;
}

pub trait InitFunc<T : OptData> {
    fn init(&self) -> T;
}

pub trait InitPopulation<T : OptData> : InitFunc<T> + Clone {
    fn init(&self) -> Vec<T>;
}

pub trait Selection<T : OptData> {
    fn select(&self, fitness: &Vec<f64>, parents_indices: &mut Vec<usize>);
}

pub trait Crossover<T : OptData> {
    fn crossover(&self, population: &Vec<T>, parents_indices: &Vec<usize>, offsprings: &mut Vec<T>);
}

pub trait ReplacementStrategy<T : OptData> {
    fn replace(&self, population: &mut Vec<T>, fitness: &mut Vec<f64>, offsprings: &Vec<T>, offsprings_fitness: &Vec<f64>);
}