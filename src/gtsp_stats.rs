use std::{rc::Rc, fs::File, io::Write};

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
    let problem = load_gtsp_problem("data/gtsp/g1.txt");
    let positions = load_gtsp_positions("data/gtsp/g1_pos.txt");
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
    plot_gtsp_solution(&positions, &perm, 0.0, &colors, 4, "out/points.svg", "gen points").unwrap();
}

pub fn gtsp_gen_problem(vert_count: usize, group_count: usize, file_name: &str) {
    let (problem, positions) = gen_euclidean_gtsp_problem(vert_count, group_count);
    println!("vert count: {}", problem.vert_count);
    println!("group count: {}", problem.groups.len());
    println!("euclidean:  {}", are_distances_euclidean(&problem.distances));
    println!("metric:     {}", are_distances_a_metric(&problem.distances));
    let colors = uniform_colors(problem.groups.len());
    plot_gtsp_points(&positions, &colors, 4, "out/points.svg", "gen points").unwrap();
    save_gtsp_problem(&format!("data/gtsp/{}.txt", file_name), &problem);
    save_gtsp_positions(&format!("data/gtsp/{}_pos.txt", file_name), &positions);
}

pub fn gtsp_basic_stats_default_params(num_repetitions: usize, population_size: usize) {
    let method_names = vec!["local move", "local swap", "local rev", "evo move cycle", "evo move order"];
    let is_method_local = [true, true, true, false, false];
    let input_files = ["g1", "g2", "g3", "a", "b", "c", "d", "e", "f"];
    let max_iters = [20, 300, 2500, 40, 90, 350, 600, 1000, 3000];
    
    let mut iter = 0;
    let total_iters = input_files.len() * num_repetitions;
    start_progress_bar();
    for f in 0..input_files.len() {
        let num_iters = max_iters[f];
        let input_file = input_files[f];
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters * population_size };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = TruncationReplacementStrategy {};
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
            progress_bar_text(&format!("progress: {:.2}%", 100.0 * (iter as f64) / (total_iters as f64)));

            // local searches
            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                move_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);

            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                swap_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);

            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                rev_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);
            
            // evolutionary searches

            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &cycle_crossover,
                rev_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);

            let (_, stats5) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &order_crossover,
                rev_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);

            let curr_stats = vec![stats1, stats2, stats3, stats4, stats5];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    let step = if is_method_local[s] { population_size } else { 1 };
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i * step];
                }
            }
            iter += 1;
            progress_bar_clear();
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/default_{}.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness", false, true).unwrap();
    }
    end_progress_bar();
}

pub fn gtsp_viz_gen_solution(num_iters: usize, population_size: usize)
{
    let input_file = "g3";
    let point_size = 4;

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
        &evo_termination_cond,
        false);
    
    let colors = uniform_colors(problem.groups.len());
    plot_gtsp_solution(&positions, &sol.value, sol.fitness, &colors, point_size, &format!("out/gtsp/viz_{}.svg", input_file), input_file).unwrap();
    
}

