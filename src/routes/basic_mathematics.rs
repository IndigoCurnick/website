use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::utils::plotting::{add_trace, create_layout, LayoutOptions};

#[get("/portals/science/basic-maths/numbers")]
async fn numbers() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("portals/science/basic_maths/1numbers", context.into_json())
}

#[get("/portals/science/basic-maths/linear-equations")]
async fn linear_equations() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("y3x2", &y3x2());
    context.insert("ym4xm6", &ym4xm6());
    context.insert("basic_simul", &basic_simul());
    Template::render(
        "portals/science/basic_maths/2linear_equations",
        context.into_json(),
    )
}

#[get("/portals/science/basic-maths/quadratic-equations")]
async fn quadratic_equations() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("basic_quadratic", &basic_quadratic());
    context.insert("negative_quadratic", &negative_quadratic());
    context.insert("showing_linear_term", &showing_linear_term());
    context.insert("showing_constant_term", &showing_constant_term());
    context.insert("first_solution", &first_solution());
    context.insert("quadratic_formula", &quadratic_formula());
    context.insert("no_roots", &no_roots());
    Template::render(
        "portals/science/basic_maths/3quadratic_equations",
        context.into_json(),
    )
}

fn basic_quadratic() -> String {
    fn func(x: &f64) -> f64 {
        return x.powf(2_f64);
    }
    let x0 = -5_f64;
    let x1 = 5_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("x^2".to_string()),
    );

    let output = plot.to_inline_html("xsq");

    return output;
}

fn no_roots() -> String {
    fn func(x: &f64) -> f64 {
        return -x.powf(2_f64) + x - 1.0;
    }
    let x0 = -2_f64;
    let x1 = 2_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("-x^2 + x - 1".to_string()),
    );

    let output = plot.to_inline_html("no_roots");

    return output;
}

fn negative_quadratic() -> String {
    fn func(x: &f64) -> f64 {
        return -x.powf(2_f64);
    }
    let x0 = -5_f64;
    let x1 = 5_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("-x^2".to_string()),
    );

    let output = plot.to_inline_html("negative_quadratic");

    return output;
}

fn showing_linear_term() -> String {
    fn func1(x: &f64) -> f64 {
        return x.powf(2_f64) + 2_f64 * x;
    }
    fn func2(x: &f64) -> f64 {
        return -x.powf(2_f64) - 4_f64 * x;
    }
    let x0 = -3_f64;
    let x1 = 3_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func1,
        Some("x^2 + 2x".to_string()),
    );
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func2,
        Some("-x^2 - 4x".to_string()),
    );

    let output = plot.to_inline_html("showing_linear_term");

    return output;
}

fn showing_constant_term() -> String {
    fn func1(x: &f64) -> f64 {
        return x.powf(2_f64) + 2_f64 * x - 8.0;
    }
    fn func2(x: &f64) -> f64 {
        return -x.powf(2_f64) - 3_f64 * x - 2_f64;
    }
    let x0 = -5_f64;
    let x1 = 5_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func1,
        Some("x^2 + 2x - 8".to_string()),
    );
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func2,
        Some("-x^2 - 3x - 2".to_string()),
    );

    let output = plot.to_inline_html("showing_constant_term");

    return output;
}

fn first_solution() -> String {
    fn func(x: &f64) -> f64 {
        return x.powf(2_f64) + 2_f64 * x - 8.0;
    }
    let x0 = -10_f64;
    let x1 = 10_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("x^2 + 2x - 8".to_string()),
    );

    let output = plot.to_inline_html("first_solution");

    return output;
}

fn quadratic_formula() -> String {
    fn func(x: &f64) -> f64 {
        return -x.powf(2_f64) + 5.0 * x - 1.0;
    }
    let x0 = -2_f64;
    let x1 = 8_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("-x^2 + 5x - 1".to_string()),
    );

    let output = plot.to_inline_html("quadratic_formula");

    return output;
}

fn y3x2() -> String {
    fn func(x: &f64) -> f64 {
        return 3_f64 * x + 2_f64;
    }
    let x0 = -5_f64;
    let x1 = 5_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("y = 3x + 2".to_string()),
    );

    let output = plot.to_inline_html("y3x2");

    return output;
}

fn ym4xm6() -> String {
    fn func(x: &f64) -> f64 {
        return -4_f64 * x - 6_f64;
    }
    let x0 = -10_f64;
    let x1 = 10_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("y = -4x - 6".to_string()),
    );

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
    routes![numbers, linear_equations, quadratic_equations]
}
