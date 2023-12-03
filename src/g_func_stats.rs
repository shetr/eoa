use std::{fs::create_dir, rc::Rc, cell::RefCell};

use crate::*;


pub fn create_g_funcs_comparison_graphs(num_repetitions: usize, num_iters: usize, population_size: usize)
{
    let mut g_fitnesses: Vec<Rc<dyn GFunc>> = Vec::new();
    g_fitnesses.push(Rc::new(G06 {}));
    g_fitnesses.push(Rc::new(G08 {}));
    g_fitnesses.push(Rc::new(G11 {}));
    g_fitnesses.push(Rc::new(G24 {}));
    let method_names = vec!["constrained", "constrained2"];
    let g_names = vec!["g06", "g08", "g11", "g24"];
    let g_means = vec![50.0, 5.0, 0.0, 1.5];
    let g_sizes = vec![50.0, 5.0, 1.0, 2.0];
    create_dir("out").unwrap();
    create_dir("out/g_funcs").unwrap();
    for g_index in 0..g_fitnesses.len() {
        let g_name = g_names[g_index];
        let mut g_fitness = GFuncDyn { func: g_fitnesses[g_index].clone() };
        let mut g_bi_fitness = BiGFunc { g_func: g_fitnesses[g_index].clone() };

        let opt_value = g_fitnesses[g_index].optimum();
        let termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        // TODO: maybe replace with rank selection
        let selection = TournamentSelection { select_count: population_size / 2, rounds_count: 8 };
        let replacement_strategy = TruncationReplacementStrategy {};
        let perturbation = NormalPerturbeRealMutOp::new(0.1 * g_sizes[g_index]);
        let crossover = OnePointCrossover {};
        let mut constrained_transformer = StochasticRankFitnessTransformer::new(0.45, g_fitness.clone());
        let mut multi_obj_transformer = NSGA2FitnessTransformer::new();

        let init_population = InitRandomFloatVecPopulation {
            size: population_size ,vec_size: 2, mean: g_means[g_index], std_dev: 0.3 * g_sizes[g_index]
        };

        let mut avg_stats = vec![
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            SingleObjStatistics { fitness: vec![0.0f64; num_iters]}];

        for _rep in 0..num_repetitions {
            let (_, stats) : (EmptySolution, StochasticRankStatistics<FloatVec>) = general_evolutionary_search(
                &mut g_fitness, 
                init_population.clone(),
                &selection,
                &crossover,
                perturbation.clone(), 
                &replacement_strategy,
                &termination_cond,
                &mut constrained_transformer);

            let curr_stats = vec![&stats, &stats];
            for s in 0..avg_stats.len() {
                for i in 0..num_iters {
                    avg_stats[s].fitness[i] += curr_stats[s].solutions[i].fitness;
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
        plot_multiple(&avg_stats, &method_names, &TAB_COLORS, format!("out/g_funcs/{}.svg", g_name).as_str(), g_name, opt_value.log10()).unwrap();
        
    }
    
}