pub fn gtsp_find_opt_params_local_search(num_repetitions: usize, num_iters: usize, prob_samples: usize) {
    let input_files = ["g1", "a", "b", "c", "d", "e", "f"];
    let mut problems = Vec::<Rc<GtspProblem>>::with_capacity(input_files.len());
    for i in 0..input_files.len() {
        problems.push(Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_files[i]).as_str())))
    }
    let total_samples = prob_samples.pow(4);
    let mut best_probs = [0.0; 4];
    let mut best_fitness_sum = f64::INFINITY;

    start_progress_bar();
    for sample in 0..total_samples {
        progress_bar_text(&format!("progress: {:.2}%", 100.0 * (sample as f64) / (total_samples as f64)));
        let mut probs = [0.0; 4];
        let mut sample_decomposed = sample;
        for p in 0..4 {
            probs[p] = ((sample_decomposed % prob_samples) as f64) / (prob_samples as f64);
            sample_decomposed /= prob_samples;
        }
        let probs = probs;

        let mut fitness_sum = 0.0;
        for i_problem in 0..problems.len() {
            let problem = problems[i_problem].clone();
            let mut fitness = GtspFitness {};
    
            let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
            let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
            let local_selection = IdentitySelection {};
            let local_replacement_strategy = TruncationReplacementStrategy {};
            let local_crossover = IdentityCrossover {};
    
            let perturbation = CombinePerturbeMutOps { mut_ops: vec![
                ProbPerturbeMutOp { prob: probs[0], op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
                ProbPerturbeMutOp { prob: probs[1], op: Rc::from(GtspMoveGroupPerturbation {})},
                ProbPerturbeMutOp { prob: probs[2], op: Rc::from(GtspSwapGroupPerturbation {})},
                ProbPerturbeMutOp { prob: probs[3], op: Rc::from(GtspReverseGroupPerturbation {})}
            ]};
            
            let mut avg_sol_fitness = 0.0;
    
            for _rep in 0..num_repetitions {
                let (sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                    = evolutionary_search(
                    &mut fitness, 
                    local_init_population.clone(),
                    &local_selection,
                    &local_crossover,
                    perturbation.clone(), 
                    &local_replacement_strategy,
                    &local_termination_cond,
                    false);
    
                avg_sol_fitness += sol.fitness;
            }
            avg_sol_fitness /= num_repetitions as f64;
            fitness_sum += avg_sol_fitness;
        }
        if fitness_sum < best_fitness_sum {
            best_fitness_sum = fitness_sum;
            best_probs = probs;
        }
        progress_bar_clear();
    }
    end_progress_bar();
    println!("{}, {}, {}, {}", best_probs[0], best_probs[1], best_probs[2], best_probs[3]);

    let mut file = File::create("data/gtsp/probs_local.txt").expect("unable to create a file.");
    file.write(format!("{}, {}, {}, {}\n", best_probs[0], best_probs[1], best_probs[2], best_probs[3]).as_bytes()).unwrap();
}

pub fn gtsp_local_search_stats(num_repetitions: usize) {
    let method_names = vec!["local move", "local swap", "local rev", "local tweaked"];
    let input_files = ["g1", "g2", "g3", "a", "b", "c", "d", "e", "f"];
    let max_iters = [600, 3000, 6000, 1500, 2000, 3000, 4000, 5000, 6000];

    let mut iter = 0;
    let total_iters = input_files.len() * num_repetitions;
    start_progress_bar();
    for f in 0..input_files.len() {
        let num_iters = max_iters[f];
        let input_file = input_files[f];
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = TruncationReplacementStrategy {};
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
            ProbPerturbeMutOp { prob: 0.0, op: Rc::from(GtspMoveGroupPerturbation {})},
            ProbPerturbeMutOp { prob: 0.0, op: Rc::from(GtspSwapGroupPerturbation {})},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; method_names.len()];

        for _rep in 0..num_repetitions {
            progress_bar_text(&format!("progress: {:.2}%", 100.0 * (iter as f64) / (total_iters as f64)));
            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                move_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);

            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                swap_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);

            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                rev_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);
            
            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
               &mut fitness, 
               local_init_population.clone(),
               &local_selection,
               &local_crossover,
               opt_perturbation.clone(), 
               &local_replacement_strategy,
               &local_termination_cond,
               false);

            let curr_stats = vec![stats1, stats2, stats3, stats4];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
            progress_bar_clear();
            iter += 1;
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/local_{}.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness", false, true).unwrap();
        
    }
    end_progress_bar();
}

