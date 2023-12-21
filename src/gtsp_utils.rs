use crate::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn load_gtsp_problem(file_path: &str) -> GtspProblem {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let vert_count = lines[0].parse::<usize>().unwrap();
    let group_count = lines[1].parse::<usize>().unwrap();
    let best_known = lines[2].parse::<f64>().unwrap();
    let mut problem = GtspProblem {
        vert_count,
        best_known,
        groups: vec![Vec::<usize>::new(); group_count],
        distances: DistanceHalfMatrix::new(vert_count)
    };

    for g in 0..group_count {
        let line = &lines[g + 3];
        let mut line_nums : Vec<usize> = line.split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect();
        line_nums.remove(0);
        problem.groups[g] = line_nums;
    }

    for v in 0..vert_count {
        let line = &lines[v + 3 + group_count];
        let line_nums : Vec<f64> = line.split_whitespace()
                .map(|num_str| num_str.parse::<f64>().unwrap())
                .collect();
        for u in 0..vert_count {
            problem.distances.set(v, u, line_nums[u]);
        }
    }

    problem
}