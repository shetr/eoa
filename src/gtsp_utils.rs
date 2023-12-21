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
                .map(|num_str| num_str.parse::<usize>().unwrap() - 1)
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

pub fn are_distances_euclidean(distances: &DistanceHalfMatrix) -> bool {
    let vert_count = distances.get_vert_count();
    let mut all_euclidean = true;
    for v1 in 0..vert_count {
        for v2 in 0..vert_count {
            let c = distances.get(v1, v2);
            for v3 in 0..vert_count {
                let a = distances.get(v1, v3);
                let b = distances.get(v2, v3);
                let cos_c = (a * a + b * b - c * c) / (2.0 * a * b);
                if cos_c.abs() - 0.0001 > 1.0 {
                    all_euclidean = false;
                    break;
                }
            }
        }
    }
    all_euclidean
}

pub fn are_distances_a_metric(distances: &DistanceHalfMatrix) -> bool {
    let vert_count = distances.get_vert_count();
    let mut is_metric = true;
    for v1 in 0..vert_count {
        if distances.get(v1, v1) < 0.0 {
            is_metric = false;
            break;
        }
        for v2 in 0..vert_count {
            let c = distances.get(v1, v2);
            if c < 0.0 {
                is_metric = false;
                break;
            }
            for v3 in 0..vert_count {
                let a = distances.get(v1, v3);
                let b = distances.get(v3, v2);
                if c > a + b {
                    is_metric = false;
                    break;
                }
            }
        }
    }
    is_metric
}

#[derive(Clone)]
pub struct GroupVertPos {
    pub group: usize,
    pub pos: [f64; 2]
}

pub fn gtsp_force_directed_positions(problem: &GtspProblem) -> Vec<GroupVertPos> {
    let mut positions = vec![GroupVertPos { group: 0, pos: [0.0, 0.0]}; problem.vert_count];
    let mut max_distance = 1.0;
    for v1 in 0..problem.vert_count {
        for v2 in 0..problem.vert_count {
            let d = problem.distances.get(v1, v2);
            if d > max_distance {
                max_distance = d;
            }
        }
    }
    for group in 0..problem.groups.len() {
        for v in &problem.groups[group] {
            positions[*v].group = group;
            for d in 0..2 {
                positions[*v].pos[d] = rand::random::<f64>() * max_distance;
            }
        }
    }
    let max_iter = 100;
    let mut iter = 0;
    while iter < max_iter {
        
        iter += 1;
    }
    positions
}