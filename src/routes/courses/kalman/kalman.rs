use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/courses/kalman-filters/notation")]
pub async fn notation() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/notation".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/kalman-filter-notation",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/introduction")]
pub async fn intro() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/introduction".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/01-introduction",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/falling")]
pub async fn falling() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/falling".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/02-falling", context.into_json())
}

#[get("/courses/kalman-filters/constant-velocity")]
pub async fn constant_velocity() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/constant-velocity".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/03-constant_velocity",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/falling-control")]
pub async fn falling_control() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/falling-control".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "courses/kalman-filters/04-falling-control",
        context.into_json(),
    )
}

#[get("/courses/kalman-filters/cannon-ball")]
pub async fn cannon_ball() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/cannon-ball".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/05-cannon-ball", context.into_json())
}

#[get("/courses/kalman-filters/sin-wave")]
pub async fn sin_wave() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/courses/kalman-filters/sin-wave".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("courses/kalman-filters/04-sin-wave", context.into_json())
}

pub fn get_kalman_courses() -> Vec<Route> {
    return routes![
        notation,
        intro,
        falling,
        constant_velocity,
        falling_control,
        cannon_ball
    ];
}
