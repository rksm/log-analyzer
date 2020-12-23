use std::path::Path;

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};

use crate::stats::RequestStats;

pub fn draw_pictures<P: AsRef<Path> + std::fmt::Debug>(path: P, stats: &RequestStats) {
    let data: Vec<(String, f32)> = stats
        .by_day
        .iter()
        .map(|(x, n)| {
            let parts: Vec<&str> = x.split('-').collect();
            let date = format!("{}/{}", parts[1], parts[2]);
            (date, *n as f32)
        })
        .collect();
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (70, 10, 50, 60);

    let x = ScaleBand::new()
        .set_domain(data.iter().map(|d| d.0.clone()).collect())
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1);

    let y = ScaleLinear::new()
        .set_domain(vec![
            0_f32,
            data.iter().map(|(_, n)| n.ceil() as isize).max().unwrap() as f32,
        ])
        .set_range(vec![height - top - bottom, 0]);

    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_rounding_precision(0)
        .load_data(&data)
        .unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("smalltalkzoo page views by day"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("views")
        .save(&path)
        .unwrap();

    println!("Rendered by-day chart into {:#?}", path);
}
