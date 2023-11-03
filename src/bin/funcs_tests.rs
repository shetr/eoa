use eoa::*;
use rand_distr::{Normal, Distribution};

fn test_real() -> Result<(), Box<dyn std::error::Error>>
{
    let data = [10.0; 2];
    let mut fitness = SphereFunc { o: vec![0.0; 2] };
    let perturbe_mut_op = NormalOneFiftPerturbeRealMutOp::new(1.0);
    let termination_cond = MaxIterTerminationCond { n_iters: 100 };
    let bounds = [Bounds { lower: f64::NEG_INFINITY, upper: f64::INFINITY }; 2];
    let init_value = data;
    let init_func = InitValue { value: FloatVec{ values: Vec::from(init_value) } };
    let (solution, stats) =
        local_search(&mut fitness, init_func, perturbe_mut_op, &termination_cond);
    println!("Solution:  {:?}", solution.value.values);
    println!("Fitness:  {:?}", solution.fitness);
    plot(&stats, "out/out.svg", "Sphere")
}


fn main() -> Result<(), Box<dyn std::error::Error>>
{
    test_real()
}
