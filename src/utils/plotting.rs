use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

pub struct LayoutOptions {
    custom_x: Option<String>,
    custom_y: Option<String>,
    matches: bool,
}

impl LayoutOptions {
    pub fn default() -> Self {
        return Self {
            custom_x: None,
            custom_y: None,
            matches: false,
        };
    }
}

pub fn create_layout(options: LayoutOptions) -> Plot {
    let matches = options.matches;

    let this_layout = Layout::default()
        .x_axis(
            Axis::new()
                .matches(matches)
                .title(Title::new(&options.custom_x.unwrap_or("x".to_string()))),
        )
        .y_axis(
            Axis::new()
                .matches(matches)
                .title(Title::new(&options.custom_y.unwrap_or("y".to_string()))),
        );

    let mut plot = Plot::new();
    plot.set_layout(this_layout);
    return plot;
}

pub fn add_trace(
    plot: &mut Plot,
    x_min: f64,
    x_max: f64,
    num_points: usize,
    func: &dyn Fn(&f64) -> f64,
    name: Option<String>,
) {
    let x: Vec<f64> = linspace(x_min, x_max, num_points).collect();
    let y: Vec<f64> = x.iter().map(|c| func(c)).collect();
    let trace = if name.is_some() {
        Scatter::new(x, y).mode(Mode::Lines).name(&name.unwrap())
    } else {
        Scatter::new(x, y).mode(Mode::Lines)
    };

    plot.add_trace(trace);
}
