use std::{fs::create_dir_all, rc::Rc};

use crate::*;

#[derive(Clone)]
pub struct GFuncMultiObjSolution<T: OptData> {
    pub value: T,
    pub fitness: Vec<f64>
}

impl<T: OptData> Solution<T, Vec<f64>, NSGA2Fitness> for GFuncMultiObjSolution<T> {
    fn from_population(population: &Vec<T>, fitness_in: &Vec<Vec<f64>>, _fitness_opt: &Vec<NSGA2Fitness>) -> Self {
        let mut best_index = 0usize;
        for i in 1..population.len() {
            let mut i_violations = 0.0;
            let mut best_violations = 0.0;
            for j in 1.. fitness_in[i].len() {
                i_violations += fitness_in[i][j];
                best_violations += fitness_in[best_index][j];
            }
            if i_violations < best_violations {
                best_index = i;
            } else if i_violations == best_violations {
                if fitness_in[i][0] < fitness_in[best_index][0] {
                    best_index = i;
                }
            }
        }
        GFuncMultiObjSolution { value: population[best_index].clone(), fitness: fitness_in[best_index].clone() }
    }

    fn diff(&self, other: &Self) -> f64 {
        self.fitness[0] - other.fitness[0]
    }

    fn is_better(&self, other: &Self) -> bool {
        let mut i_violations = 0.0;
        let mut best_violations = 0.0;
        for j in 1.. self.fitness.len() {
            i_violations += self.fitness[j];
            best_violations += other.fitness[j];
        }
        i_violations < best_violations || (i_violations == best_violations && self.fitness[0] < other.fitness[0])
    }
}

#[derive(Clone)]
pub struct GFuncMultiObjStatistics<T: OptData> {
    pub solutions: Vec<GFuncMultiObjSolution<T>>
}

impl<T: OptData> Statistics<T, Vec<f64>, NSGA2Fitness> for GFuncMultiObjStatistics<T> {
    fn new() -> Self {
        GFuncMultiObjStatistics { solutions: Vec::<GFuncMultiObjSolution<T>>::new() }
    }

    fn report_iter(&mut self, _iter: usize, population: &Vec<T>, fitness_in: &Vec<Vec<f64>>, fitness_opt: &Vec<NSGA2Fitness>) {
        let mut solution = GFuncMultiObjSolution::from_population(population, fitness_in, fitness_opt);
        if let Some(last) = self.solutions.last() {
            if last.is_better(&solution) {
                solution = last.clone();
            }
        }
        self.solutions.push(solution);
    }
}

pub fn create_g_funcs_comparison_graphs(num_repetitions: usize, num_iters: usize, population_size: usize)
{
    let g_fitnesses: Vec<Rc<dyn GFunc>> = vec![
        // basic problems
        Rc::new(G06 {}),
        Rc::new(G08 {}),
        Rc::new(G11 {}),
        Rc::new(G24 {}),
        // harder problems
        Rc::new(G04 {}),
        Rc::new(G05 {}),
        Rc::new(G09 {}),
        Rc::new(G21 {}),
    ];
    let method_names = vec!["Stochastic Ranking", "NSGA-II 2-vals", "NSGA-II n-vals"];
    let g_names = vec!["g06", "g08", "g11", "g24", "h_g04", "h_g05", "h_g09", "h_g21"];
    create_dir_all("out/g_funcs").unwrap();
    for g_index in 0..g_fitnesses.len() {
        let g_name = g_names[g_index];
        let mut g_fitness = GFuncDyn { func: g_fitnesses[g_index].clone() };
        let mut g_bi_fitness = BiGFunc { g_func: g_fitnesses[g_index].clone() };
        let mut g_multi_fitness = MultiGFunc { g_func: g_fitnesses[g_index].clone() };
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

        let optimum = g_fitness.optimum();
        let opt_value = g_fitness.eval(&optimum);
        let termination_cond = MaxIterTerminationCond { n_iters: num_iters };
        let selection = RankSelection { select_count: population_size / 2 };
        let replacement_strategy = TruncationReplacementStrategy {};
        let perturbation = BoundedNormalPerturbeRealMutOp::new(
            0.1 * val_range,
            &bounds
        );
        let crossover = ArithmetricCrossover {};
        let mut constrained_transformer = StochasticRankFitnessTransformer::new(
            0.45, 
            g_fitness.clone()
        );
        let mut multi_obj_transformer = NSGA2FitnessTransformer::new();

        let init_population = InitRandomFloatVecPopulation {
            size: population_size ,vec_size: g_fitness.vec_size(), mean: mean, std_dev: 0.3 * val_range, bounds: bounds.clone()
        };

        let mut avg_fitness_stats = vec![
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}
        ];
        let mut avg_constraints_stats = vec![
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]},
            BSFSingleObjStatistics { fitness: vec![0.0f64; num_iters]}
        ];

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

            let (_, stats2) : (EmptySolution, GFuncMultiObjStatistics<FloatVec>) = general_evolutionary_search(
                &mut g_bi_fitness, 
                init_population.clone(),
                &selection,
                &crossover,
                perturbation.clone(), 
                &replacement_strategy,
                &termination_cond,
                &mut multi_obj_transformer);

            let (_, stats3) : (EmptySolution, GFuncMultiObjStatistics<FloatVec>) = general_evolutionary_search(
                &mut g_multi_fitness, 
                init_population.clone(),
                &selection,
                &crossover,
                perturbation.clone(), 
                &replacement_strategy,
                &termination_cond,
                &mut multi_obj_transformer);
            
            for i in 0..num_iters {
                // stats1
                avg_fitness_stats[0].fitness[i] += stats1.solutions[i].fitness;
                avg_constraints_stats[0].fitness[i] += stats1.solutions[i].violations;
                // stats2
                avg_fitness_stats[1].fitness[i] += stats2.solutions[i].fitness[0];
                avg_constraints_stats[1].fitness[i] += stats2.solutions[i].fitness[1];
                // stats3
                avg_fitness_stats[2].fitness[i] += stats3.solutions[i].fitness[0];
                for j in 1..stats3.solutions[i].fitness.len() {
                    avg_constraints_stats[2].fitness[i] += stats3.solutions[i].fitness[j];
                }
            }

        }
        let log_opt_value = process_avg_stats(&mut avg_fitness_stats, opt_value, num_iters, num_repetitions);
        let log_1 = process_avg_stats(&mut avg_constraints_stats, 0.0, num_iters, num_repetitions);
        let fitness_plot_name = format!("{}_fitness", g_name);
        let fitness_plot_filename = format!("out/g_funcs/{}.svg", fitness_plot_name);
        let constraints_plot_name = format!("{}_constraints", g_name);
        let constraints_plot_filename = format!("out/g_funcs/{}.svg", constraints_plot_name);
        plot_multiple(&avg_fitness_stats, &method_names, &TAB_COLORS, fitness_plot_filename.as_str(), fitness_plot_name.as_str(), log_opt_value, "Log avg. fitness", true, true).unwrap();
        plot_multiple(&avg_constraints_stats, &method_names, &TAB_COLORS, constraints_plot_filename.as_str(), constraints_plot_name.as_str(), log_1, "Log avg. constraints sum", true, true).unwrap();
        
    }
    
}

