use eoa::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'r', long, default_value_t = 7)]
    num_rep: usize,
    #[arg(short = 'i', long, default_value_t = 3000)]
    num_iters: usize,
    #[arg(short = 'p', long, default_value_t = 50)]
    pop_size: usize
}

fn main()
{
    let args = Cli::parse();
}