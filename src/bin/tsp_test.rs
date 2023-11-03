use eoa::*;
use rand_distr::{Normal, Distribution};

fn test_tsp() -> Result<(), Box<dyn std::error::Error>>
{
    const VEC_SIZE: usize = 2;
    let mut fitness = SphereFunc { o: vec![0.0; VEC_SIZE] };
    let perturbe_mut_op = NormalOneFiftPerturbeRealMutOp::new(1.0);
    let termination_cond = MaxIterTerminationCond { n_iters: 100 };
    let init_population = InitRandomFloatVecPopulation { size: 10, vec_size: VEC_SIZE, mean: 0.0, std_dev: 10.0 };
    let selection = TournamentSelection { select_count: VEC_SIZE / 2, rounds_count: 4 };
    let crossover = OnePointCrossover {};
    let replacement_strategy = TruncationReplacementStrategy {};

    let (solution, stats) = evolutionary_search(
        &mut fitness, 
        init_population,
        &selection,
        &crossover,
        perturbe_mut_op, 
        &replacement_strategy,
        &termination_cond);
    println!("Solution:  {:?}", solution.value.values);
    println!("Fitness:  {:?}", solution.fitness);
    plot(&stats, "out.svg", "Sphere")
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    test_tsp()
}
