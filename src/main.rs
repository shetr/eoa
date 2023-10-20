use eoa::*;
use rand_distr::{Normal, Distribution};

fn test_bin() -> Result<(), Box<dyn std::error::Error>>
{
    let zeros = [0u8; 8];
    let ones = [1u8; 8];
    let v1 = [1, 1, 0, 0, 1, 1, 0, 0, 1, 1];
    let v2 = perturbe(&v1, 1.0 / (v1.len() as f64));
    let test_bounds = [Bounds { lower: 0.0, upper: 1.0 }, Bounds { lower: 0.0, upper: 31.0 }];
    let v3 = bin_to_real(&v1, &test_bounds);
    
    //println!("Original:  {:?}", v1);
    //println!("Perturbed: {:?}", v2);
    //println!("Real: {:?}", v3);

    let bounds = [Bounds { lower: 0.0, upper: 1.0 }; 2];
    let mut fitness = NaiveBitRealFunc::new( 
        SphereFunc { o: vec![0.0; 2] },
        Vec::from(bounds)
    );
    //let fitness = RosenbrockFunc {};
    let perturbe_mut_op = BasicNaiveBitPerturbeMutOp {};
    let termination_cond = MaxIterTerminationCond { n_iters: 30 };
    //let bounds = [Bounds { lower: -10.0, upper: 10.0 }; 2];
    let init_value = ones;
    let init_func = InitValue { value: NaiveBitVec{ bits: Vec::from(init_value) } };
    let (solution, stats) =
        local_search(&mut fitness, perturbe_mut_op, &termination_cond, init_func);
    println!("Solution:  {:?}", solution.value.bits);
    println!("Fitness:  {:?}", solution.fitness);
    plot(&stats, "out.svg", "Sphere")
}

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
        local_search(&mut fitness, perturbe_mut_op, &termination_cond, init_func);
    println!("Solution:  {:?}", solution.value.values);
    println!("Fitness:  {:?}", solution.fitness);
    plot(&stats, "out.svg", "Sphere")
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    test_real()
}
