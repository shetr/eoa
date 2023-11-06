use crate::opt_data::*;

use plotters::prelude::*;

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

pub fn plot_multiple(stats: &Vec<Statistics>, fun_names: &Vec<&str>, colors: &Vec<RGBColor>, out_file_name: &str, plot_name: &str, optimum: f64) -> Result<(), Box<dyn std::error::Error>>
{
    let mut max_fitness = f64::NEG_INFINITY;
    let mut min_fitness = f64::INFINITY;
    for i in 0..stats.len() {
        max_fitness = max_fitness.max(stats[i].fitness.iter().copied().fold(f64::NEG_INFINITY, f64::max));
        min_fitness = min_fitness.min(stats[i].fitness.iter().copied().fold(f64::INFINITY, f64::min));
    }
    min_fitness = min_fitness.min(optimum - 1.0);
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

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;


    root.present()?;

    Ok(())
}