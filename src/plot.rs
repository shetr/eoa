use std::rc::Rc;

use crate::GroupVertPos;
use crate::GtspPermutation;
use crate::GtspProblem;
use crate::jarvis_convex_hull;
use crate::opt_data::*;
use crate::tsp::*;
use rand::seq::SliceRandom;

use plotters::prelude::*;

pub const TAB_COLORS: [RGBColor; 8] = [
    RGBColor(4, 88, 147),
    RGBColor(219, 97, 0),
    RGBColor(16, 128, 16),
    RGBColor(116, 73, 156),
    RGBColor(180, 12, 13),
    RGBColor(154, 156, 7),
    RGBColor(0, 157, 174),
    RGBColor(193, 88, 160)
];

pub fn plot(stats: &BSFSingleObjStatistics, out_name: &str, fun_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let max_fitness = stats.fitness.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_fitness = stats.fitness.iter().copied().fold(f64::INFINITY, f64::min);
    let root = SVGBackend::new(out_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(fun_name, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0..stats.fitness.len(), min_fitness..max_fitness)?;

    chart.configure_mesh()
        .x_desc("Iterations")
        .y_desc("Fitness")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..stats.fitness.len()).map(|iter| (iter, stats.fitness[iter])),
            &RED,
        ))?;

    root.present()?;

    Ok(())
}

pub fn plot_multiple(stats: &Vec<BSFSingleObjStatistics>, fun_names: &Vec<&str>, colors: &[RGBColor], out_file_name: &str, plot_name: &str, log_optimum: f64, y_desc: &str, use_optimum: bool, use_mesh: bool) -> Result<(), Box<dyn std::error::Error>>
{
    let mut max_fitness = f64::NEG_INFINITY;
    let mut min_fitness = f64::INFINITY;
    for i in 0..stats.len() {
        max_fitness = max_fitness.max(stats[i].fitness.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        min_fitness = min_fitness.min(stats[i].fitness.iter().copied().fold(f64::INFINITY, f64::min));
    }
    if use_optimum {
        min_fitness = min_fitness.min(log_optimum);
    }
    let range = max_fitness - min_fitness;
    min_fitness -= 0.1 * range;
    if max_fitness == min_fitness {
        max_fitness = min_fitness + 1.0;
    }
    let root = SVGBackend::new(out_file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_name, ("sans-serif", 50).into_font())
        .margin(5)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..stats[0].fitness.len(), min_fitness..max_fitness)?;

    if use_mesh {
        chart.configure_mesh()
            .x_desc("Iterations")
            .y_desc(y_desc)
            .light_line_style(WHITE)
            .draw()?;
    } else {
        chart.configure_mesh()
            .x_desc("Iterations")
            .y_desc(y_desc)
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;
    }

    for i in 0..stats.len() {
        let color = colors[i];
        chart
            .draw_series(LineSeries::new(
                (0..stats[i].fitness.len()).map(|iter| (iter, stats[i].fitness[iter])),
                colors[i].clone(),
            ))?
            .label(fun_names[i])
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    if use_optimum {
        let opt_color = colors[stats.len()];
        chart
            .draw_series(LineSeries::new(
                (0..stats[0].fitness.len()).map(|iter| (iter, log_optimum)),
                opt_color,
            ))?
            .label("optimum")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], opt_color));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;


    root.present()?;

    Ok(())
}

pub fn plot_tsp_viz(positions: &Vec<[f64; 2]>, perm: &TspPermutation, out_file_name: &str, plot_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let mut max = [f64::NEG_INFINITY; 2];
    let mut min = [f64::INFINITY; 2];
    for i in 0..positions.len() {
        for d in 0..2 {
            max[d] = max[d].max(positions[i][d]);
            min[d] = min[d].min(positions[i][d]);
        }
    }

    let root = SVGBackend::new(out_file_name, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_name, ("sans-serif", 50))
        .build_cartesian_2d(min[0]..max[0], min[1]..max[1])?;

    let mut vertices: Vec<(f64, f64)> = (0..perm.vert_perm.len()).map(|i| {
        (positions[perm.vert_perm[i]][0], positions[perm.vert_perm[i]][1])
    }).collect();
    vertices.push((positions[perm.vert_perm[0]][0], positions[perm.vert_perm[0]][1]));

    chart.draw_series(std::iter::once(PathElement::new(vertices, RED)))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present()?;
    Ok(())
}


