use std::rc::Rc;

use crate::*;

pub fn gtsp_basic_stats() {
    let problem_names = ["a", "b", "c", "d", "e", "f"];
    for problem_name in problem_names {
        println!("{}", problem_name);
        let problem = load_gtsp_problem(format!("data/gtsp/{}.txt", problem_name).as_str());
        println!("vert count: {}", problem.vert_count);
        println!("group count: {}", problem.groups.len());
        println!("euclidean:  {}", are_distances_euclidean(&problem.distances));
        println!("metric:     {}", are_distances_a_metric(&problem.distances));
        let positions = gtsp_force_directed_positions(&problem);
        let colors = uniform_colors(problem.groups.len());
        plot_gtsp_points(&positions, &colors, 4, "out/points.svg", problem_name).unwrap();
        break;
    }
}

pub fn gtsp_basic_stats_gen_instance() {
    let problem = load_gtsp_problem("data/gtsp/gen1.txt");
    let positions = load_gtsp_positions("data/gtsp/gen1_pos.txt");
    println!("vert count: {}", problem.vert_count);
    println!("group count: {}", problem.groups.len());
    println!("euclidean:  {}", are_distances_euclidean(&problem.distances));
    println!("metric:     {}", are_distances_a_metric(&problem.distances));
    let colors = uniform_colors(problem.groups.len());
    let mut perm = GtspPermutation {
        spec: Rc::new(problem.clone()),
        perm: Vec::new()
    };
    for g in 0..problem.groups.len() {
        perm.perm.push(GroupVert { group: g, vert: 0 });
    }
    plot_gtsp_solution(&positions, &perm, &colors, 4, "out/points.svg", "gen points").unwrap();
}

pub fn gtsp_gen_problem() {
    let (problem, positions) = gen_euclidean_gtsp_problem(24, 5);
    println!("vert count: {}", problem.vert_count);
    println!("group count: {}", problem.groups.len());
    println!("euclidean:  {}", are_distances_euclidean(&problem.distances));
    println!("metric:     {}", are_distances_a_metric(&problem.distances));
    let colors = uniform_colors(problem.groups.len());
    plot_gtsp_points(&positions, &colors, 4, "out/points.svg", "gen points").unwrap();
    save_gtsp_problem("data/gtsp/gen1.txt", &problem);
    save_gtsp_positions("data/gtsp/gen1_pos.txt", &positions);
}

pub fn gtsp_basic_stats_default_params(num_repetitions: usize, num_iters: usize, population_size: usize) {
    let method_names = vec!["local move", "local swap", "local rev", "evo move cycle", "evo move order"];
    let input_files = vec!["gen1", "a", "b", "c", "d", "e", "f"];

    for input_file in input_files {
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = GenerationalReplacementStrategy {};
        let local_crossover = IdentityCrossover {};

        let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
        let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let evo_selection = RankSelection { select_count: population_size / 2 };
        let evo_replacement_strategy = TruncationReplacementStrategy {};

        let move_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspMoveGroupPerturbation {})}
        ]};
        let swap_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspSwapGroupPerturbation {})}
        ]};
        let rev_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let cycle_crossover = GtspCycleCrossover::new();
        let order_crossover = GtspOrderCrossover::new();
        

        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; method_names.len()];

        for _rep in 0..num_repetitions {

            // local searches
            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                move_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);

            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                swap_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);

            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                rev_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);
            
            // evolutionary searches

            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &cycle_crossover,
                move_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond);

            let (_, stats5) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &order_crossover,
                move_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond);

            let curr_stats = vec![stats1, stats2, stats3, stats4, stats5];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/{}_default.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness").unwrap();
        
    }
}

pub fn gtsp_viz_gen_solution(num_iters: usize, population_size: usize)
{
    let input_file = "gen1";

    let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
    let positions = load_gtsp_positions(format!("data/gtsp/{}_pos.txt", input_file).as_str());
    let mut fitness = GtspFitness {};

    let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
    let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
    let evo_selection = RankSelection { select_count: population_size / 2 };
    let evo_replacement_strategy = TruncationReplacementStrategy {};

    let move_perturbation = CombinePerturbeMutOps { mut_ops: vec![
        ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
        ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspMoveGroupPerturbation {})}
    ]};

    let order_crossover = GtspOrderCrossover::new();

    let (sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
         = evolutionary_search(
        &mut fitness, 
        evo_init_population.clone(),
        &evo_selection,
        &order_crossover,
        move_perturbation.clone(), 
        &evo_replacement_strategy,
        &evo_termination_cond);
    
    let colors = uniform_colors(problem.groups.len());
    plot_gtsp_solution(&positions, &sol.value, &colors, 4, "out/gtsp/gen1_viz.svg", "gen1").unwrap();
    
}

