use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/blog/economics/after-the-welfare-state")]
async fn after_the_welfare_state() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/economics/after-the-welfare-state".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/economics/after_the_welfare_state",
        context.into_json(),
    )
}

#[get("/blog/economics/british-welfare-state")]
async fn british_welfare_state() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/economics/british-welfare-state".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/economics/british_welfare_state",
        context.into_json(),
    )
}

#[get("/blog/economics/bourgeois-virtues")]
async fn bourgeois_virtues() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/economics/bourgeois-virtues".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/economics/bourgeois_virtues",
        context.into_json(),
    )
}

pub fn get_econ_routes() -> Vec<Route> {
    routes![
        after_the_welfare_state,
        british_welfare_state,
        bourgeois_virtues
    ]
}
