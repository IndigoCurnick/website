use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};
use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/portals/science/basic_maths/algebra")]
async fn algebra() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let y3x2 = y3x2();
    let ym4xm6 = ym4xm6();
    context.insert("y3x2", &y3x2);
    context.insert("ym4xm6", &ym4xm6);
    context.insert("basic_simul", &basic_simul());
    Template::render("portals/science/basic_maths/algebra", context.into_json())
}

fn y3x2() -> String {
    let x0 = -5_f64;
    let x1 = 5_f64;
    let num_points = 1000;

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("x")))
        .y_axis(Axis::new().matches(false).title(Title::new("y")));

    let time: Vec<f64> = linspace::<f64>(x0, x1, num_points).collect();
    let amplitude: Vec<f64> = time.iter().map(|x| 3_f64 * x + 2_f64).collect();
    let trace = Scatter::new(time, amplitude).mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(trace);

    let output = plot.to_inline_html("y3x2");

    return output;
}

fn ym4xm6() -> String {
    let x0 = -10_f64;
    let x1 = 10_f64;
    let num_points = 1000;

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("x")))
        .y_axis(Axis::new().matches(false).title(Title::new("y")));

    let time: Vec<f64> = linspace::<f64>(x0, x1, num_points).collect();
    let amplitude: Vec<f64> = time.iter().map(|x| -4_f64 * x + -6_f64).collect();
    let trace = Scatter::new(time, amplitude).mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(trace);

    let output = plot.to_inline_html("ym4xm6");

    return output;
}

fn basic_simul() -> String {
    let x0 = -10_f64;
    let x1 = 10_f64;
    let num_points = 1000;

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("x")))
        .y_axis(Axis::new().matches(false).title(Title::new("y")));

    let time: Vec<f64> = linspace::<f64>(x0, x1, num_points).collect();
    let amplitude: Vec<f64> = time.iter().map(|x| 2_f64 * x + 2_f64).collect();
    let trace = Scatter::new(time.clone(), amplitude)
        .mode(Mode::Lines)
        .name("y = 2x + 2");

    let amplitude: Vec<f64> = time.iter().map(|x| -2_f64 * x + 3_f64).collect();
    let trace2 = Scatter::new(time, amplitude)
        .mode(Mode::Lines)
        .name("y = -2x + 3");

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(trace);
    plot.add_trace(trace2);

    let output = plot.to_inline_html("basic_simul");

    return output;
}

pub fn get_basic_mathematics_routes() -> Vec<Route> {
    routes![algebra]
}
