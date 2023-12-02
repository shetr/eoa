use std::cmp::Ordering;


pub trait OptData : Clone {
    fn dim(&self) -> usize;
}

pub trait GeneralFitnessFunc<T: OptData, F> {
    fn eval_general(&mut self, data: &T, out: &mut F);

    fn is_better(f1: &F, f2: &F) -> Ordering;

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<F>);
}

pub trait FitnessFunc<T: OptData> {
    fn eval(&mut self, data: &T) -> f64;
}

impl<T: OptData, FitnessFuncT : FitnessFunc<T>> GeneralFitnessFunc<T, f64> for FitnessFuncT {
    fn eval_general(&mut self, data: &T, out: &mut f64) {
        *out = self.eval(data);
    }

    fn is_better(f1: &f64, f2: &f64) -> Ordering {
        f1.total_cmp(f2)
    }

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<f64>) {
        fitness.resize(poulation.len(), 0.0);
        for i in 0..poulation.len() {
            self.eval_general(&poulation[i], &mut fitness[i]);
        }
    }
}

pub trait MultiObjFitnessFunc<T: OptData> : GeneralFitnessFunc<T, Vec<f64>> {
    fn eval(&mut self, data: &T, out: &mut Vec<f64>);
}

impl<T: OptData, MultiObjFitnessFuncT : MultiObjFitnessFunc<T>> GeneralFitnessFunc<T, Vec<f64>> for MultiObjFitnessFuncT {
    fn eval_general(&mut self, data: &T, out: &mut Vec<f64>) {
        self.eval(data, out)
    }

    fn is_better(f1: &Vec<f64>, f2: &Vec<f64>) -> Ordering {
        let mut at_least_one_better = false;
        for i in 0..f1.len() {
            if f1[i] > f2[i] {
                return Ordering::Greater;
            } else if f1[i] < f2[i] {
                at_least_one_better = true;
            }
        }
        if at_least_one_better {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<Vec<f64>>) {
        fitness.resize(poulation.len(), Vec::<f64>::new());
        for i in 0..poulation.len() {
            self.eval_general(&poulation[i], &mut fitness[i]);
        }
    }
}

trait SingleObjectiveTransformer<T: OptData, F> {
    fn transform(&self, poulation: &Vec<T>, fitness: &Vec<F>, single_obj: &mut Vec<f64>);
}

pub trait Constraints<T: OptData> {
    fn has_constrains() -> bool { false }
    fn is_feasible(&self, _data: &T) -> bool { true }
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