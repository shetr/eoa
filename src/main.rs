use eoa::*;

fn main() {
    let zeros: [u8; 8] = [0; 8];
    let ones: [u8; 8] = [1; 8];
    let v1 = [1, 1, 0, 0, 1, 1, 0, 0, 1, 1];
    let v2 = perturbe(&v1, 1.0 / (v1.len() as f64));
    let test_bounds = [Bounds { lower: 0.0, upper: 1.0 }, Bounds { lower: 0.0, upper: 31.0 }];
    let v3 = bin_to_real(&v1, &test_bounds);
    
    println!("Original:  {:?}", v1);
    println!("Perturbed: {:?}", v2);
    println!("Real: {:?}", v3);

    let fitness = OneMaxFunc{};
    let perturbe_mut_op = BasicNaiveBitPerturbeMutOp {};
    let termination_cond = MaxIterNaiveBitTerminationCond { n_iters: 100 };
    let bounds = [Bounds { lower: 0.0, upper: 1.0 }, Bounds { lower: 0.0, upper: 1.0 }];
    let init_value = zeros;
    let (solution, stats) =
        naive_bit_local_search(&fitness, &perturbe_mut_op, &termination_cond, &bounds, &init_value);
    println!("Solution:  {:?}", solution.value);
    println!("Fitness:  {:?}", solution.fitness);
}
