use std::f64::consts::PI;

use rocket::Route;
use rocket_dyn_templates::Template;

use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

use crate::{database::insert_to_database, DOMAIN};

#[get("/portals/science/basic-physics/kinematics")]
pub async fn kinematics() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/kinematics".to_string(),
        )
        .await;
    });
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

#[get("/portals/science/basic-physics/dynamics")]
pub async fn dynamics() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/dynamics".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/science/basic_physics/dynamics",
        context.into_json(),
    )
}

#[get("/portals/science/basic-physics/fields")]
pub async fn fields() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/fields".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/science/basic_physics/fields", context.into_json())
}

#[get("/portals/science/basic-physics/rotation")]
pub async fn rotation() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/rotation".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/science/basic_physics/rotation",
        context.into_json(),
    )
}

#[get("/portals/science/basic-physics/oscillations")]
pub async fn oscillations() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/oscillations".to_string(),
        )
        .await;
    });
    let wave_graph = basic_wave();
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("basic_wave", &wave_graph);
    Template::render(
        "portals/science/basic_physics/oscillations",
        context.into_json(),
    )
}

#[get("/portals/science/basic-physics/circuits")]
pub async fn circuits() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/portals/science/basic-physics/circuits".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "portals/science/basic_physics/circuits",
        context.into_json(),
    )
}

fn basic_wave() -> String {
    let x0 = 0.0_f64;
    let x1 = 2.0_f64;
    let num_points = 1000;

    let this_layout = Layout::default()
        .x_axis(Axis::new().matches(false).title(Title::new("Time")))
        .y_axis(Axis::new().matches(false).title(Title::new("Amplitude")));

    let time: Vec<f64> = linspace::<f64>(x0, x1, num_points).collect();
    let amplitude: Vec<f64> = time.iter().map(|x| (4.0 * PI * x).sin()).collect();
    let trace = Scatter::new(time, amplitude)
        .name("Linear Acceleration")
        .mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.set_layout(this_layout);
    plot.add_trace(trace);

    let output = plot.to_inline_html("average_speed");

    return output;
}

pub fn get_basic_physics_routes() -> Vec<Route> {
    return routes![
        kinematics,
        dynamics,
        fields,
        rotation,
        oscillations,
        circuits
    ];
}