pub fn gtsp_find_opt_params_evolutionary_search(num_repetitions: usize, _num_iters: usize, population_size: usize, prob_samples: usize) {
    let input_files = ["g1", "a", "b", "c", "d", "e", "f"];
    let max_iters = [100, 300, 500, 500, 1000, 1500, 3000];
    let mut problems = Vec::<Rc<GtspProblem>>::with_capacity(input_files.len());
    for i in 0..input_files.len() {
        problems.push(Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_files[i]).as_str())))
    }
    const PROB_COUNT: usize = 4;
    let total_samples = prob_samples.pow(PROB_COUNT as u32);
    let mut best_probs = [0.0; PROB_COUNT];
    let mut best_fitness_sum = f64::INFINITY;

    start_progress_bar();
    for sample in 0..total_samples {
        progress_bar_text(&format!("progress: {:.3}%", 100.0 * (sample as f64) / (total_samples as f64)));
        let mut probs = [0.0; PROB_COUNT];
        let mut sample_decomposed = sample;
        let mut probs2to3_sum = 0.0;
        for p in 0..PROB_COUNT {
            probs[p] = ((sample_decomposed % prob_samples) as f64) / (prob_samples as f64);
            if p > 1 {
                probs2to3_sum += probs[p];
            }
            sample_decomposed /= prob_samples;
        }
        // could iterated more cleverly so that we don't have to skip iterations
        if probs2to3_sum > 1.0 {
            progress_bar_clear();
            continue;
        }
        let probs = probs;

        let mut fitness_sum = 0.0;
        for i_problem in 0..problems.len() {
            let num_iters = max_iters[i_problem];
            let problem = problems[i_problem].clone();
            let mut fitness = GtspFitness {};
    
            let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
            let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
            let evo_selection = RankSelection { select_count: population_size / 2 };
            let evo_replacement_strategy = TruncationReplacementStrategy {};
    
            let perturbation = CombinePerturbeMutOps { mut_ops: vec![
                ProbPerturbeMutOp { prob: probs[0], op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
                ProbPerturbeMutOp { prob: probs[1], op: Rc::from(GtspReverseGroupPerturbation {})}
            ]};

            let crossover = GtspGeneralCrossover {
                city_prob: probs[2],
                cycle_prob: 0.0,
                order_prob: probs[3]
            };
            
            let mut avg_sol_fitness = 0.0;
    
            for _rep in 0..num_repetitions {
                let (sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                    = evolutionary_search(
                    &mut fitness, 
                    evo_init_population.clone(),
                    &evo_selection,
                    &crossover,
                    perturbation.clone(), 
                    &evo_replacement_strategy,
                    &evo_termination_cond,
                    false);
    
                avg_sol_fitness += sol.fitness;
            }
            avg_sol_fitness /= num_repetitions as f64;
            fitness_sum += avg_sol_fitness;
        }
        if fitness_sum < best_fitness_sum {
            best_fitness_sum = fitness_sum;
            best_probs = probs;
        }
        progress_bar_clear();
    }
    end_progress_bar();
    println!("{}, {}, {}, {}", best_probs[0], best_probs[1], best_probs[2], best_probs[3]);

    let mut file = File::create("data/gtsp/probs_evo.txt").expect("unable to create a file.");
    file.write(format!("{}, {}, {}, {}\n", best_probs[0], best_probs[1], best_probs[2], best_probs[3]).as_bytes()).unwrap();
}

struct EvoSearchParamsFitness {
    num_repetitions: usize,
    population_size: usize,
    problems: Vec<Rc<GtspProblem>>
}

impl EvoSearchParamsFitness {
    pub fn new(num_repetitions: usize, population_size: usize) -> Self {
        let input_files = ["g1", "a", "b", "c", "d", "e", "f"];
        let mut problems = Vec::<Rc<GtspProblem>>::with_capacity(input_files.len());
        for i in 0..input_files.len() {
            problems.push(Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_files[i]).as_str())))
        }
        EvoSearchParamsFitness { num_repetitions, population_size, problems }
    }
}

impl FitnessFunc<FloatVec> for EvoSearchParamsFitness {
    fn eval(&self, probs: &FloatVec) -> f64 {
        let max_iters = [100, 300, 500, 500, 1000, 1500, 3000];
        let mut fitness_sum = 0.0;
        for i_problem in 0..self.problems.len() {
            let num_iters = max_iters[i_problem];
            let problem = self.problems[i_problem].clone();
            let mut fitness = GtspFitness {};
    
            let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: self.population_size };
            let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
            let evo_selection = RankSelection { select_count: self.population_size / 2 };
            let evo_replacement_strategy = TruncationReplacementStrategy {};
    
            let perturbation = CombinePerturbeMutOps { mut_ops: vec![
                ProbPerturbeMutOp { prob: probs.values[0], op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
                ProbPerturbeMutOp { prob: probs.values[1], op: Rc::from(GtspReverseGroupPerturbation {})}
            ]};

            let crossover = GtspGeneralCrossover {
                city_prob: probs.values[2],
                cycle_prob: probs.values[3],
                order_prob: 0.0
            };
            
            let mut avg_sol_fitness = 0.0;
    
            for _rep in 0..self.num_repetitions {
                let (sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                    = evolutionary_search(
                    &mut fitness, 
                    evo_init_population.clone(),
                    &evo_selection,
                    &crossover,
                    perturbation.clone(), 
                    &evo_replacement_strategy,
                    &evo_termination_cond,
                    false);
    
                avg_sol_fitness += sol.fitness;
            }
            avg_sol_fitness /= self.num_repetitions as f64;
            fitness_sum += avg_sol_fitness;
        }
        fitness_sum
    }
}

#[derive(Clone)]
pub struct EvoOptStatistics {
    pub best_fitness: f64,
    pub best_probs: FloatVec
}

impl Statistics<FloatVec, f64, f64> for EvoOptStatistics {
    fn new() -> Self {
        EvoOptStatistics { best_fitness: f64::INFINITY, best_probs: FloatVec { values: vec![0.0; 4] } }
    }

