use crate::*;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use rand::Rng;

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

pub fn save_gtsp_problem(file_path: &str, problem: &GtspProblem)
{
    let mut file = File::create(file_path).expect("unable to create a file.");
    file.write(format!("{}\n", problem.vert_count).as_bytes()).unwrap();
    file.write(format!("{}\n", problem.groups.len()).as_bytes()).unwrap();
    file.write(format!("{}\n", problem.best_known).as_bytes()).unwrap();
    for g in 0..problem.groups.len() {
        let line: String = problem.groups[g].iter().map(|v| (v + 1).to_string())
            .fold(problem.groups[g].len().to_string(), |acc, s| String::from(acc) + " " + &s);
        file.write(format!("{}\n", line).as_bytes()).unwrap();
    }
    for v1 in 0..problem.vert_count {
        let mut line = String::from("");
        for v2 in 0..problem.vert_count {
            if v2 > 0 {
                line += " ";
            }
            line += &problem.distances.get(v1, v2).to_string();
        }
        file.write(format!("{}\n", line).as_bytes()).unwrap();
    }
}

pub fn load_gtsp_positions(file_path: &str) -> Vec<GroupVertPos>
{
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line_nums : Vec<String> = line.unwrap().split_whitespace().map(|num| String::from(num)).collect();
            if line_nums.len() != 3 {
                panic!("incorect format of vertex positons");
            }
            GroupVertPos {
                group: line_nums[0].parse().unwrap(),
                pos: [
                    line_nums[1].parse::<f64>().unwrap(),
                    line_nums[2].parse::<f64>().unwrap()
                ]
            }
        })
        .collect()
}

pub fn save_gtsp_positions(file_path: &str, positions: &Vec<GroupVertPos>)
{
    let mut file = File::create(file_path).expect("unable to create a file.");
    for v in positions {
        file.write(format!("{} {} {}\n", v.group, v.pos[0], v.pos[1]).as_bytes()).unwrap();
    }
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

// abbandoned, it would be probably very misleading
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
    // TODO: implement force-directed layout, attracting forces are based on distance matrix and group idex, repeling force by distance in vizualization
    let max_iter = 100;
    let mut iter = 0;
    while iter < max_iter {
        
        iter += 1;
    }
    positions
}

pub fn gen_euclidean_gtsp_problem(vert_count: usize, group_count: usize) -> (GtspProblem, Vec<GroupVertPos>)
{
    assert!(group_count < vert_count);
    let mut problem = GtspProblem {
        vert_count,
        best_known: 1.0,
        groups: vec![Vec::<usize>::new(); group_count],
        distances: DistanceHalfMatrix::new(vert_count)
    };
    // generate vertex positons
    let mut positions = vec![GroupVertPos { group: 0, pos: [0.0, 0.0]}; vert_count];
    let dim_size = (vert_count as f64).sqrt().ceil();
    for i in 0..vert_count {
        for d in 0..2 {
            positions[i].pos[d] =  rand::random::<f64>() * dim_size;
        }
    }
    // compute distances
    problem.distances = gtsp_positions_to_distances(&positions);
    // k-means clustering
    let mut etalons = vec![[0.0; 2]; group_count];
    // init etalons to some vertices
    let init_step = vert_count / group_count;
    for g in 0..group_count {
        etalons[g] = positions[g * init_step].pos;
    }
    let mut terminate = false;
    let mut prev_groups = vec![Vec::<usize>::new(); group_count];
    while !terminate {
        for g in 0..group_count {
            problem.groups[g].clear();
        }
        // asssign points to closest etalons
        for v in 0..vert_count {
            let mut closest = 0;
            let mut closest_dist = squared_distance(positions[v].pos, etalons[0]);
            for g in 1..group_count {
                let dist = squared_distance(positions[v].pos, etalons[g]);
                if dist < closest_dist {
                    closest = g;
                    closest_dist = dist;
                }
            }
            problem.groups[closest].push(v);
        }
        // recompute etalons
        for g in 0..group_count {
            etalons[g] = [0.0; 2];
            if problem.groups[g].len() > 0 {
                // compute mean
                for v in &problem.groups[g] {
                    for d in 0..2 {
                        etalons[g][d] += positions[*v].pos[d];
                    }
                }
                for d in 0..2 {
                    etalons[g][d] /= problem.groups[g].len() as f64;
                }
            } else {
                // re-init etalon
                etalons[g] = positions[rand::thread_rng().gen_range(0..positions.len())].pos;
            }
        }
        // check for termination
        terminate = true;
        for g in 0..group_count {
            if prev_groups[g].len() != problem.groups[g].len() {
                terminate = false;
                break;
            }
            for i in 0..prev_groups[g].len() {
                if prev_groups[g][i] != problem.groups[g][i] {
                    terminate = false;
                    break;
                }
            }
            if !terminate {
                break;
            }
        }
        // assing current result
        for g in 0..group_count {
            prev_groups[g] = problem.groups[g].clone();
        }
    }
    // assign groups to positons
    for g in 0..group_count {
        for v in &problem.groups[g] {
            positions[*v].group = g;
        }
    }
    // return result
    (problem, positions)
}

