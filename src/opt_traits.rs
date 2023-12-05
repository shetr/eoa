use std::cmp::Ordering;


pub trait OptData : Clone {
    fn dim(&self) -> usize;
}

pub trait Fitness : Clone {
    fn opt_cmp(f1: &Self, f2: &Self) -> Ordering;

    fn diff(curr: &Self, prev: &Self) -> f64;
}

impl Fitness for f64 {
    fn opt_cmp(f1: &f64, f2: &f64) -> Ordering {
        f1.total_cmp(f2)
    }

    fn diff(curr: &Self, prev: &Self) -> f64 {
        curr - prev
    }
}

impl Fitness for Vec<f64> {
    fn opt_cmp(f1: &Vec<f64>, f2: &Vec<f64>) -> Ordering {
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

    fn diff(curr: &Self, prev: &Self) -> f64 {
        let mut dist = 0.0;
        for i in 0..curr.len() {
            let v = (*curr)[i] - (*prev)[i];
            dist += v * v;
        }
        dist.sqrt()
    }
}

pub trait GeneralFitnessFunc<T: OptData, F: Fitness> {
    fn eval_general(&self, data: &T, out: &mut F);

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<F>);
}

pub trait FitnessFunc<T: OptData> {
    fn eval(&self, data: &T) -> f64;
}

impl<T: OptData, FitnessFuncT : FitnessFunc<T>> GeneralFitnessFunc<T, f64> for FitnessFuncT {
    fn eval_general(&self, data: &T, out: &mut f64) {
        *out = self.eval(data);
    }

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<f64>) {
        fitness.resize(poulation.len(), 0.0);
        for i in 0..poulation.len() {
            self.eval_general(&poulation[i], &mut fitness[i]);
        }
    }
}

pub trait MultiObjFitnessFunc<T: OptData> : GeneralFitnessFunc<T, Vec<f64>> {
    fn eval(&self, data: &T, out: &mut Vec<f64>);
}

impl<T: OptData, MultiObjFitnessFuncT : MultiObjFitnessFunc<T>> GeneralFitnessFunc<T, Vec<f64>> for MultiObjFitnessFuncT {
    fn eval_general(&self, data: &T, out: &mut Vec<f64>) {
        self.eval(data, out)
    }

    fn eval_population(&mut self, poulation: &mut Vec<T>, fitness: &mut Vec<Vec<f64>>) {
        fitness.resize(poulation.len(), Vec::<f64>::new());
        for i in 0..poulation.len() {
            self.eval_general(&poulation[i], &mut fitness[i]);
        }
    }
}

pub trait FitnessTransformer<T: OptData, FIn: Fitness, FOut: Fitness> {
    fn transform(&mut self, pouplation: &Vec<T>, fitness_in: &Vec<FIn>, fitness_out: &mut Vec<FOut>);
}

pub trait Constraints<T: OptData> {
    fn has_constrains() -> bool { false }
    fn is_feasible(&self, _data: &T) -> bool { true }
    fn violations_sum(&self, _data: &T) -> f64 { 0.0 }
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

pub trait Selection<T : OptData, F: Fitness> {
    fn select(&self, fitness: &Vec<F>, parents_indices: &mut Vec<usize>);
}

pub trait Crossover<T : OptData> {
    fn crossover(&self, population: &Vec<T>, parents_indices: &Vec<usize>, offsprings: &mut Vec<T>);
}

pub trait ReplacementStrategy<T : OptData, FIn: Fitness, FOpt: Fitness> {
    fn replace(&self, population: &mut Vec<T>, fitness_in: &mut Vec<FIn>, fitness_opt: &mut Vec<FOpt>, offsprings_from: usize);
}

pub trait Solution<T: OptData, FIn: Fitness, FOpt: Fitness> : Clone {
    fn from_population(population: &Vec<T>, fitness_in: &Vec<FIn>, fitness_opt: &Vec<FOpt>) -> Self;
    fn diff(&self, other: &Self) -> f64;
    fn is_better(&self, other: &Self) -> bool;
}

pub trait Statistics<T: OptData, FIn: Fitness, FOpt: Fitness> : Clone {
    fn new() -> Self;
    fn report_iter(&mut self, iter: usize, population: &Vec<T>, fitness_in: &Vec<FIn>, fitness_opt: &Vec<FOpt>);
}