    fn report_iter(&mut self, iter: usize, population: &Vec<FloatVec>, _fitness_in: &Vec<f64>, fitness_opt: &Vec<f64>) {
        let best_index = find_best_fitness(fitness_opt);
        let curr_fitness = fitness_opt[best_index];
        if curr_fitness < self.best_fitness {
            self.best_fitness = curr_fitness;
            self.best_probs = population[best_index].clone();
            let probs = &self.best_probs.values;
            let mut file = File::create("data/gtsp/probs_evo.txt").expect("unable to create a file.");
            file.write(format!("{}, {}, {}, {}\n", probs[0], probs[1], probs[2], probs[3]).as_bytes()).unwrap();
        }
        let probs = &self.best_probs.values;
        let currp = &population[best_index].values;
        println!("iter: {}", iter);
        println!("  best f: {}, p: {}, {}, {}, {}", self.best_fitness, probs[0], probs[1], probs[2], probs[3]);
        println!("  curr f: {}, p: {}, {}, {}, {}",      curr_fitness, currp[0], currp[1], currp[2], currp[3]);
    }
}

#[derive(Clone)]
struct EvoProbsPerturbeMutOp {}

impl PerturbeMutOp<FloatVec> for EvoProbsPerturbeMutOp {
    fn eval(&self, data: &mut FloatVec) {
        data.values[0] = rand::random::<f64>();
        data.values[1] = rand::random::<f64>();
        data.values[2] = rand::random::<f64>();
        data.values[3] = rand::random::<f64>() * (1.0 - data.values[2]);
    }
}

pub fn gtsp_find_opt_params_evolutionary_search_with_local_search(num_repetitions: usize, population_size: usize) {
    let total_samples = 20000;
    let mut fitness = EvoSearchParamsFitness::new(num_repetitions, population_size);

    let init_population = InitPopulationFromValues { population: vec![FloatVec {
        values: vec![0.9096472983082879, 0.4195441504092986, 0.2818948804986545, 0.37922819461965035]
    }]};
    let termination_cond = MaxIterTerminationCond { n_iters: total_samples };
    let selection = IdentitySelection {};
    let replacement_strategy = TruncationReplacementStrategy {};
    let crossover = IdentityCrossover {};

    //let perturbation = BoundedNormalPerturbeRealMutOp::new(0.05,
    //    &vec![Bounds { lower: 0.0, upper: 1.0}; 4]
    //);
    let perturbation = EvoProbsPerturbeMutOp {};

    let (_, _) : (BSFSingleObjSolution<FloatVec>, EvoOptStatistics)
        = evolutionary_search(
        &mut fitness, 
        init_population.clone(),
        &selection,
        &crossover,
        perturbation.clone(), 
        &replacement_strategy,
        &termination_cond,
        true);
}

pub fn gtsp_evolutionary_search_stats(num_repetitions: usize, population_size: usize) {
    let method_names = vec!["evo move cycle", "evo move order", "evo swap cycle", "evo swap order", "evo rev cycle", "evo rev order"];
    let input_files = ["g1", "g2", "g3", "a", "b", "c", "d", "e", "f"];
    let max_iters = [40, 400, 3000, 30, 1000, 1000, 1500, 2500, 4000];
    
    let mut iter = 0;
    let total_iters = input_files.len() * num_repetitions;
    start_progress_bar();
    for f in 0..input_files.len() {
        let num_iters = max_iters[f];
        let input_file = input_files[f];
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
        let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let evo_selection = RankSelection { select_count: population_size / 2 };
        let evo_replacement_strategy = TruncationReplacementStrategy {};

        let move_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspMoveGroupPerturbation {})}
        ]};
        let swap_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspSwapGroupPerturbation {})}
        ]};
        let rev_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let cycle_crossover = GtspCycleCrossover::new();
        let order_crossover = GtspOrderCrossover::new();
        
        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; method_names.len()];

        for _rep in 0..num_repetitions {
            progress_bar_text(&format!("progress: {:.2}%", 100.0 * (iter as f64) / (total_iters as f64)));

            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &cycle_crossover,
                move_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);
            
            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &order_crossover,
                move_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);
            
            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &cycle_crossover,
                swap_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);

            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &order_crossover,
                swap_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);
            
            let (_, stats5) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &cycle_crossover,
                rev_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);

            let (_, stats6) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &order_crossover,
                rev_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);

            let curr_stats = vec![stats1, stats2, stats3, stats4, stats5, stats6];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i];
                }
            }
            iter += 1;
            progress_bar_clear();
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/evo_{}.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness", false, true).unwrap();
    }
    end_progress_bar();
}

