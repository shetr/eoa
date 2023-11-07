use crate::opt_data::*;
use crate::tsp::*;

use plotters::prelude::*;

pub const TAB_COLORS: [RGBColor; 8] = [
    RGBColor(4, 88, 147),
    RGBColor(219, 97, 0),
    RGBColor(16, 128, 16),
    RGBColor(116, 73, 156),
    RGBColor(0, 157, 174),
    RGBColor(180, 12, 13),
    RGBColor(154, 156, 7),
    RGBColor(193, 88, 160)
];

pub fn plot(stats: &Statistics, out_name: &str, fun_name: &str) -> Result<(), Box<dyn std::error::Error>>
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

pub fn plot_multiple(stats: &Vec<Statistics>, fun_names: &Vec<&str>, colors: &[RGBColor], out_file_name: &str, plot_name: &str, log_optimum: f64) -> Result<(), Box<dyn std::error::Error>>
{
    let mut max_fitness = f64::NEG_INFINITY;
    let mut min_fitness = f64::INFINITY;
    for i in 0..stats.len() {
        max_fitness = max_fitness.max(stats[i].fitness.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        min_fitness = min_fitness.min(stats[i].fitness.iter().copied().fold(f64::INFINITY, f64::min));
    }
    min_fitness = min_fitness.min(log_optimum);
    let range = max_fitness - min_fitness;
    min_fitness -= 0.1 * range;
    let root = SVGBackend::new(out_file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_name, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0..stats[0].fitness.len(), min_fitness..max_fitness)?;

    chart.configure_mesh()
        .x_desc("Iterations")
        .y_desc("Log avg. fitness")
        .draw()?;

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

    let opt_color = colors[stats.len()];
    chart
            .draw_series(LineSeries::new(
                (0..stats[0].fitness.len()).map(|iter| (iter, log_optimum)),
                opt_color,
            ))?
            .label("optimum")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], opt_color));

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