pub fn plot_gtsp_points(positions: &Vec<GroupVertPos>, group_colors: &Vec<RGBColor>, point_size: i32, out_file_name: &str, plot_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let mut max = [f64::NEG_INFINITY; 2];
    let mut min = [f64::INFINITY; 2];
    for i in 0..positions.len() {
        for d in 0..2 {
            max[d] = max[d].max(positions[i].pos[d]);
            min[d] = min[d].min(positions[i].pos[d]);
        }
    }

    for d in 0..2 {
        let diameter = max[d] - min[d];
        max[d] += 0.1 * diameter;
        min[d] -= 0.1 * diameter;
    }

    let root = SVGBackend::new(out_file_name, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_name, ("sans-serif", 50))
        .build_cartesian_2d(min[0]..max[0], min[1]..max[1])?;

    chart.draw_series(
        positions
            .iter()
            .map(|vert| Circle::new((vert.pos[0], vert.pos[1]), point_size, group_colors[vert.group].filled())),
    )?;

    root.present()?;
    Ok(())
}

fn get_convex_hulls(positions: &Vec<GroupVertPos>, spec: Rc<GtspProblem>) -> Vec<Vec<(f64, f64)>>
{
    let mut hulls = vec![Vec::<(f64, f64)>::new(); spec.groups.len()];
    let mut hull = Vec::<GroupVertPos>::new();
    let mut group_positions = Vec::<GroupVertPos>::new();
    for g in 0..spec.groups.len() {
        group_positions.clear();
        for v in &spec.groups[g] {
            group_positions.push(positions[*v].clone());
        }
        jarvis_convex_hull(&group_positions, &mut hull);
        for i in 0..hull.len() {
            hulls[g].push((hull[i].pos[0], hull[i].pos[1]));
        }
        hulls[g].push((hull[0].pos[0], hull[0].pos[1]));
    }
    hulls
}

const HULL_COLOR_MUL: f64 = 0.25;

pub fn plot_gtsp_solution(positions: &Vec<GroupVertPos>, solution: &GtspPermutation, fitness: f64, group_colors: &Vec<RGBColor>, point_size: i32, out_file_name: &str, plot_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let hulls = get_convex_hulls(positions, solution.spec.clone());

    let root: DrawingArea<SVGBackend<'_>, plotters::coord::Shift> = SVGBackend::new(out_file_name, (640, 480)).into_drawing_area();
    let mut max = [f64::NEG_INFINITY; 2];
    let mut min = [f64::INFINITY; 2];
    for i in 0..positions.len() {
        for d in 0..2 {
            max[d] = max[d].max(positions[i].pos[d]);
            min[d] = min[d].min(positions[i].pos[d]);
        }
    }

    for d in 0..2 {
        let diameter = max[d] - min[d];
        max[d] += 0.1 * diameter;
        min[d] -= 0.1 * diameter;
    }

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_name, ("sans-serif", 50))
        .margin(2)
        .set_label_area_size(LabelAreaPosition::Bottom, 10)
        .build_cartesian_2d(min[0]..max[0], min[1]..max[1])?;

    let mut vertices: Vec<(f64, f64)> = (0..solution.perm.len()).map(|i| {
        let v = solution.spec.groups[solution.perm[i].group][solution.perm[i].vert];
        (positions[v].pos[0], positions[v].pos[1])
    }).collect();
    let v0 = solution.spec.groups[solution.perm[0].group][solution.perm[0].vert];
    vertices.push((positions[v0].pos[0], positions[v0].pos[1]));

    chart.configure_mesh()
        .x_desc(format!("Fitness = {:.2}", fitness))
        .y_desc("")
        .disable_x_mesh()
        .disable_y_mesh()
        .disable_x_axis()
        .draw()?;

    chart.draw_series(std::iter::once(PathElement::new(vertices, BLACK)))?;

    for g in 0..hulls.len() {
        chart.draw_series(std::iter::once(PathElement::new(hulls[g].clone(), group_colors[g].mix(HULL_COLOR_MUL))))?;
    }

    chart.draw_series(
        positions
            .iter()
            .map(|vert| Circle::new((vert.pos[0], vert.pos[1]), point_size, group_colors[vert.group].filled())),
    )?;

    root.present()?;
    Ok(())
}

