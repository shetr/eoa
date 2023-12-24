use std::rc::Rc;
use std::fs::create_dir_all;

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
    create_dir_all("out/gtsp").unwrap();
    let method_names = vec!["evo move cycle", "evo move order"];
    let input_files = vec!["gen1", "a", "b", "c", "d", "e", "f"];

    for input_file in input_files {
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
        
        let termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let selection = RankSelection { select_count: population_size / 2 };
        let replacement_strategy = TruncationReplacementStrategy {};

        let move_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspMoveGroupPerturbation {})}
        ]};
        let _swap_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspSwapGroupPerturbation {})}
        ]};
        let _rev_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.5, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let cycle_crossover = GtspCycleCrossover::new();
        let order_crossover = GtspOrderCrossover::new();
        

        let mut avg_stats = vec![
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}
        ];

        for _rep in 0..num_repetitions {

            let (_sol, stats1) = evolutionary_search(
                &mut fitness, 
                init_population.clone(),
                &selection,
                &cycle_crossover,
                move_perturbation.clone(), 
                &replacement_strategy,
                &termination_cond);

            //println!("{} {}", input_file, _sol.fitness.to_string());

            let (_, stats2) = evolutionary_search(
                &mut fitness, 
                init_population.clone(),
                &selection,
                &order_crossover,
                move_perturbation.clone(), 
                &replacement_strategy,
                &termination_cond);

            let curr_stats = vec![stats1, stats2];
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
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/{}.svg", input_file).as_str(), input_file, opt_value.log10(), "Log avg. fitness").unwrap();
        
    }
}
