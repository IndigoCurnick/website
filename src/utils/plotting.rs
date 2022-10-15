use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

pub struct LayoutOptions {
    pub custom_x: Option<String>,
    pub custom_y: Option<String>,
    pub matches: bool,
    pub y_range: Option<Vec<f64>>,
    pub x_range: Option<Vec<f64>>,
    pub title: Option<String>,
}

impl LayoutOptions {
    pub fn default() -> Self {
        return Self {
            custom_x: None,
            custom_y: None,
            matches: false,
            y_range: None,
            x_range: None,
            title: None,
        };
    }
}

pub fn create_layout(options: LayoutOptions) -> Plot {
    fn make_axis(
        matches: bool,
        title: Option<String>,
        range: Option<Vec<f64>>,
        axis: String,
    ) -> Axis {
        if range.is_some() {
            let unwrapped_range = range.unwrap();
            assert!(
                unwrapped_range.len() == 2,
                "Range must have only two elements"
            );
            return Axis::new()
                .matches(matches)
                .title(Title::new(&title.unwrap_or(axis)))
                .range(unwrapped_range);
        } else {
            return Axis::new()
                .matches(matches)
                .title(Title::new(&title.unwrap_or(axis)));
        }
    }
    let matches = options.matches;

    let this_layout = Layout::new()
        .x_axis(make_axis(
            matches,
            options.custom_x,
            options.x_range,
            "x".to_string(),
        ))
        .y_axis(make_axis(
            matches,
            options.custom_y,
            options.y_range,
            "y".to_string(),
        ))
        .title(Title::new(&options.title.unwrap_or("".to_string())));

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
