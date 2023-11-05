use crate::tsp::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn load_opt_permutation(file_path: &str) -> TspPermutation {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let vert_perm: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap() - 1)
        .collect();
    TspPermutation { vert_perm: vert_perm }
}

pub fn load_vert_positions(file_path: &str) -> Vec<[f64; 2]> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let positions: Vec<[f64; 2]> = reader
        .lines()
        .map(|line| {
            let line_nums : Vec<f64> = line.unwrap().split_whitespace()
                .map(|num_str| num_str.parse::<f64>().unwrap())
                .collect();
            if line_nums.len() != 2 {
                panic!("incorect format of vertex positons");
            }
            [line_nums[0], line_nums[1]]
        })
        .collect();
    positions
}

pub fn vert_positions_to_distances(vert_positions: &Vec<[f64; 2]>) -> DistanceHalfMatrix {
    let mut distances = DistanceHalfMatrix::new(vert_positions.len());
    for v1 in 0..vert_positions.len() {
        for v2 in 0..vert_positions.len() {
            let mut dist = 0.0f64;
            for d in 0..2 {
                let diff = vert_positions[v1][d] - vert_positions[v2][d];
                dist += diff * diff;
            }
            distances.set(v1, v2, dist.sqrt());
        }
    }
    distances
}