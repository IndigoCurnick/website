use rocket::Route;
use rocket_dyn_templates::Template;

use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

#[get("/portals/science/basic_physics/kinematics")]
pub async fn kinematics() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let average_speed = average_speed_graph();
    let projectile = projectile_graph();
    context.insert("average_speed", &average_speed);
    context.insert("projectile", &projectile);
    Template::render(
        "portals/science/basic_physics/kinematics",
        context.into_json(),
    )
}

fn average_speed_graph() -> String {
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

    let output = plot.to_inline_html("average_speed");

    return output;
}

fn projectile_graph() -> String {
    let ux = 10_f64;
    let uy = 10_f64;
    let g = 9.81_f64;

    let t = (2_f64 * uy) / g;

    let t_list: Vec<f64> = linspace::<f64>(0_f64, t, 1000).collect();

    let sy_list: Vec<f64> = t_list.iter().map(|x| displacement(uy, -g, x)).collect();
    let sx_list: Vec<f64> = t_list.iter().map(|x| displacement(ux, 0_f64, x)).collect();

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("x")))
        .y_axis(Axis::new().matches(false).title(Title::new("y")));

    let trace = Scatter::new(sx_list, sy_list)
        .name("Projectile Motion")
        .mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(trace);

    let output = plot.to_inline_html("projectile_graph");

    return output;
}

fn displacement(u: f64, a: f64, t: &f64) -> f64 {
    return u * t + 0.5_f64 * a * t.powf(2_f64);
}

pub fn get_physics_routes() -> Vec<Route> {
    return routes![kinematics];
}
