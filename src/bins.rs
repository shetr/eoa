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
    //gtsp_basic_stats_default_params(args.num_rep, args.num_iters, args.pop_size);
    gtsp_viz_gen_solution(args.num_iters, args.pop_size);
}