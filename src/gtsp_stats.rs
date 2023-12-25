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
        

        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; 5];

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
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/{}.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness").unwrap();
        
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

// TODO:
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

