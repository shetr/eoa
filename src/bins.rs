use std::fs::create_dir_all;

use crate::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'r', long, default_value_t = 7)]
    num_rep: usize,
    #[arg(short = 'i', long, default_value_t = 3000)]
    num_iters: usize,
    #[arg(short = 'p', long, default_value_t = 64)]
    pop_size: usize
}

pub fn hw1()
{
    let args = Cli::parse();
    create_comparison_graphs(args.num_rep, args.num_iters, args.pop_size);
    create_vizualization_graphs(args.num_iters, args.pop_size);
}

pub fn hw2()
{
    let args = Cli::parse();
    create_g_funcs_comparison_graphs(args.num_rep, args.num_iters, args.pop_size);
}

pub fn sem()
{
    create_dir_all("out/gtsp").unwrap();
    // TODO: try seeding random generators
    let args = Cli::parse();
    //gtsp_basic_stats_gen_instance();
    //gtsp_gen_problem(100, 20, "g2");
    //gtsp_gen_problem(500, 80, "g3");
    //gtsp_viz_gen_solution(args.num_iters, args.pop_size);
    //gtsp_find_opt_params_local_search(args.num_rep, args.num_iters, 10);
    //gtsp_find_opt_params_evolutionary_search(args.num_rep, args.num_iters, args.pop_size, 10);
    //gtsp_find_opt_params_evolutionary_search_with_local_search(args.num_rep, args.pop_size);

    let plot_stats = false;
    let plot_viz = true;

    // stats
    if plot_stats {
        gtsp_basic_stats_default_params(args.num_rep, args.pop_size);
        println!("default algs plots finished");
        gtsp_local_search_stats(args.num_rep);
        println!("local algs plots finished");
        gtsp_evolutionary_search_stats(args.num_rep, args.pop_size);
        println!("evolutionary algs plots finished");
        gtsp_stats_optimized_params(args.num_rep, args.pop_size);
        println!("best algs plots finished");
    }

    if plot_viz {
        // viz
        init_solution_viz();
    }
}