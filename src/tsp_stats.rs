use crate::*;

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
