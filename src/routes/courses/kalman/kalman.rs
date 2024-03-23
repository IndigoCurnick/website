use rocket::Route;
use rocket_dyn_templates::Template;

#[get("/courses/kalman-filters/notation")]
pub async fn notation() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/kalman-filter-notation",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/introduction")]
pub async fn intro() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/01-introduction",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/falling")]
pub async fn falling() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/02-falling", context.into_json())
}

#[get("/courses/kalman-filters/constant-velocity")]
pub async fn constant_velocity() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/03-constant_velocity",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/falling-control")]
pub async fn falling_control() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/04-falling-control",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/cannon-ball")]
pub async fn cannon_ball() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/05-cannon-ball", context.into_json())
}

#[get("/courses/kalman-filters/sin-wave")]
pub async fn sin_wave() -> Template {
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/06-sin-wave", context.into_json())
}

pub fn get_kalman_courses() -> Vec<Route> {
    return routes![
        notation,
        intro,
        falling,
        constant_velocity,
        falling_control,
        cannon_ball,
        sin_wave
    ];
}
