use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/blog/politics/a-defence-of-machiavelli")]
async fn a_defence_of_mach() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/politics/a-defence-of-machiavelli".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/politics/a_defence_of_mach",
        context.into_json(),
    )
}

#[get("/blog/politics/war-that-will-end-war")]
async fn the_war_that_will_end_war() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/politics/war-that-will-end-war".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/politics/the_war_that_will_end_war",
        context.into_json(),
    )
}

#[get("/blog/politics/beyond-left-right")]
async fn beyond_left_right() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/politics/beyond-left-right".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/humanities/politics/beyond_left_right",
        context.into_json(),
    )
}

pub fn get_politics_routes() -> Vec<Route> {
    routes![
        a_defence_of_mach,
        the_war_that_will_end_war,
        beyond_left_right
    ]
}