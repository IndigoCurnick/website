use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};
use rocket_dyn_templates::Template;

#[get("/physics/kinematics")]
pub async fn kinematics() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let map = average_speed_graph();
    context.insert("map", &map);
    Template::render("physics/kinematics", context.into_json())
}

pub fn average_speed_graph() -> String {
    let u = 0.0_f64;
    let v = 10.0_f64;
    let num_points = 100;

    let t0 = 0.0_f64;
    let t = 10.0_f64;

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("Time")))
        .y_axis(Axis::new().matches(false).title(Title::new("Speed")));

    let constant_increase = linspace::<f64>(u, v, num_points);
    let average = vec![(u + v) / 2.0_f64; num_points];
    let time = linspace::<f64>(t0, t, num_points);
    let accel_trace = Scatter::new(time.clone(), constant_increase)
        .name("Linear Acceleration")
        .mode(Mode::Lines);

    let constant_trace = Scatter::new(time.clone(), average)
        .name("Constnat Velocity")
        .mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(accel_trace);
    plot.add_trace(constant_trace);

    let output = plot.render(false, "", 0, 0);

    return output;
}
