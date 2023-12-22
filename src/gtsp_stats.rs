use crate::*;

pub fn gtsp_basic_stats() {
    let problem_names = ["a", "b", "c", "d", "e", "f"];
    for problem_name in problem_names {
        println!("{}", problem_name);
        let problem = load_gtsp_problem(format!("data/gtsp/{}.txt", problem_name).as_str());
        println!("vert count: {}", problem.vert_count);
        println!("group count: {}", problem.groups.len());
        println!("euclidean:  {}", are_distances_euclidean(&problem.distances));
        println!("metric:     {}", are_distances_a_metric(&problem.distances));
        let positions = gtsp_force_directed_positions(&problem);
        let colors = uniform_colors(problem.groups.len(), 0.25, 0.75);
        plot_points(&positions, &colors, "out/points.svg", problem_name).unwrap();
        break;
    }
}