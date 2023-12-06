use std::{fs::create_dir_all, rc::Rc};

use crate::*;


pub fn create_g_funcs_comparison_graphs(num_repetitions: usize, num_iters: usize, population_size: usize)
{
    let g_fitnesses: Vec<Rc<dyn GFunc>> = vec![
        Rc::new(G06 {}),
        Rc::new(G08 {}),
        Rc::new(G11 {}),
        Rc::new(G24 {})
    ];
    let method_names = vec!["constrained", "constrained2"];
    let g_names = vec!["g06", "g08", "g11", "g24"];
    create_dir_all("out/g_funcs").unwrap();
    for g_index in 0..g_fitnesses.len() {
        let g_name = g_names[g_index];
        let mut g_fitness = GFuncDyn { func: g_fitnesses[g_index].clone() };
        let mut g_bi_fitness = BiGFunc { g_func: g_fitnesses[g_index].clone() };
        let bounds = g_fitness.bounds();
        let mut mean = 0.0;
        let mut val_range = 0.0;
        for bound in &bounds {
            let diff = bound.upper - bound.lower;
            mean += diff * 0.5;
            val_range += diff;
        }
        mean /= bounds.len() as f64;
        val_range /= bounds.len() as f64;

        let opt_value = g_fitness.optimum();
        let termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        // TODO: maybe replace with rank selection
        let selection = TournamentSelection { select_count: population_size / 2, rounds_count: 8 };
        let replacement_strategy = TruncationReplacementStrategy {};
        // TODO: try BoundedNormalOneFiftPerturbeRealMutOp
        let perturbation = BoundedNormalPerturbeRealMutOp::new(
            0.1 * val_range,
            &bounds
        );
        // TODO: try ArithmetricCrossover
        let crossover = OnePointCrossover {};
        let mut constrained_transformer = StochasticRankFitnessTransformer::new(
            0.45, 
            g_fitness.clone()
        );
        let mut multi_obj_transformer = NSGA2FitnessTransformer::new();

        let init_population = InitRandomFloatVecPopulation {
            size: population_size ,vec_size: 2, mean: mean, std_dev: 0.3 * val_range
        };

        let mut avg_stats = vec![
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}];

        for _rep in 0..num_repetitions {
            let (_, stats1) : (EmptySolution, StochasticRankStatistics<FloatVec>) = general_evolutionary_search(
                &mut g_fitness, 
                init_population.clone(),
                &selection,
                &crossover,
                perturbation.clone(), 
                &replacement_strategy,
                &termination_cond,
                &mut constrained_transformer);

            let (_, stats2) : (EmptySolution, MultiObjStatistics<FloatVec>) = general_evolutionary_search(
                &mut g_bi_fitness, 
                init_population.clone(),
                &selection,
                &crossover,
                perturbation.clone(), 
                &replacement_strategy,
                &termination_cond,
                &mut multi_obj_transformer);
            
            for i in 0..num_iters {
                avg_stats[0].fitness[i] += stats1.solutions[i].fitness;
            }
            for i in 0..num_iters {
                let mut best_fitness = f64::INFINITY;
                for fitness in &stats2.solutions[i].fitness {
                    best_fitness = best_fitness.min(fitness[0]);
                }
                avg_stats[0].fitness[i] += best_fitness;
            }

        }
        let mut fitness_min = opt_value;
        for s in 0..avg_stats.len() {
            for i in 0..num_iters {
                avg_stats[s].fitness[i] /= num_repetitions as f64;
                fitness_min = fitness_min.min(avg_stats[s].fitness[i]);
            }
        }
        let log_opt_value = (opt_value - fitness_min + 1.0).log10();
        // log scale
        for s in 0..avg_stats.len() {
            for i in 0..num_iters {
                avg_stats[s].fitness[i] = (avg_stats[s].fitness[i] - fitness_min + 1.0).log10();
            }
            //plot(&avg_stats[s], format!("out/tsp/{}_{}.svg", method_names[s], input_file).as_str(), method_names[s]).unwrap();
        }
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/g_funcs/{}.svg", g_name).as_str(), g_name, log_opt_value).unwrap();
        
    }
    
}

