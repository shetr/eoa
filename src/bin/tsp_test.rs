use eoa::*;

fn test_tsp()
{
    const VERT_COUNT: usize = 5;
    let mut fitness = TspFitness { distances: DistanceHalfMatrix::from(VERT_COUNT, vec![
        0.0f64,
        1.0, 0.0,
        3.0, 0.5, 0.0,
        4.0, 1.0, 2.0, 0.0,
        5.0, 2.0, 1.5, 7.0, 0.0
    ]) };
    let perturbe_mut_op = TspMovePerturbation {};
    let termination_cond = MaxIterTerminationCond { n_iters: 100 };
    let init_population = InitTspPopulation { size: 10, vert_count: VERT_COUNT };
    let selection = TournamentSelection { select_count: VERT_COUNT / 2, rounds_count: 4 };
    let crossover = TspCycleCrossover {};
    let replacement_strategy = TruncationReplacementStrategy {};

    let (solution, stats) = evolutionary_search(
        &mut fitness, 
        init_population,
        &selection,
        &crossover,
        perturbe_mut_op, 
        &replacement_strategy,
        &termination_cond);
    println!("Solution:  {:?}", solution.value.vert_perm);
    println!("Fitness:  {:?}", solution.fitness);
    plot(&stats, "out/tsp_test.svg", "TSP").unwrap();
}

fn test_tsp_file()
{
    let vert_positions = load_vert_positions("data/att48.tsp");
    let opt_vert_permutation = load_opt_permutation("data/att48.opt.tour");
    let vert_distances = vert_positions_to_distances(&vert_positions);
    let vert_count = vert_positions.len();
    let mut fitness = TspFitness { distances: vert_distances };
    let opt_value = fitness.eval(&opt_vert_permutation);
    let perturbe_mut_op = TspMovePerturbation {};
    let termination_cond = MaxIterTerminationCond { n_iters: 100 };
    let init_population = InitTspPopulation { size: 10, vert_count: vert_count };
    let selection = TournamentSelection { select_count: vert_count / 2, rounds_count: 4 };
    let crossover = TspCycleCrossover {};
    let replacement_strategy = TruncationReplacementStrategy {};

    let (solution, stats) = evolutionary_search(
        &mut fitness, 
        init_population,
        &selection,
        &crossover,
        perturbe_mut_op, 
        &replacement_strategy,
        &termination_cond);
    println!("Solution:  {:?}", solution.value.vert_perm);
    println!("Fitness:  {:?}", solution.fitness);
    println!("Optimal fitness:  {:?}", opt_value);
    plot(&stats, "out/tsp_file_test.svg", "TSP").unwrap();
}

fn main() {
    test_tsp_file();
}
