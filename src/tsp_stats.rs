use crate::*;

pub fn create_comparison_graphs(num_repetitions: usize, num_iters: usize, population_size: usize)
{
    let input_files = ["att48", "berlin52", "eil76"];
    let method_names = vec!["loc_move", "loc_swap", "loc_rev", "evo_cycle", "evo_order"];
    for input_file in input_files {
        let vert_positions = load_vert_positions(format!("data/{}.tsp", input_file).as_str());
        let opt_vert_permutation = load_opt_permutation(format!("data/{}.opt.tour", input_file).as_str());
        let vert_distances = vert_positions_to_distances(&vert_positions);
        let mut fitness = TspFitness { distances: vert_distances };
        let opt_value = fitness.eval(&opt_vert_permutation);
        let vert_count = vert_positions.len();
        
        let termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let selection = TournamentSelection { select_count: vert_count / 2, rounds_count: 8 };
        let replacement_strategy = TruncationReplacementStrategy {};
        
        let move_perturbation = TspMovePerturbation {};
        let swap_perturbation = TspSwapPerturbation {};
        let rev_perturbation = TspReversePerturbation {};

        let cycle_crossover = TspCycleCrossover {};
        let order_crossover = TspOrderCrossover {};
        
        let init_population = InitTspPopulation { size: population_size, vert_count: vert_count };

        let mut avg_stats = vec![
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]}];

        //let mut search_funs = [
        //    EvolutionarySearchFunCall {
        //        fitness_func: &mut fitness,
        //        init_population: &init_population,
        //        selection: &selection,
        //        crossover: &cycle_crossover,
        //        perturbe_mut_op: &move_perturbation,
        //        replacement_strategy: &replacement_strategy,
        //        termination_cond: &termination_cond,
        //        search_fun: local_search_evolutionary_api
        //    },
        //    EvolutionarySearchFunCall {
        //        fitness_func: &mut fitness,
        //        init_population: &init_population,
        //        selection: &selection,
        //        crossover: &cycle_crossover,
        //        perturbe_mut_op: &swap_perturbation,
        //        replacement_strategy: &replacement_strategy,
        //        termination_cond: &termination_cond,
        //        search_fun: local_search_evolutionary_api
        //    },
        //    EvolutionarySearchFunCall {
        //        fitness_func: &mut fitness,
        //        init_population: &init_population,
        //        selection: &selection,
        //        crossover: &cycle_crossover,
        //        perturbe_mut_op: &move_perturbation,
        //        replacement_strategy: &replacement_strategy,
        //        termination_cond: &termination_cond,
        //        search_fun: local_search_evolutionary_api
        //    }
        //];
        //let stats1: Statistics = search_funs[0].search();
        for _rep in 0..num_repetitions {

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
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
        }

        for s in 0..avg_stats.len() {
            for i in 0..num_iters {
                avg_stats[s].fitness[i] /= num_repetitions as f64;
                // maybe log scale
                avg_stats[s].fitness[i] = avg_stats[s].fitness[i].log10();
            }
            //plot(&avg_stats[s], format!("out/tsp/{}_{}.svg", method_names[s], input_file).as_str(), method_names[s]).unwrap();
        }
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/tsp/{}.svg", input_file).as_str(), input_file, opt_value.log10()).unwrap();
        
    }
}

pub fn create_vizualization_graphs(num_iters: usize, population_size: usize)
{
    let input_file  = "berlin52";
    let vert_positions = load_vert_positions(format!("data/{}.tsp", input_file).as_str());
    let opt_vert_permutation = load_opt_permutation(format!("data/{}.opt.tour", input_file).as_str());
    let vert_distances = vert_positions_to_distances(&vert_positions);
    let mut fitness = TspFitness { distances: vert_distances };
    let _opt_value = fitness.eval(&opt_vert_permutation);
    let vert_count = vert_positions.len();
    
    let selection = TournamentSelection { select_count: vert_count / 2, rounds_count: 8 };
    let replacement_strategy = TruncationReplacementStrategy {};
    let perturbation = TspMovePerturbation {};
    let crossover = TspCycleCrossover {};
    
    let init_population = InitTspPopulation { size: population_size, vert_count: vert_count };

    
    let termination_cond = MaxIterTerminationCond { n_iters: num_iters };

    let (solution, _stats) = evolutionary_search(
        &mut fitness, 
        init_population.clone(),
        &selection,
        &crossover,
        perturbation.clone(), 
        &replacement_strategy,
        &termination_cond);

    plot_tsp_viz(&vert_positions, &opt_vert_permutation, "out/tsp/opt_viz.svg", "berlin52 optimum").unwrap();
    plot_tsp_viz(&vert_positions, &solution.value, format!("out/tsp/iter{}_viz.svg", num_iters).as_str(), format!("berlin52 iter{}", num_iters).as_str()).unwrap();
}
