use itertools_num::linspace;
use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};
use rocket::Route;
use rocket_dyn_templates::Template;

use crate::utils::plotting::{add_trace, create_layout, LayoutOptions};

#[get("/courses/mathematics/basic-maths/numbers")]
async fn numbers() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/mathematics/basic_maths/1numbers",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/linear-equations")]
async fn linear_equations() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("y3x2", &y3x2());
    context.insert("ym4xm6", &ym4xm6());
    context.insert("basic_simul", &basic_simul());
    Template::render(
        "courses/mathematics/basic_maths/2linear_equations",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/quadratic-equations")]
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
        "courses/mathematics/basic_maths/3quadratic_equations",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/distances-and-angles")]
async fn distance_angles() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("sin", &sing());
    context.insert("cos", &cosg());
    context.insert("tan", &tang());
    context.insert("asin", &asing());
    context.insert("acos", &acosg());
    context.insert("atan", &atang());
    Template::render(
        "courses/mathematics/basic_maths/4distance_angles",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/angles-parallel")]
async fn angles_parallel() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();

    Template::render(
        "courses/mathematics/basic_maths/5angles_on_lines",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/area")]
async fn area() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/mathematics/basic_maths/6area", context.into_json())
}

#[get("/courses/mathematics/basic-maths/coordinates")]
async fn coordinates() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("circle_graph", &cirlce_graph());
    context.insert("x_cubed", &x_cube_coord());
    context.insert("multiply_points", &point_graph());
    Template::render(
        "courses/mathematics/basic_maths/7coordinates",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/trigonometry")]
async fn trigonometry() -> Template {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("sin", &sing());
    context.insert("cos", &cosg());
    context.insert("asin", &asing());
    context.insert("acos", &acosg());
    context.insert("atan", &atang());
    Template::render(
        "courses/mathematics/basic_maths/8trigonometry",
        context.into_json(),
    )
}

#[get("/courses/mathematics/basic-maths/complex-numbers")]
async fn complex_numbers() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/mathematics/basic_maths/9complex_numbers",
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

fn sing() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().sin();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: None,
        title: Some("sin(x)".to_string()),
    });
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("sin(x)".to_string()),
    );

    return plot.to_inline_html("sin");
}

fn cosg() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().cos();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: None,
        title: Some("cos(x)".to_string()),
    });
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("cos(x)".to_string()),
    );

    return plot.to_inline_html("cos");
}

fn tang() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().tan();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 10000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: Some(vec![-10.0, 10.0]),
        title: Some("tan(x)".to_string()),
    });

    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("tan(x)".to_string()),
    );

    return plot.to_inline_html("tan");
}

fn asing() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().asin();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: None,
        title: Some("asin(x)".to_string()),
    });
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("asin(x)".to_string()),
    );

    return plot.to_inline_html("asin");
}

fn acosg() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().acos();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 1000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: None,
        title: Some("acos(x)".to_string()),
    });
    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("acos(x)".to_string()),
    );

    return plot.to_inline_html("acos");
}

fn atang() -> String {
    fn func(x: &f64) -> f64 {
        return x.to_radians().atan();
    }
    let x0 = -360_f64;
    let x1 = 360_f64;
    let num_points = 10000;

    let mut plot = create_layout(LayoutOptions {
        custom_x: Some("Degrees".to_string()),
        custom_y: None,
        matches: false,
        x_range: None,
        y_range: Some(vec![-10.0, 10.0]),
        title: Some("atan(x)".to_string()),
    });

    add_trace(
        &mut plot,
        x0,
        x1,
        num_points,
        &func,
        Some("atan(x)".to_string()),
    );

    return plot.to_inline_html("atan");
}

fn x_cube_coord() -> String {
    fn func(x: &f64) -> f64 {
        return x.powf(3_f64) + 2.0 * x.powf(2_f64) - x + 4.0;
    }

    let mut plot = create_layout(LayoutOptions::default());
    add_trace(&mut plot, -10.0, 10.0, 1000, &func, None);

    return plot.render_inline("x_cubed");
}