pub fn gtsp_find_opt_params_local_search(num_repetitions: usize, num_iters: usize) {
    let input_files = ["gen1", "a", "b", "c", "d", "e", "f"];
    let mut problems = Vec::<Rc<GtspProblem>>::with_capacity(input_files.len());
    for i in 0..input_files.len() {
        problems.push(Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_files[i]).as_str())))
    }
    let prob_samples = 10usize;
    let total_samples = prob_samples.pow(4);
    let mut best_problem_probs = vec![[0.0; 4]; problems.len()];
    let mut best_problem_fitness = vec![f64::INFINITY; problems.len()];

    start_progress_bar();
    for sample in 0..total_samples {
        progress_bar_text(&format!("progress: {}%", 100.0 * (sample as f64) / (total_samples as f64)));
        let mut probs = [0.0; 4];
        let mut sample_decomposed = sample;
        for p in 0..4 {
            probs[p] = ((sample_decomposed % prob_samples) as f64) / (prob_samples as f64);
            sample_decomposed /= prob_samples;
        }
        let probs = probs;

        for i_problem in 0..problems.len() {
            let problem = problems[i_problem].clone();
            let mut fitness = GtspFitness {};
    
            let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
            let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
            let local_selection = IdentitySelection {};
            let local_replacement_strategy = GenerationalReplacementStrategy {};
            let local_crossover = IdentityCrossover {};
    
            let perturbation = CombinePerturbeMutOps { mut_ops: vec![
                ProbPerturbeMutOp { prob: probs[0], op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
                ProbPerturbeMutOp { prob: probs[1], op: Rc::from(GtspMoveGroupPerturbation {})},
                ProbPerturbeMutOp { prob: probs[2], op: Rc::from(GtspSwapGroupPerturbation {})},
                ProbPerturbeMutOp { prob: probs[3], op: Rc::from(GtspReverseGroupPerturbation {})}
            ]};
            
            let mut avg_sol_fitness = 0.0;
    
            for _rep in 0..num_repetitions {
    
                // local searches
                let (sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                    = evolutionary_search(
                    &mut fitness, 
                    local_init_population.clone(),
                    &local_selection,
                    &local_crossover,
                    perturbation.clone(), 
                    &local_replacement_strategy,
                    &local_termination_cond);
    
                avg_sol_fitness += sol.fitness;
            }
            avg_sol_fitness /= num_repetitions as f64;
            if avg_sol_fitness < best_problem_fitness[i_problem] {
                best_problem_fitness[i_problem] = avg_sol_fitness;
                best_problem_probs[i_problem] = probs;
            }
        }
        progress_bar_clear();
    }
    end_progress_bar();

    let mut unique_probs = Vec::<[f64;4]>::new();
    for i in 0..best_problem_probs.len() {
        let mut is_unique = true;
        for j in 0..unique_probs.len() {
            if best_problem_probs[i] == unique_probs[j] {
                is_unique = false;
                break;
            }
        }
        if is_unique {
            unique_probs.push(best_problem_probs[i]);
        }
    }
    let mut best_probs = best_problem_probs[0];
    let mut best_count = 0;
    for j in 0..unique_probs.len() {
        let mut count = 0;
        for i in 0..best_problem_probs.len() {
            if best_problem_probs[i] == unique_probs[j] {
                count += 1;
            }
        }
        if count > best_count {
            best_count = count;
            best_probs = unique_probs[j];
        }
    }
    println!("{}, {}, {}, {}", best_probs[0], best_probs[1], best_probs[2], best_probs[3]);
}

pub fn gtsp_local_search_stats(num_repetitions: usize, num_iters: usize) {
    let method_names = vec!["local move", "local swap", "local rev", "local tweaked"];
    let input_files = vec!["gen1", "a", "b", "c", "d", "e", "f"];

    for input_file in input_files {
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = GenerationalReplacementStrategy {};
        let local_crossover = IdentityCrossover {};

        let move_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspMoveGroupPerturbation {})}
        ]};
        let swap_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspSwapGroupPerturbation {})}
        ]};
        let rev_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};
        let opt_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.4, op: Rc::from(GtspMoveGroupPerturbation {})},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspSwapGroupPerturbation {})},
            ProbPerturbeMutOp { prob: 0.6, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};
        

        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; method_names.len()];

        for _rep in 0..num_repetitions {
            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                move_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);

            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                swap_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);

            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                rev_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond);
            
            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
               &mut fitness, 
               local_init_population.clone(),
               &local_selection,
               &local_crossover,
               opt_perturbation.clone(), 
               &local_replacement_strategy,
               &local_termination_cond);

            let curr_stats = vec![stats1, stats2, stats3, stats4];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/{}_local.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness").unwrap();
        
    }
}

// TODO:
// u nekterych grafu vypnout zobrazovani optima
// hledani optimalnich parametru
// pravdepodobnosti u local searche u perturbacnich operatoru
// - 1 varianta: 4 pravdepodobnosti dohromady a projet najednou
// - udelat grafy porovnavajici default local search a tenhle
// pravdepodobnosti u evolucnich algoritmu - perturbacni operatory a crossover
// - uvidim podle toho jak dopadne u local searche
// - nejspis udelat pro obecny perturbacni operator a obecny crossover operator, celkem tedy 7 pravděpodobností
// - udelat grafy porovnavajici default evo search a tenhle
// grafy porovnani pro zakladni nastaveni parametru - mam
// - vyresit ruzne iterace u local searche a evolucniho alg.
// grafy porovnani pro optimalni nalezene parametry
// - porovnani optimalinho local searche, optimalniho evolucniho algoritmu, specializovaneho algoritmu nastaveneho jako evolucni alg
// - specializovany udelat pomoci heuristiky, potom predano jako init do evolucniho algoritmu
//   - vybere nahodny vrchol a z něj začně skládat cestu, vždy vybere hranu s nejnižší vzdáleností co nevede zpět
// vizualizace heuristickeho reseni pouziteho v specializovanem algoritmu, staci jeden priklad pro kazdy dataset (idealne, ale 1 dataset by taky stacil)
// vizualizace behu jednoho vybraneho algoritmu - ukazat ten nejlepsi, idealne na slozitejsim problemu (pokud pujde snadno zobrazit)

