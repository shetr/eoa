use crate::opt::*;

use plotters::prelude::*;

// TODO: plotovat do svg

pub fn plot(stats: &Statistics, out_name: &str, fun_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let max_fitness = stats.fitness.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_fitness = stats.fitness.iter().copied().fold(f64::INFINITY, f64::min);
    let root = BitMapBackend::new(out_name, (640, 480)).into_drawing_area();
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