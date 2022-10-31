use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

#[get("/blog/programming/handling-errors-in-rust")]
async fn handling_errors() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/handling-errors-in-rust".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/handling_errors", context.into_json())
}

#[get("/blog/programming/how-to-refactor")]
async fn refactor() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/how-to-refactor".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/how_to_refactor", context.into_json())
}

#[get("/blog/programming/joy-of-programming")]
async fn joy() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/joy-of-programming".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/joy_of_programming", context.into_json())
}

#[get("/blog/programming/lifetime-ownership-rust")]
async fn lifetimes() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/lifetime-ownership-rust".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/lifetime_ownership", context.into_json())
}

#[get("/blog/programming/i-hate-oo")]
async fn i_hate_oo() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/i-hate-oo".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/i_hate_oo", context.into_json())
}

#[get("/blog/programming/programming-blog")]
async fn programming_blog() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/programming/programming-blog".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/programming/programming_blog", context.into_json())
}

pub fn get_programming_routes() -> Vec<Route> {
    routes![
        handling_errors,
        refactor,
        joy,
        lifetimes,
        i_hate_oo,
        programming_blog
    ]
}
