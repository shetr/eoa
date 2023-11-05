use plotters::style::*;

use crate::*;

pub fn create_comparison_graphs()
{
    let input_files = ["att48", "berlin52", "eil76"];
    let method_names = vec!["loc_move", "loc_swap", "loc_rev", "evo_cycle", "evo_order"];
    let colors = vec![RED, GREEN, BLUE, YELLOW, MAGENTA];
    const NUM_REPETITIONS: usize = 7;
    const NUM_ITERS: usize = 300;
    const POPULATION_SIZE: usize = 50;
    for input_file in input_files {
        let vert_positions = load_vert_positions(format!("data/{}.tsp", input_file).as_str());
        let opt_vert_permutation = load_opt_permutation(format!("data/{}.opt.tour", input_file).as_str());
        let vert_distances = vert_positions_to_distances(&vert_positions);
        let mut fitness = TspFitness { distances: vert_distances };
        let opt_value = fitness.eval(&opt_vert_permutation);
        let vert_count = vert_positions.len();
        
        let termination_cond = MaxIterTerminationCond { n_iters: NUM_ITERS };
        let selection = TournamentSelection { select_count: vert_count / 2, rounds_count: 8 };
        let replacement_strategy = TruncationReplacementStrategy {};
        
        let move_perturbation = TspMovePerturbation {};
        let swap_perturbation = TspSwapPerturbation {};
        let rev_perturbation = TspReversePerturbation {};

        let cycle_crossover = TspCycleCrossover {};
        let order_crossover = TspOrderCrossover {};

        let mut avg_stats = vec![
            Statistics { fitness: vec![0.0f64; NUM_ITERS]},
            Statistics { fitness: vec![0.0f64; NUM_ITERS]},
            Statistics { fitness: vec![0.0f64; NUM_ITERS]},
            Statistics { fitness: vec![0.0f64; NUM_ITERS]},
            Statistics { fitness: vec![0.0f64; NUM_ITERS]}];

        for rep in 0..NUM_REPETITIONS {
            let init_population = InitTspPopulation { size: POPULATION_SIZE, vert_count: vert_count };

            let (_, stats1) =
                local_search(&mut fitness, init_population.clone(), move_perturbation.clone(), &termination_cond);

            let (_, stats2) =
                local_search(&mut fitness, init_population.clone(), swap_perturbation.clone(), &termination_cond);

            let (_, stats3) =
                local_search(&mut fitness, init_population.clone(), rev_perturbation.clone(), &termination_cond);
            
            let (_, stats4) = evolutionary_search(
                &mut fitness, 
                init_population.clone(),
                &selection,
                &cycle_crossover,
                move_perturbation.clone(), 
                &replacement_strategy,
                &termination_cond);

            let (_, stats5) = evolutionary_search(
                &mut fitness, 
                init_population.clone(),
                &selection,
                &order_crossover,
                move_perturbation.clone(), 
                &replacement_strategy,
                &termination_cond);

            let curr_stats = vec![stats1, stats2, stats3, stats4, stats5];
            for s in 0..avg_stats.len() {
                for i in 0..NUM_ITERS {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
        }

        for s in 0..avg_stats.len() {
            for i in 0..NUM_ITERS {
                avg_stats[s].fitness[i] /= NUM_REPETITIONS as f64;
                // maybe log scale
                avg_stats[s].fitness[i] = avg_stats[s].fitness[i].log10();
            }
            //plot(&avg_stats[s], format!("out/tsp/{}_{}.svg", method_names[s], input_file).as_str(), method_names[s]).unwrap();
        }
        plot_multiple(&avg_stats, &method_names, &colors, format!("out/tsp/{}.svg", input_file).as_str(), input_file, opt_value).unwrap();
        
    }
}