pub fn gtsp_stats_optimized_params(num_repetitions: usize, population_size: usize) {
    let method_names = vec!["local", "local heuristic", "evo", "evo heuristic"];
    let is_method_local = [true, true, false, false];
    let input_files = ["g1", "g2", "g3", "a", "b", "c", "d", "e", "f"];
    let max_iters = [20, 1000, 3000, 25, 200, 600, 1000, 1500, 3000];
    
    let mut iter = 0;
    let total_iters = input_files.len() * num_repetitions;
    start_progress_bar();
    for f in 0..input_files.len() {
        let num_iters = max_iters[f];
        let input_file = input_files[f];
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let mut fitness = GtspFitness {};
        let opt_value = problem.best_known;

        let local_heuristic_init_population = InitHeuristicGtspPopulation { spec: problem.clone(), size: 1 };
        let evo_heuristic_init_population = InitHeuristicGtspPopulation { spec: problem.clone(), size: population_size };

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: num_iters * population_size };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = TruncationReplacementStrategy {};
        let local_crossover = IdentityCrossover {};

        let evo_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: population_size };
        let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let evo_selection = RankSelection { select_count: population_size / 2 };
        let evo_replacement_strategy = TruncationReplacementStrategy {};

        let local_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};
        let evo_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let crossover = GtspGeneralCrossover {
            city_prob: 0.5,
            cycle_prob: 0.0,
            order_prob: 0.5
        };
        
        let mut avg_stats = vec![BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}; method_names.len()];

        for _rep in 0..num_repetitions {
            progress_bar_text(&format!("progress: {:.2}%", 100.0 * (iter as f64) / (total_iters as f64)));

            // local searches
            let (_, stats1) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                local_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);
            
            let (_, stats2) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
                &mut fitness, 
                local_heuristic_init_population.clone(),
                &local_selection,
                &local_crossover,
                local_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);
            
            // evolutionary searches
            let (_, stats3) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                 = evolutionary_search(
                &mut fitness, 
                evo_init_population.clone(),
                &evo_selection,
                &crossover,
                evo_perturbation.clone(), 
                &evo_replacement_strategy,
                &evo_termination_cond,
                false);
            
            let (_, stats4) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
               &mut fitness, 
               evo_heuristic_init_population.clone(),
               &evo_selection,
               &crossover,
               evo_perturbation.clone(), 
               &evo_replacement_strategy,
               &evo_termination_cond,
               false);

            let curr_stats = vec![stats1, stats2, stats3, stats4];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    let step = if is_method_local[s] { population_size } else { 1 };
                    avg_stats[s].fitness[i] += curr_stats[s].fitness[i * step];
                }
            }
            iter += 1;
            progress_bar_clear();
        }
        let log_opt_value = process_avg_stats(&mut avg_stats, opt_value, num_iters, num_repetitions);
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/gtsp/best_{}.svg", input_file).as_str(), input_file, log_opt_value, "Log avg. fitness", false, true).unwrap();
    }
    end_progress_bar();
}

// TODO:
// vizualizace - udelat jen pro euclidovske datasety, vizualizovat pocatecni heuristiku.
// - vizualizovat u nejlepsich algoritmu prubeh v gifu
// udelat convex hull u group bodu ve vizualizacich
//
// vizualizace heuristickeho reseni pouziteho v specializovanem algoritmu, staci jeden priklad pro kazdy dataset (idealne, ale 1 dataset by taky stacil)
// vizualizace behu jednoho vybraneho algoritmu - ukazat ten nejlepsi, idealne na slozitejsim problemu (pokud pujde snadno zobrazit)