fn point_graph() -> String {
    let x = 1.0;
    let y = 1.0;

    let mut plot = create_layout(LayoutOptions::default());

    let trace0 = Scatter::new(vec![x], vec![y])
        .mode(Mode::Markers)
        .name("Original");
    let trace1 = Scatter::new(vec![2.0 * x], vec![2.0 * y])
        .mode(Mode::Markers)
        .name("2");
    let trace2 = Scatter::new(vec![-1.0 * x], vec![-1.0 * y])
        .mode(Mode::Markers)
        .name("-1");
    let trace3 = Scatter::new(vec![0.5 * x], vec![0.5 * y])
        .mode(Mode::Markers)
        .name("0.5");
    let trace4 = Scatter::new(vec![-0.5 * x], vec![-0.5 * y])
        .mode(Mode::Markers)
        .name("-0.5");
    let trace5 = Scatter::new(vec![-2.0 * x], vec![-2.0 * y])
        .mode(Mode::Markers)
        .name("-2");

    plot.add_traces(vec![trace0, trace1, trace2, trace3, trace4, trace5]);
    return plot.render_inline("multiply_points");
}

fn cirlce_graph() -> String {
    let points = get_circle_points(&circle_eqaution, (-2.0, 2.0), (-2.0, 2.0), 50);
    let mut layout = LayoutOptions::default();
    layout.matches = true;
    let mut plot = create_layout(layout);

    let outside = Scatter::new(points.outside.x, points.outside.y)
        .mode(Mode::Markers)
        .name("Outside");
    let perimeter = Scatter::new(points.perimeter.x, points.perimeter.y)
        .mode(Mode::Markers)
        .name("Perimeter");
    let inside = Scatter::new(points.inside.x, points.inside.y)
        .mode(Mode::Markers)
        .name("Inside");

    plot.add_traces(vec![outside, perimeter, inside]);

    return plot.to_inline_html("circle-plot");
}

struct Coordinates {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl Coordinates {
    fn default() -> Coordinates {
        return Coordinates {
            x: vec![],
            y: vec![],
        };
    }

    fn new_point(&mut self, x: &f64, y: &f64) {
        self.x.push(x.clone());
        self.y.push(y.clone());
    }
}

struct CirclePoints {
    pub outside: Coordinates,
    pub inside: Coordinates,
    pub perimeter: Coordinates,
}

impl CirclePoints {
    fn default() -> CirclePoints {
        return CirclePoints {
            outside: Coordinates::default(),
            inside: Coordinates::default(),
            perimeter: Coordinates::default(),
        };
    }
}

enum CircleCondition {
    Inside,
    Outside,
    Perimeter,
}

fn circle_eqaution(x: &f64, y: &f64) -> CircleCondition {
    fn equation(x: &f64, y: &f64) -> f64 {
        return (x.powf(2_f64) + y.powf(2_f64)).sqrt();
    }
    let radius = 1.0;
    let result = equation(x, y);
    if result > radius {
        return CircleCondition::Outside;
    } else if result == radius {
        return CircleCondition::Perimeter;
    } else {
        return CircleCondition::Inside;
    }
}

fn get_circle_points(
    circle_eqaution: &dyn Fn(&f64, &f64) -> CircleCondition,
    x_lim: (f64, f64),
    y_lim: (f64, f64),
    resolution: usize,
) -> CirclePoints {
    let mut coords = CirclePoints::default();
    let x_points: Vec<f64> = linspace(x_lim.0, x_lim.1, resolution).collect();
    let y_points: Vec<f64> = linspace(y_lim.0, y_lim.1, resolution).collect();

    for x in &x_points {
        for y in &y_points {
            match circle_eqaution(x, y) {
                CircleCondition::Inside => coords.inside.new_point(x, y),
                CircleCondition::Outside => coords.outside.new_point(x, y),
                CircleCondition::Perimeter => coords.perimeter.new_point(x, y),
            };
        }
    }

    return coords;
}

pub fn get_basic_mathematics_routes() -> Vec<Route> {
    routes![
        numbers,
        linear_equations,
        quadratic_equations,
        distance_angles,
        angles_parallel,
        area,
        coordinates,
        trigonometry,
        complex_numbers,
    ]
}
