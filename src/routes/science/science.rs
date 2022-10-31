use rocket::Route;
use rocket_dyn_templates::Template;

use crate::{database::insert_to_database, DOMAIN};

use super::{
    basic_mathematics::get_basic_mathematics_routes, basic_physics::get_basic_physics_routes,
};

#[get("/blog/science/introduction-to-truth-tables")]
async fn truth_tables() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/science/introduction-to-truth-tables".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/science/introduction_to_truth_tables",
        context.into_json(),
    )
}

#[get("/blog/science/introduction-to-variables")]
async fn variables() -> Template {
    tokio::spawn(async move {
        insert_to_database(
            DOMAIN.to_string(),
            "/blog/science/introduction-to-variables".to_string(),
        )
        .await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render(
        "blog/science/introduction_to_variables",
        context.into_json(),
    )
}

#[get("/blog/science/science-blog")]
async fn science_blog() -> Template {
    tokio::spawn(async move {
        insert_to_database(DOMAIN.to_string(), "/blog/science/science-blog".to_string()).await;
    });
    let context = rocket_dyn_templates::tera::Context::new();
    Template::render("blog/science/science_blog", context.into_json())
}

pub fn get_science_routes() -> Vec<Route> {
    let basic_maths = get_basic_mathematics_routes();
    let basic_physics = get_basic_physics_routes();
    let science_routes = routes![truth_tables, variables, science_blog];

    return vec![basic_maths, basic_physics, science_routes].concat();
}