pub fn gtsp_positions_to_distances(positions: &Vec<GroupVertPos>) -> DistanceHalfMatrix {
    vert_positions_to_distances(&positions.iter().map(|gpos| gpos.pos).collect())
}

pub fn gtsp_group_avg_distances(problem: GtspProblem) -> DistanceHalfMatrix {
    let mut distances = DistanceHalfMatrix::new(problem.groups.len());
    for g1 in 0..problem.groups.len() {
        for g2 in 0..g1 {
            let mut dist_sum = 0.0;
            for v1 in &problem.groups[g1] {
                for v2 in &problem.groups[g2] {
                    dist_sum += problem.distances.get(*v1, *v2);
                }
            }
            distances.set(g1, g2, dist_sum / ((problem.groups[g1].len() * problem.groups[g2].len()) as f64))
        }
    }
    distances
}

enum OrientationType {
    RightTurn,
    Straight,
    LeftTurn
}

fn orient2d(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> OrientationType
{
    let res = (a[0] - c[0])*(b[1] - c[1]) - (a[1] - c[1])*(b[0] - c[0]);
    if res > 0.0 {
        OrientationType::LeftTurn
    } else if res < 0.0 {
        OrientationType::RightTurn
    } else {
        OrientationType::Straight
    }
}

fn extended(p: [f64; 2], q: [f64; 2], r: [f64; 2]) -> bool
{
    return (q[0]-p[0]) * (r[0]-q[0]) >= (p[1]-q[1]) * (r[1]-q[1]);
}

pub fn jarvis_convex_hull(points: &Vec<GroupVertPos>, out_hull: &mut Vec<GroupVertPos>)
{
    out_hull.clear();
    if points.len() <= 2 {
        for i in 0..points.len() {
            out_hull.push(points[i].clone());
        }
        return;
    }
    let mut min_y_point_index = 0usize;
    for i in 1..points.len() {
        if points[i].pos[1] < points[min_y_point_index].pos[1] {
            min_y_point_index = i;
        }
    }
    let mut current_index = min_y_point_index;
    let mut iter = 0usize;

    loop {
        out_hull.push(points[current_index].clone());
        let mut rightmost_index = 0usize;
        for i in 1..points.len() {
            if i == current_index {
                continue;
            }
            let orient_res = orient2d(points[current_index].pos, points[rightmost_index].pos, points[i].pos);
            match orient_res {
                OrientationType::RightTurn => {
                    rightmost_index = i;
                },
                OrientationType::Straight => {
                    if extended(points[current_index].pos, points[rightmost_index].pos, points[i].pos) {
                        rightmost_index = i;
                    }
                },
                OrientationType::LeftTurn => ()
            }
        }
        current_index = rightmost_index;
        iter += 1;
        if current_index == min_y_point_index || iter >= points.len() {
            break;
        }
    }

}