pub fn viz_all() {
    let population_size = 64;
    let input_files = ["g1", "g2", "g3"];
    let point_sizes = [4, 4, 2];
    let max_iters = [50, 1000, 3000];
    let local_max_iters = [100, 1000, 7000];
    let local_steps = [1, population_size / 5, population_size / 5];
    let local_delays = [100, 100, 100];

    for i in 0..input_files.len() {
        let num_iters = max_iters[i];
        let input_file = input_files[i];
        let point_size = point_sizes[i];
    
        let problem = Rc::from(load_gtsp_problem(format!("data/gtsp/{}.txt", input_file).as_str()));
        let positions = load_gtsp_positions(format!("data/gtsp/{}_pos.txt", input_file).as_str());
        let mut fitness = GtspFitness {};

        let heuristic_init_population = InitHeuristicGtspPopulation { spec: problem.clone(), size: 1 };
        let heuristic_sol = InitFunc::init(&heuristic_init_population);
        let heuristic_fitness = fitness.eval(&heuristic_sol);
        
        let random_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let random_sol = InitFunc::init(&random_init_population);
        let random_fitness = fitness.eval(&random_sol);

        let local_init_population = InitRandomGtspPopulation { spec: problem.clone(), size: 1 };
        let local_termination_cond = MaxIterTerminationCond { n_iters: max_iters[i] };
        let local_selection = IdentitySelection {};
        let local_replacement_strategy = TruncationReplacementStrategy {};
        let local_crossover = IdentityCrossover {};
        let local_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};

        let (_, local_stats) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatisticsSolutions<GtspPermutation>)
                = evolutionary_search(
                &mut fitness, 
                local_init_population.clone(),
                &local_selection,
                &local_crossover,
                local_perturbation.clone(), 
                &local_replacement_strategy,
                &local_termination_cond,
                false);

        let evo_heuristic_init_population = InitHeuristicGtspPopulation { spec: problem.clone(), size: population_size };
        let evo_termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let evo_selection = RankSelection { select_count: population_size / 2 };
        let evo_replacement_strategy = TruncationReplacementStrategy {};
        let evo_perturbation = CombinePerturbeMutOps { mut_ops: vec![
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspRandGroupVertPerturbation::new(problem.groups.len()))},
            ProbPerturbeMutOp { prob: 0.9, op: Rc::from(GtspReverseGroupPerturbation {})}
        ]};
        let crossover = GtspGeneralCrossover {
            city_prob: 0.5,
            cycle_prob: 0.0,
            order_prob: 0.5
        };

        let (evo_sol, _) : (BSFSingleObjSolution<GtspPermutation>, BSFSingleObjStatistics)
                = evolutionary_search(
               &mut fitness, 
               evo_heuristic_init_population.clone(),
               &evo_selection,
               &crossover,
               evo_perturbation.clone(), 
               &evo_replacement_strategy,
               &evo_termination_cond,
               false);

        let colors = uniform_colors(problem.groups.len());
        let out_file_name = format!("out/gtsp/viz_init_heuristic_{}.svg", input_file);
        let plot_name = format!("{} heuristic", input_file);
        plot_gtsp_solution(&positions, &heuristic_sol, heuristic_fitness, &colors, point_size, &out_file_name, &plot_name).unwrap();
        let out_file_name = format!("out/gtsp/viz_init_random_{}.svg", input_file);
        let plot_name = format!("{} random", input_file);
        plot_gtsp_solution(&positions, &random_sol, random_fitness, &colors, point_size, &out_file_name, &plot_name).unwrap();
        let out_file_name = format!("out/gtsp/viz_best_sol_{}.svg", input_file);
        let plot_name = format!("{} best found", input_file);
        plot_gtsp_solution(&positions, &evo_sol.value, evo_sol.fitness, &colors, point_size, &out_file_name, &plot_name).unwrap();
        let out_file_name = format!("out/gtsp/viz_local_{}.gif", input_file);
        let plot_name = format!("{} local", input_file);
        plot_gtsp_solutions(&positions, &local_stats.solutons, &local_stats.fitness, local_max_iters[i], local_steps[i], local_delays[i], &colors, point_size, &out_file_name, &plot_name).unwrap();
        
    }
}