pub fn plot_gtsp_solutions(positions: &Vec<GroupVertPos>, solutions: &Vec<GtspPermutation>, fitness: &Vec<f64>, max_iter: usize, iter_step: usize, delay: u32, group_colors: &Vec<RGBColor>, point_size: i32, out_file_name: &str, plot_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let hulls = get_convex_hulls(positions, solutions[0].spec.clone());

    let mut max = [f64::NEG_INFINITY; 2];
    let mut min = [f64::INFINITY; 2];
    for i in 0..positions.len() {
        for d in 0..2 {
            max[d] = max[d].max(positions[i].pos[d]);
            min[d] = min[d].min(positions[i].pos[d]);
        }
    }

    for d in 0..2 {
        let diameter = max[d] - min[d];
        max[d] += 0.1 * diameter;
        min[d] -= 0.1 * diameter;
    }

    let root = BitMapBackend::gif(out_file_name, (640, 480), delay)?.into_drawing_area();

    for i in (0..solutions.len().min(max_iter)).step_by(iter_step) {
        let solution = &solutions[i];
        let fitness = fitness[i];

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("{}, iter: {}", plot_name, i), ("sans-serif", 50))
            .margin(2)
            .set_label_area_size(LabelAreaPosition::Bottom, 10)
            .build_cartesian_2d(min[0]..max[0], min[1]..max[1])?;

        let mut vertices: Vec<(f64, f64)> = (0..solution.perm.len()).map(|i| {
            let v = solution.spec.groups[solution.perm[i].group][solution.perm[i].vert];
            (positions[v].pos[0], positions[v].pos[1])
        }).collect();
        let v0 = solution.spec.groups[solution.perm[0].group][solution.perm[0].vert];
        vertices.push((positions[v0].pos[0], positions[v0].pos[1]));

        chart.configure_mesh()
            .x_desc(format!("Fitness = {:.2}", fitness))
            .y_desc("")
            .disable_x_mesh()
            .disable_y_mesh()
            .disable_x_axis()
            .draw()?;

        chart.draw_series(std::iter::once(PathElement::new(vertices, BLACK)))?;

        for g in 0..hulls.len() {
            chart.draw_series(std::iter::once(PathElement::new(hulls[g].clone(), group_colors[g].mix(HULL_COLOR_MUL))))?;
        }

        chart.draw_series(
            positions
                .iter()
                .map(|vert| Circle::new((vert.pos[0], vert.pos[1]), point_size, group_colors[vert.group].filled())),
        )?;

        root.present()?;
    }
    root.present().expect("Unable to write result to file");

    Ok(())
}

fn hsl_to_rgb(color: HSLColor) -> RGBColor {
    let (r, g, b) = color.rgb();
    RGBColor(r, g, b)
}

pub fn uniform_colors(colors_count: usize) -> Vec<RGBColor> {
    let mut colors = Vec::<RGBColor>::with_capacity(colors_count);
    for i in 0..colors_count {
        colors.push(hsl_to_rgb(HSLColor((i as f64) / (colors_count as f64), 0.9, 0.5)));
    }
    colors.shuffle(&mut rand::thread_rng());
    colors
}

pub fn rand_colors(colors_count: usize, min: f64, max: f64) -> Vec<RGBColor> {
    let mut colors = Vec::<RGBColor>::with_capacity(colors_count);
    let size = max - min;
    for _ in 0..colors_count {
        colors.push(
            RGBColor(
                ((min + size * rand::random::<f64>()) * 255.0) as u8,
                ((min + size * rand::random::<f64>()) * 255.0) as u8,
                ((min + size * rand::random::<f64>()) * 255.0) as u8
            )
        );
    }
    colors
}